fn main() {
    part_one();
    part_two();
    // run_tests();
}

fn part_one() {
    let lines = include_str!("../input.txt").lines();

    let games: Vec<Game> =
        lines
            .map(|line| Game::from_line(line))
            .collect();

    let valid_games_sum: u32 =
        games.iter()
            .filter(|game| game.is_valid(12, 14, 13))
            .map(|game| {
                // println!("Valid game: {}", game.id);
                game.id
            })
            .sum();

    println!("Valid game sum: {}", valid_games_sum);
}

fn part_two() {
    let lines = include_str!("../input.txt").lines();

    let games: Vec<Game> =
        lines
            .map(|line| Game::from_line(line))
            .collect();

    let game_power_sum: u32 =
        games.iter()
            .map(|game| game.get_power())
            .sum();

    println!("Game power sum: {}", game_power_sum);
}

struct Game {
    id: u32,
    reveals: Vec<Reveal>
}

impl Game {
    fn from_line(text: &str) -> Game {
        // separate game name from the reveals in the game
        let top_level_tokens: Vec<&str> = text.split(": ").collect();
        let game_id: u32 =
            top_level_tokens[0]
                .split(" ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

        // separate out all the different reveals in raw token form. Ex: "3 blue, 4 red" or "1 red, 2 green, 6 blue"
        let reveal_tokens: Vec<&str> =
            top_level_tokens[1]
                .split("; ")
                .collect();

        // create Reveals from the text
        let reveals: Vec<Reveal> =
            reveal_tokens
                .iter()
                .map(|reveal_token_raw| Reveal::from_input(reveal_token_raw))
                .collect();

        Game { id: game_id, reveals}
    }

    fn get_power(&self) -> u32 {
        let max_red =
            &self.reveals
                .iter()
                .max_by_key(|reveal| reveal.reds)
                .unwrap()
                .reds;

        let max_blue =
            &self.reveals
                .iter()
                .max_by_key(|reveal| reveal.blues)
                .unwrap()
                .blues;

        let max_green =
            &self.reveals
                .iter()
                .max_by_key(|reveal| reveal.greens)
                .unwrap()
                .greens;

        return max_red * max_blue * max_green;
    }

    fn display(&self) {
        println!();
        println!("Game - id: {}", self.id);
        for reveal in &self.reveals {
            reveal.display();
        }
    }

    fn is_valid(&self, reds: u32, blues: u32, greens: u32) -> bool {
        !self.reveals
            .iter()
            .any(|reveal|
                !reveal.is_valid(reds, blues, greens))
    }
}

struct Reveal {
    reds: u32,
    blues: u32,
    greens: u32,
}

impl Reveal {
    fn new(reds: u32, blues: u32, greens: u32) -> Reveal {
        Reveal { reds, blues, greens }
    }

    fn from_input(text: &str) -> Reveal {
        let pairs: Vec<&str> = text.trim().split(", ").collect();

        let mut reds: u32 = 0;
        let mut blues: u32 = 0;
        let mut greens: u32 = 0;

        for pair in pairs {
            let tokens: Vec<&str> = pair.split(" ").collect();
            let count = tokens[0].parse::<u32>().unwrap();

            match tokens[1].chars().next().unwrap() {
                'r' => reds += count,
                'b' => blues += count,
                'g' => greens += count,
                _ => println!("Failure parsing reveal text: '{}'.  It appears there is no color portion", pair)
            }
        }

        Reveal { reds, blues, greens }
    }

    fn is_valid(&self, reds: u32, blues: u32, greens: u32) -> bool {
        return self.reds <= reds && self.blues <= blues && self.greens <= greens;
    }

    fn display(&self) {
        println!("   Reveal - reds: {}, blues: {}, greens: {}", self.reds, self.blues, self.greens);
    }

    fn get_display(&self) -> String {
        return format!("   Reveal - reds: {}, blues: {}, greens: {}", self.reds, self.blues, self.greens);
    }
}


fn run_tests() {
    test_reveal(&Reveal::new(0, 0, 0), true);
    test_reveal(&Reveal::new(5, 0, 0), true);
    test_reveal(&Reveal::new(0, 5, 0), true);
    test_reveal(&Reveal::new(0, 0, 5), true);
    test_reveal(&Reveal::new(5, 5, 5), true);
    test_reveal(&Reveal::new(12, 0, 0), true);
    test_reveal(&Reveal::new(0, 12, 0), true);
    test_reveal(&Reveal::new(0, 0, 12), true);
    test_reveal(&Reveal::new(0, 13, 0), true);
    test_reveal(&Reveal::new(0, 0, 14), true);
    test_reveal(&Reveal::new(12, 13, 14), true);
    test_reveal(&Reveal::new(99, 0, 0), false);
    test_reveal(&Reveal::new(0, 99, 0), false);
    test_reveal(&Reveal::new(0, 0, 99), false);
    test_reveal(&Reveal::new(99, 99, 99), false);

    test_game(&Game::from_line("Game 1: 0 red, 0 blue, 0 green; 0 red, 0 blue, 0 green"), true);
    test_game(&Game::from_line("Game 1: 12 red, 13 blue, 14 green; 0 red, 0 blue, 0 green"), true);
    test_game(&Game::from_line("Game 1: 13 blue, 14 green, 12 red; 0 red, 0 blue, 0 green"), true);
    test_game(&Game::from_line("Game 1: 99 red, 13 blue, 14 green; 0 red, 0 blue, 0 green"), false);
    test_game(&Game::from_line("Game 1: 0 red, 0 blue, 0 green; 99 red, 13 blue, 14 green; 0 red, 0 blue, 0 green"), false);
    test_game(&Game::from_line("Game 1: 12 red, 13 blue, 14 green; 0 red, 0 blue, 0 green"), true);

}

fn test_reveal(reveal: &Reveal, expected: bool) {

    let actual = reveal.is_valid(12, 13, 14);

    if actual != expected {
        println!("FAIL - {} - Expected: {}, Actual: {}", reveal.get_display(), expected, actual);
    } else {
        println!("PASS - {}", reveal.get_display());
    }
}

fn test_game(game: &Game, expected: bool) {
    let actual = game.is_valid(12, 13, 14);

    if actual != expected {
        println!("FAIL");
    } else {
        println!("PASS");
    }
}