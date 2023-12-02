const MAX_RED: u32 = 12;
const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;

#[must_use]
pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game_num, line) = line.split_once(':').unwrap();

            let line: bool = line
                .trim()
                .split(';')
                .map(|draw| {
                    let draw = draw
                        .split(',')
                        .map(|color| {
                            let mut color = color.split_whitespace();

                            let num = color.next().unwrap().parse::<u32>().unwrap();
                            match color.next().unwrap() {
                                "red" => num <= MAX_RED,
                                "green" => num <= MAX_GREEN,
                                "blue" => num <= MAX_BLUE,
                                _ => unreachable!(),
                            }
                        })
                        .all(|x| x);

                    draw
                })
                .all(|x| x);

            if line {
                game_num
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                0
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let output = process(input);

        assert_eq!(output, 8);
    }
}
