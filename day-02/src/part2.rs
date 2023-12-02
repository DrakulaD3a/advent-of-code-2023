#[must_use]
pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line.split_once(':').unwrap().1;

            let (mut max_red, mut max_blue, mut max_green) = (1, 1, 1);

            line.trim().split(';').for_each(|draw| {
                draw.split(',').for_each(|color| {
                    let mut color = color.split_whitespace();

                    let num = color.next().unwrap().parse::<u32>().unwrap();
                    match color.next().unwrap() {
                        "red" => {
                            if num > max_red {
                                max_red = num;
                            }
                        }
                        "green" => {
                            if num > max_green {
                                max_green = num;
                            }
                        }
                        "blue" => {
                            if num > max_blue {
                                max_blue = num;
                            }
                        }
                        _ => unreachable!(),
                    };
                });
            });

            max_red * max_green * max_blue
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

        assert_eq!(output, 2286);
    }
}
