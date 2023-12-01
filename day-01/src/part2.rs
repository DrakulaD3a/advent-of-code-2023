pub fn process(input: &str) -> u32 {
    let mut lines = input.split('\n');
    let mut output = 0;

    for line in &mut lines {
        // TODO: Make this pretty
        let line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        let line_copy = line.clone();
        let mut buf = String::new();

        for c in line.chars() {
            if c.is_numeric() {
                buf.push(c);
                break;
            }
        }

        for c in line_copy.chars().rev() {
            if c.is_numeric() {
                buf.push(c);
                break;
            }
        }

        output += buf.parse::<u32>().unwrap_or(0);
    }

    output
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
