use std::str::FromStr;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for line in input.lines()
    {
        if let Ok(fuel) = u32::from_str(line) {
            let cost = (fuel / 3) - 2;
            vec.push(cost);
        }
    }

    Some(vec.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for line in input.lines()
    {
        if let Ok(fuel) = u32::from_str(line) {
            vec.push(calculate_fuel_recursive(0, fuel));
        }
    }

    Some(vec.iter().sum())
}

pub fn calculate_fuel_recursive(aggr: u32, fuel: u32) -> u32 {
    if fuel > 6 {
        let cost = (fuel / 3) - 2;
        calculate_fuel_recursive(aggr + cost, cost)
    }
    else
    {
        aggr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33583));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50346));
    }
}
