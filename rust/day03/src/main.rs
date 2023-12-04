#![allow(dead_code)]
use std::cmp;

fn main() {
    part_one();
}

fn part_one () {
    let grid: Vec<Vec<char>> = include_str!("../input_short.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let schematic = Schematic::from_grid(grid);
    // schematic.display();

    let part_numbers: Vec<i32> = schematic.get_part_numbers();

    for part_number in &part_numbers {
        println!("Part number: {}", part_number);
    }

    let part_num_sum: i32 = part_numbers.iter().sum();
    println!("Part num sum: {}", part_num_sum);
}

// TODO : change these all to i32?
struct Label {
    row: usize,
    col: usize,
    length: usize,
    value: i32,
}

struct Schematic {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    labels: Vec<Label>
}

impl Schematic {
    fn from_grid(grid: Vec<Vec<char>>) -> Schematic {
        let mut labels: Vec<Label> = Vec::new();

        // TODO : put this in a testable function
        for (row_index, row) in grid.iter().enumerate() {
            let mut col_start: usize = 99999;
            let mut num: String = String::new();

            for (col_index, _col_value) in row.iter().enumerate() {
                let curr: char = grid[row_index][col_index];

                // case: starting a new number or adding to an existing one
                if curr.is_digit(10) {
                    if col_start == 99999 {
                        // new number starting
                        col_start = col_index;
                        num.push(curr);
                    } else {
                        // adding a digit to an existing number
                        num.push(curr);
                    }
                } else {
                    if col_start != 99999 {
                        // case: this element ends a number
                        // println!("num: {}", num);

                        let label: Label = Label {
                            row: row_index,
                            col: col_start,
                            length: num.len(),
                            value: num.parse::<i32>().unwrap()
                        };
                        labels.push(label);
                        col_start = 99999;
                        num = String::new();
                    }
                }
            }

            // account for col_start not being reset which means a line ends with a number and never got terminated in the inner loop
            if col_start != 99999 {
                // println!("num: {}", num);
                let label: Label = Label {
                    row: row_index,
                    col: col_start,
                    length: num.len(),
                    value: num.parse::<i32>().unwrap()
                };
                labels.push(label);
            }
        }

        Schematic { grid: grid.clone(), width: grid[0].len(), height:grid.len(),  labels }
    }

    fn get_part_numbers(&self) -> Vec<i32> {
        let mut part_numbers: Vec<i32> = Vec::new();

        for label in &self.labels {
            let adjacents: Vec<char> = self.get_adjacent(label);
            if adjacents.iter().any(|a| !a.is_digit(10) && *a != '.') {
                part_numbers.push(label.value);
            }
        }

        return part_numbers;
    }

    fn get_adjacent(&self, label: &Label) ->  Vec<char> {
        let mut adjacents: Vec<char> = Vec::new();

        let left_index = (label.col as i32 - 1) as usize;
        let left_bound = cmp::max(0, left_index);

        let right_index = (((label.col + label.length) as i32) + 1) as usize;
        let right_bound = cmp::min(self.width, right_index);

        println!("left_bound {}, right_bound {}", left_bound, right_bound);

        // add adjacents from row above
        if label.row != 0 {
            let adjacent_above: Vec<char> =
                (&self.grid[label.row-1][left_bound..right_bound])
                    .to_vec();
            adjacents.extend(adjacent_above);
        }

        // add adjacents from same row
        {
            println!("On row {}. attempting to access range {} to {}", label.row, left_bound, right_bound);

            let adjacent_same_row: Vec<char> =
                (&self.grid[label.row][left_bound..right_bound])
                    .to_vec();
            adjacents.extend(adjacent_same_row);
        }

        // add adjacents from row below
        if label.row + 1 != self.grid.len() {
            let adjacent_below: Vec<char> =
                (&self.grid[label.row+1][left_bound..right_bound])
                    .to_vec();
            adjacents.extend(adjacent_below);
        }

        return adjacents;
    }

    fn display(&self) {
        for label in &self.labels {
            println!("Label - x: {}, y: {}, length: {}, value: {}", label.row, label.col, label.length, label.value);
        }
    }
}