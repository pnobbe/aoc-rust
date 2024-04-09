use std::str::FromStr;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).iter().max().cloned()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut data = parse(input);
    data.sort();

    Some(data.iter().rev().take(3).sum())
}

fn parse(input: &str) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    let mut aggr: u32 = 0;
    for line in input.lines()
    {
        if let Ok(x) = u32::from_str(line) {
            aggr += x;
        }
        else {
            vec.push(aggr);
            aggr = 0;
        }
    }
    vec.push(aggr);

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
