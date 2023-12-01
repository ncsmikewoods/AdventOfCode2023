fn main() {
    let lines = include_str!("../input.txt")
        .split("\n");

    let sum: u32 = lines.map(|line| {
        let digits: Vec<u32> = line
            .chars()
            .filter(|&c| c.is_digit(10))
            .map(|d| d.to_digit(10).unwrap())
            .collect();

        return digits[0] * 10 + digits[digits.len() - 1];
    })
        .sum();

    println!("{:?}", sum);
}
