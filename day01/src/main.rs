#![allow(dead_code)]

const REPLACEMENTS: [(&str, &str); 9] = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"), ];
const REPLACEMENTS_REVERSED: [(&str, &str); 9] = [("eno", "1"), ("owt", "2"), ("eerht", "3"), ("ruof", "4"), ("evif", "5"), ("xis", "6"), ("neves", "7"), ("thgie", "8"), ("enin", "9"), ];

fn main() {
    part_one();
    part_two();
    // tests();
}

fn part_one() {
    let lines = include_str!("../input.txt")
        .split("\n")
        .collect();

    let sum = sum_lines(lines);

    println!("Part 1: {:?}", sum);
}

fn part_two() {
    let raw = include_str!("../input.txt");

    let calibration_values = get_calibration_values(raw);

    let sum: u32 = calibration_values.iter().sum();
    println!("Part 2: {:?}", sum);
}

fn get_calibration_values(raw_text: &str) -> Vec<u32> {
     raw_text
        .lines()
        .map(|line| {
            let better_line: String = line.to_string(); // idk man

            // Get first number starting from the left
            let first_number = get_first_number(&better_line, REPLACEMENTS);

            // Get first number starting from the right
            let reversed = better_line.chars().rev().collect();
            let last_number = get_first_number(&reversed, REPLACEMENTS_REVERSED);

            first_number * 10 + last_number
        }).collect()
}

fn get_first_number(line: &String, replacements: [(&str, &str); 9]) -> u32 {
    let first_digit_index: Option<usize> = line.find(|c| char::is_digit(c, 10));
    let first_word_index: Option<(String, u32, usize)> = get_first_word(line, replacements);

    // Case: There is neither a digit or a word in this line
    if first_digit_index.is_none() && first_word_index.is_none() { return 0; }

    // Case: There are only digits
    if first_word_index.is_none() {

        return line
            .chars()
            .nth(first_digit_index.unwrap())
            .map(|c| c.to_digit(10).unwrap())
            .unwrap();
    }

    // Case: There are only text words
    if first_digit_index.is_none() {
        return first_word_index.unwrap().1;
    }

    // this is a hack because I got pissed off at borrow checking
    let first_digit_index_unwrapped = first_digit_index.unwrap();
    let first_word_index_unwrapped = first_word_index.unwrap();

    // Case: There are both digits and words so we need to return whichever is first
    return if first_digit_index_unwrapped < first_word_index_unwrapped.2 {
        // digit comes before word
        line
            .chars()
            .nth(first_digit_index_unwrapped)
            .map(|c| c.to_digit(10).unwrap())
            .unwrap()
    } else {
        // word comes before digit
        first_word_index_unwrapped.1
    }
}

fn get_first_word(line: &String, replacements: [(&str, &str); 9]) -> Option<(String, u32, usize)> {
    let mut indexes: Vec<(&str, &str, Option<usize>)> = Vec::new();

    // get the location of all text words in the string
    for replacement in replacements {
        indexes.push((replacement.0, replacement.1, line.find(replacement.0)))
    }

    // filter a vector of options to only ones with values
    let mut index_hits: Vec<(&str, &str, usize)> = indexes
        .into_iter()
        .filter_map(|(name, value, location)| location.map(|l| (name, value, l)))
        .collect();

    if index_hits.is_empty() {
        return None;
    }

    // Find the earliest occurrence of a text word
    index_hits.sort_by(|a, b| a.2.cmp(&b.2));

    // Now we know what the first word to replace is.  Replace it and return it
    let found_word = index_hits[0];
    return Some((found_word.0.to_string(), found_word.1.parse().unwrap(), found_word.2));
}

fn sum_lines(lines: Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter(|&c| c.is_digit(10))
                .map(|d| d.to_digit(10).unwrap())
                .collect();

            return digits[0] * 10 + digits[digits.len() - 1];
        })
        .sum()
}

fn tests() {
    run_test(&String::from("1twothree9"), 19);
    run_test(&String::from("two19three"), 23);
    run_test(&String::from("1two9three"), 13);
    run_test(&String::from("two1three9"), 29);
    run_test(&String::from("eightwo"), 82);
    run_test(&String::from("eight"), 88);
    run_test(&String::from("1"), 11);
    run_test(&String::from("12837462387462348"), 18);
    run_test(&String::from("eightsixsevenfivethreenine"), 89);
    run_test(&String::from("iuhwefuywefuywgef"), 0);
    run_test(&String::from(""), 0);
}

fn run_test(line: &String, expected_value: u32) {
    let first_number = get_first_number(&line, REPLACEMENTS);
    let last_number = get_first_number(&line.chars().rev().collect(), REPLACEMENTS_REVERSED);
    let sum = first_number * 10 + last_number;

    if sum != expected_value {
        println!("FAIL - {} - Expected: {}, Actual: {}", line, expected_value, sum);
    } else {
        println!("PASS - {}", line);
    }
}