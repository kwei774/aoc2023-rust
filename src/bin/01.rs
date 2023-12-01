advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    let v: Vec<&str> = _input.split('\n').collect();
    let mut total_calories = 0;
    let mut highest_cal = 0;
    for element in v {
        if element.parse::<u32>().is_ok(){
            total_calories = total_calories + element.parse::<u32>().unwrap();
        } else {
            if highest_cal < total_calories {
                highest_cal = total_calories
            }
            total_calories = 0
        }
    }
    Some(highest_cal)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let v: Vec<&str> = _input.split('\n').collect();
    let mut total_calories = 0;
    let mut highest_calories = vec![0,0,0];
    for element in v {
        if element.parse::<u32>().is_ok(){
            total_calories = total_calories + element.parse::<u32>().unwrap();
        } else {
            if highest_calories[0] < total_calories {
                highest_calories[2] = highest_calories[1];
                highest_calories[1] = highest_calories[0];
                highest_calories[0] = total_calories;
            } else if highest_calories[1] < total_calories {
                highest_calories[2] = highest_calories[1];
                highest_calories[1] = total_calories;
            } else if highest_calories[2] < total_calories {
                highest_calories[2] = total_calories;
            }
            total_calories = 0
        }
    }
    let mut sum_highest_calories = 0;
    for each in highest_calories {
        sum_highest_calories += each
    }
    Some(sum_highest_calories)
}

pub fn first_function() -> i32 {
    return 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41000));
    }

    #[test]
    fn test_print_hello_world () {
        println!("hello world");
        assert_eq!(1, 1);
    }

    #[test]
    fn test_return_integer () {
        let expected = 10;
        assert_eq!(first_function(), expected);
    }
}
