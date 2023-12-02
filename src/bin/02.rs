use core::num;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum_game_ids = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    for line in lines{
        let mut possible = true;
        let info: Vec<&str> = line.split(':').collect();
        let game_info: Vec<&str> = info[0].split(' ').collect();
        let game_id = String::from(game_info[1]).parse::<i32>().unwrap();
        let handful_of_cubes: Vec<&str> = info[1].split(';').collect();
        let mut color_cube_count: Vec<&str> = Vec::new();
        for handful in handful_of_cubes {
            let mut color_cube: Vec<&str> = handful.split(',').collect();
            color_cube_count.append(&mut color_cube)
        }
        println!("game_id: {}", game_id);
        for color_cubes in color_cube_count {
            let color_info: Vec<&str> = color_cubes.split(' ').collect();
            let number_of_cubes = String::from(color_info[1]).parse::<i32>().unwrap();
            if color_info[2] == "red" && number_of_cubes > 12{
                possible = false;
                break;
            }
            if color_info[2] == "green" && number_of_cubes > 13{
                possible = false;
                break;
            }
            if color_info[2] == "blue" && number_of_cubes > 14{
                possible = false;
                break;
            }
            println!("item: {}", color_cubes);
        }
        if possible {
            sum_game_ids += game_id
        }
        println!("sum_game_ids: {}", sum_game_ids);
    }
    Some(sum_game_ids)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum_power = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    for line in lines{
        let info: Vec<&str> = line.split(':').collect();
        let game_info: Vec<&str> = info[0].split(' ').collect();
        let game_id = String::from(game_info[1]).parse::<i32>().unwrap();
        let handful_of_cubes: Vec<&str> = info[1].split(';').collect();
        let mut color_cube_count: Vec<&str> = Vec::new();
        for handful in handful_of_cubes {
            let mut color_cube: Vec<&str> = handful.split(',').collect();
            color_cube_count.append(&mut color_cube)
        }
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        println!("game_id: {}", game_id);
        for color_cubes in color_cube_count {
            let color_info: Vec<&str> = color_cubes.split(' ').collect();
            let number_of_cubes = String::from(color_info[1]).parse::<i32>().unwrap();
            if color_info[2] == "red" && number_of_cubes > max_red{
                max_red = number_of_cubes;
            }
            if color_info[2] == "green" && number_of_cubes > max_green{
                max_green = number_of_cubes;
            }
            if color_info[2] == "blue" && number_of_cubes > max_blue{
                max_blue = number_of_cubes;
            }
            println!("item: {}", color_cubes);
        }
        let power = max_red * max_green * max_blue;
        sum_power += power;
    }
    Some(sum_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
