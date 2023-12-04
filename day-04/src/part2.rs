#[derive(Debug)]
struct Line<'a> {
    line: &'a str,
    times: i32,
}

#[must_use]
pub fn process(input: &str) -> u32 {
    let mut input = input
        .lines()
        .map(|line| Line { line, times: 1 })
        .collect::<Vec<_>>();

    for i in 0..input.len() {
        if let Some((_, content)) = input.get(i).unwrap().line.split_once(':') {
            if let Some((numbers_i_have, winning_numbers)) = content.split_once('|') {
                let numbers_i_have: Vec<u32> = numbers_i_have
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();
                let winning_numbers: Vec<u32> = winning_numbers
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();

                let mut matches = None;
                for number in numbers_i_have {
                    if winning_numbers.iter().find(|&&num| num == number).is_some() {
                        if matches.is_none() {
                            matches = Some(1);
                        } else {
                            matches = Some(matches.unwrap() + 1);
                        }
                    }
                }

                let multiplier = input.get(i).unwrap().times;
                for j in 1..=matches.unwrap_or(0) {
                    if let Some(line) = input.get_mut(i + j as usize) {
                        line.times += multiplier;
                    } else {
                        break;
                    }
                }
            };
        }
    }

    input.iter().map(|line| line.times).sum::<i32>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let output = process(input);

        assert_eq!(output, 30);
    }
}
