#![allow(dead_code)]

const REPLACEMENTS: [(&str, &str); 9] = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"), ];
const REPLACEMENTS_REVERSED: [(&str, &str); 9] = [("eno", "1"), ("owt", "2"), ("eerht", "3"), ("ruof", "4"), ("evif", "5"), ("xis", "6"), ("neves", "7"), ("thgie", "8"), ("enin", "9"), ];

fn main() {
    // part_one();
    part_two();
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
    println!("Before replacements: {:?}", raw);

    let calibration_values = get_calibration_values(raw);

    let sum: u32 = calibration_values.iter().sum();
    println!("Part 2: {:?}", sum);
}

// TODO : this can probably be significantly cleaned up and turned into a pure function
fn get_calibration_values(raw_text: &str) -> Vec<u32> {
    let mut lines: Vec<String> = raw_text
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut calibration_values: Vec<u32> = Vec::new();
    for (i, _line) in lines.clone().into_iter().enumerate() {
        // println!();
        // println!("Line: {}", lines[i]);

        let first_number = get_first_number(&lines[i], REPLACEMENTS);
        let last_number = get_first_number(&lines[i].chars().rev().collect(), REPLACEMENTS_REVERSED);

        // println!("first_number: {}", first_number);
        // println!("last_number: {}", last_number);

        let sum = first_number * 10 + last_number;
        // println!("sum: {}", sum);
        calibration_values.push(sum);
    }
    return calibration_values;
}

fn get_first_number(line: &String, replacements: [(&str, &str); 9]) -> u32 {
    let first_digit_index: Option<usize> = line.find(|c| char::is_digit(c, 10));
    let first_word_index: Option<(String, u32, usize)> = get_first_word(line, replacements);

    // Case: There is neither a digit or a word in this line
    if first_digit_index.is_none() && first_word_index.is_none() { return 0; }

    // Case: There's only digits
    if first_word_index.is_none() {

        return line
            .chars()
            .nth(first_digit_index.unwrap())
            .map(|c| c as u32)
            .unwrap();
    }

    // Case: There's only text words
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