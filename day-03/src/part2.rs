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

#[derive(Debug, Clone)]
struct Num {
    buf: String,
    adjacent_star: Option<(usize, usize)>,
}

#[must_use]
pub fn process(input: &str) -> u32 {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut num = Num {
        buf: String::new(),
        adjacent_star: None,
    };

    let mut nums = Vec::new();

    for (line_num, line) in input.iter().enumerate() {
        if !num.buf.is_empty() && num.adjacent_star.is_some() {
            nums.push(num.clone());
        }
        num.buf.clear();
        num.adjacent_star = None;

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

                    if !adj.is_numeric() && *adj == '*' {
                        num.adjacent_star = Some((line_cord, char_cord));
                    }
                }
            } else {
                if num.adjacent_star.is_some() {
                    nums.push(num.clone());
                }
                num.buf.clear();
                num.adjacent_star = None;
            }
        }
    }

    nums.sort_by(|a, b| a.adjacent_star.partial_cmp(&b.adjacent_star).unwrap());

    let mut final_nums: Vec<(u32, Option<(usize, usize)>, isize)> = Vec::new();

    for num in nums {
        let pos = final_nums.iter().position(|x| x.1 == num.adjacent_star);
        if let Some(pos) = pos {
            final_nums[pos].0 *= num.buf.parse::<u32>().unwrap();
            final_nums[pos].2 += 1;
        } else {
            final_nums.push((num.buf.parse::<u32>().unwrap(), num.adjacent_star, 1));
        }
    }

    final_nums
        .iter()
        .map(|n| if n.2 > 1 { n.0 } else { 0 })
        .sum()
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

        assert_eq!(output, 467835);
    }
}
