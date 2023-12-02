#![allow(dead_code)]

use std::str::Split;

const REPLACEMENTS: [(&str, &str); 9] = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"), ];
const REPLACEMENTS_REVERSED: [(&str, &str); 9] = [("eno", "1"), ("owt", "2"), ("eerht", "3"), ("ruof", "4"), ("evif", "5"), ("xis", "6"), ("neves", "7"), ("thgie", "8"), ("enin", "9"), ];

fn main() {
    // part_one();
    part_two();
}


fn part_one() {
    let lines = include_str!("../input.txt")
        .split("\n")
        .collect(); // TODO : convert to .lines()

    let sum = sum_lines(lines);

    println!("Part 1: {:?}", sum);
}

fn part_two() {
    let raw = include_str!("../input_short.txt");
    println!("Before replacements: {:?}", raw);

    let replaced_string = replace_words(raw);
    println!("After replacements: {:?}", replaced_string);

    let converted_strings = replaced_string.iter().map(|s| s.as_str()).collect(); // no idea why I have to do this
    let sum = sum_lines(converted_strings);

    println!("Part 2: {:?}", sum);
}

fn replace_words(raw_text: &str) -> Vec<String> {
    let mut lines: Vec<String> = raw_text
        .lines()
        .map(|line| line.to_string())
        .collect();

    for (i, line) in lines.clone().into_iter().enumerate() {
        println!();
        println!("Raw line before mutation: {}", line);

        // find and replace text digits starting at the front
        lines[i] = replace_first_word(&lines[i], REPLACEMENTS);

        println!("Raw line after forward mutation: {}", line);

        // find and replace text digits starting at the back
        lines[i] = replace_first_word(&lines[i].chars().rev().collect(), REPLACEMENTS_REVERSED);

        println!("Raw line after backward mutation: {}", line);
    }
    return lines;
}

fn replace_first_word(line: &String, replacements: [(&str, &str); 9]) -> String {
    let mut indexes: Vec<(&str, &str, Option<usize>)> = Vec::new();

    for replacement in replacements {
        indexes.push((replacement.0, replacement.1, line.find(replacement.0)))
    }

    // filter a vector of options to only ones with values
    let mut index_hits: Vec<(&str, &str, usize)> = indexes
        .into_iter()
        .filter_map(|(name, value, location)| location.map(|l| (name, value, l)))
        .collect();

    if index_hits.is_empty() {
        return line.clone();
    }

    // Find the earliest occurrence of a text word
    index_hits.sort_by(|a, b| a.2.cmp(&b.2));

    // Now we know what the first word to replace is.  Replace it and return it
    return line.replace(index_hits[0].0, index_hits[0].1);
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