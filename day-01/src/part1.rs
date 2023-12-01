#[must_use]
pub fn process(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let output = process(input);

        assert_eq!(output, 142);
    }
}
