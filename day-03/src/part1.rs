#[rustfmt::skip]
const ADJACENCIES: [(i32, i32); 8] = [
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    (-1,  0),
    ( 1,  0),
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
];

struct Num {
    buf: String,
    adjacent_symbol: bool,
}

#[must_use]
pub fn process(input: &str) -> u32 {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut num = Num {
        buf: String::new(),
        adjacent_symbol: false,
    };

    let mut sum = 0;

    for (line_num, line) in input.iter().enumerate() {
        if !num.buf.is_empty() && num.adjacent_symbol {
            sum += num.buf.parse::<u32>().unwrap();
        }
        num.buf.clear();
        num.adjacent_symbol = false;

        for (char_num, char) in line.iter().enumerate() {
            if char.is_numeric() {
                num.buf.push(*char);

                for adjacency in ADJACENCIES {
                    let Ok(line_cord) = usize::try_from(line_num as i32 + adjacency.0) else {
                        continue;
                    };
                    let Some(adj_line) = input.get(line_cord) else {
                        continue;
                    };

                    let Ok(char_cord) = usize::try_from(char_num as i32 + adjacency.1) else {
                        continue;
                    };
                    let Some(adj) = adj_line.get(char_cord) else {
                        continue;
                    };

                    if !adj.is_numeric() && *adj != '.' {
                        num.adjacent_symbol = true;
                    }
                }
            } else {
                if num.adjacent_symbol {
                    sum += num.buf.parse::<u32>().unwrap();
                }
                num.buf.clear();
                num.adjacent_symbol = false;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let output = process(input);

        assert_eq!(output, 4361);
    }
}
