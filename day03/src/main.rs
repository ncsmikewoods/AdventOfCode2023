fn main() {
    part_one();
}

fn part_one () {
    let grid: Vec<Vec<char>> = include_str!("../input_short.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let schematic = Schematic::from_grid(grid);
    schematic.display();
}

struct Label {
    x: usize,
    y: usize,
    length: usize,
    value: i32,
}

struct Schematic {
    grid: Vec<Vec<char>>,
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
                        println!("num: {}", num);

                        let label: Label = Label {
                            x: row_index,
                            y: col_start,
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
                println!("num: {}", num);
                let label: Label = Label {
                    x: row_index,
                    y: col_start,
                    length: num.len(),
                    value: num.parse::<i32>().unwrap()
                };
                labels.push(label);
            }
        }

        Schematic { grid, labels }
    }

    fn display(&self) {
        for label in &self.labels {
            println!("Label - x: {}, y: {}, length: {}, value: {}", label.x, label.y, label.length, label.value);
        }
    }
}