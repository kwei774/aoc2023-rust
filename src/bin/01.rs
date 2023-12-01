advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

pub fn first_function() -> i32 {
    return 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
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
