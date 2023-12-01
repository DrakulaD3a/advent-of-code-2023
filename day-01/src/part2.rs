#[must_use]
pub fn process(input: &str) -> u32 {
    let input = replace_nums_by_digits(input);
    input
        .lines()
        .map(|line| {
            let buf: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();

            format!(
                "{}{}",
                buf.first().unwrap_or(&'0'),
                buf.last().unwrap_or(&'0')
            )
            .parse::<u32>()
            .unwrap_or(0)
        })
        .sum()
}

const NUMS_TO_DIGITS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn replace_nums_by_digits(input: &str) -> String {
    let mut input = String::from(input);
    for (old_word, new_word) in NUMS_TO_DIGITS {
        input = input.replace(old_word, new_word);
    };

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        let output = process(input);

        assert_eq!(output, 281);
    }
}
