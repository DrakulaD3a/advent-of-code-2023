pub fn process(input: &str) -> u32 {
    let lines = input.split('\n');
    let mut output = 0;

    for line in lines {
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
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let output = process(input);

        assert_eq!(output, 142);
    }
}
