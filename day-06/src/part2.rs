#[must_use]
pub fn process(input: &str) -> u64 {
    let time = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter(|n| n.chars().nth(0).unwrap().is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter(|n| n.chars().nth(0).unwrap().is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut sum = 0;
    for charging in 1..time {
        if (time - charging) * charging > distance {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let output = process(input);

        assert_eq!(output, 71503);
    }
}
