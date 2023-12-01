advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines: Vec<&str> = _input.split('\n').collect();
    for line in lines{
        let mut calibration = 0;
        for character in line.chars() {
            if character.is_numeric() {
                calibration = character.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for character in line.chars().rev() {
            if character.is_numeric() {
                calibration += character.to_digit(10).unwrap();
                break;
            }
        }
        sum += calibration
    }
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut numbers_galore = String::from(_input);
    numbers_galore = numbers_galore.replace("one", "o1e");
    numbers_galore = numbers_galore.replace("two", "t2o");
    numbers_galore = numbers_galore.replace("three", "t3e");
    numbers_galore = numbers_galore.replace("four", "f4r");
    numbers_galore = numbers_galore.replace("five", "f5e");
    numbers_galore = numbers_galore.replace("six", "s6x");
    numbers_galore = numbers_galore.replace("seven", "s7n");
    numbers_galore = numbers_galore.replace("eight", "e8t");
    numbers_galore = numbers_galore.replace("nine", "n9e");
    numbers_galore = numbers_galore.replace("zero", "z0o");
    part_one(&numbers_galore)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
#[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
#[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
