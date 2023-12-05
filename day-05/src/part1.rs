#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Map {
    _source_category: Category,
    _destination_category: Category,
    sources_and_destinations: Vec<SourceAndDest>,
}

#[derive(Debug)]
struct SourceAndDest {
    destination: u64,
    source: u64,
    range_length: u64,
}

#[must_use]
pub fn process(input: &str) -> u64 {
    let mut seeds: Vec<u64> = Vec::new();

    let input = input
        .split("\n\n")
        .filter_map(|category| {
            let Some((name, value)) = category.split_once(":") else {
                panic!("Split as ':' failed");
            };

            if name == "seeds" {
                seeds = value
                    .split_whitespace()
                    .map(|seed| seed.parse::<u64>().unwrap())
                    .collect();

                None
            } else {
                let conversion = name
                    .split_whitespace()
                    .nth(0)
                    .unwrap()
                    .split("-to-")
                    .map(|category| {
                        use Category as C;
                        match category {
                            "seed" => C::Seed,
                            "soil" => C::Soil,
                            "fertilizer" => C::Fertilizer,
                            "water" => C::Water,
                            "light" => C::Light,
                            "temperature" => C::Temperature,
                            "humidity" => C::Humidity,
                            "location" => C::Location,
                            _ => unreachable!(),
                        }
                    })
                    .collect::<Vec<_>>();

                let sources_and_destinations: Vec<_> = value
                    .trim()
                    .lines()
                    .map(|line| {
                        let line_nums = line
                            .split_whitespace()
                            .map(|num| num.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();

                        SourceAndDest {
                            destination: line_nums[0],
                            source: line_nums[1],
                            range_length: line_nums[2],
                        }
                    })
                    .collect();

                Some(Map {
                    _source_category: conversion[0],
                    _destination_category: conversion[1],
                    sources_and_destinations,
                })
            }
        })
        .collect::<Vec<Map>>();

    input.iter().for_each(|map| {
        'outer: for seed in seeds.iter_mut() {
            for source_and_dest in map.sources_and_destinations.iter() {
                let source_range =
                    source_and_dest.source..(source_and_dest.source + source_and_dest.range_length);

                if source_range.contains(seed) {
                    *seed = (*seed - source_and_dest.source) + source_and_dest.destination;
                    continue 'outer;
                }
            }
        }
    });

    *seeds.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let output = process(input);

        assert_eq!(output, 35);
    }
}
