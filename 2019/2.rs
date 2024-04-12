advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut codes: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().expect("parse error"))
        .collect();

    solve(&mut codes)
}

pub fn part_two(input: &str) -> Option<i32> {
    let codes: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().expect("parse error"))
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut codes_copy = codes.clone();
            codes_copy[1] = noun;
            codes_copy[2] = verb;
            if let Some(result) = solve(&mut codes_copy) {
                if result == 19690720 {
                    return Some(100 * noun + verb);
                }
            }
        }
    }
    None
}

fn solve(codes: &mut Vec<i32>) -> Option<i32> {
    let mut index: usize = 0;
    let mut cmd = codes[index];
    while cmd != 99 {
        let input1_index = codes[index + 1] as usize;
        let input2_index = codes[index + 2] as usize;
        let output_index = codes[index + 3] as usize;
        match cmd {
            1 => codes[output_index] = codes[input1_index] + codes[input2_index],
            2 => codes[output_index] = codes[input1_index] * codes[input2_index],
            _ => break,
        }

        index += 4;
        cmd = codes[index];
    }
    Some(codes[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
