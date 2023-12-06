use std::collections::HashMap;

#[must_use]
pub fn process(input: &str) -> u32 {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let time_to_distance: HashMap<u32, u32> = times
        .iter()
        .enumerate()
        .map(|(i, &time)| (time, distances[i]))
        .collect();

    time_to_distance
        .iter()
        .map(|(&time, &distance)| {
            let mut sum = 0;
            for charging in 1..time {
                if (time - charging) * charging > distance {
                    sum += 1;
                }
            }

            sum
        })
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let output = process(input);

        assert_eq!(output, 288);
    }
}
