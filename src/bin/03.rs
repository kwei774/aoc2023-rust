advent_of_code::solution!(3);
use std::cmp;
use std::collections::HashMap;


pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_part_numbers = 0;
    let rows: Vec<&str> = input.split('\n').collect();
    let mut two_d_diagram: Vec<Vec<char>> = Vec::new();
    for input in rows {
        let characters: Vec<char> = input.chars().collect();
        two_d_diagram.push(characters);
    }

    for row_number in 0..two_d_diagram.len(){
        let mut found_number = false;
        let mut current_number = 0;
        let mut current_number_start = 0;
        for column_number in 0..two_d_diagram.get(row_number).unwrap().len(){
            if two_d_diagram[row_number][column_number].is_numeric() {
                if found_number {
                    current_number *= 10;
                    current_number += two_d_diagram[row_number][column_number].to_digit(10).unwrap();
                } else {
                    current_number = two_d_diagram[row_number][column_number].to_digit(10).unwrap();
                    current_number_start = column_number;
                    found_number = true;
                }
                if column_number == two_d_diagram.get(row_number).unwrap().len() - 1{
                    let search_begin_column_number = cmp::max(current_number_start as i32 - 1, 0);
                    let search_end_column_number = cmp::min(column_number as i32, (two_d_diagram.get(row_number).unwrap().len() - 1) as i32);
                    let search_begin_row_number = cmp::max(row_number as i32 - 1, 0);
                    let search_end_row_number = cmp::min((row_number + 1) as i32, (two_d_diagram.len() - 1) as i32);
                    for search_column in search_begin_column_number..search_end_column_number + 1{
                        for search_row in search_begin_row_number..search_end_row_number + 1{
                            if is_symbol_but_not_period(two_d_diagram[search_row as usize][search_column as usize]){
                                sum_part_numbers += current_number;
                                break;
                            }
                        }
                    }
                }
            } else {
                if found_number {
                    let search_begin_column_number = cmp::max(current_number_start as i32 - 1, 0);
                    let search_end_column_number = cmp::min(column_number as i32, (two_d_diagram.get(row_number).unwrap().len() - 1) as i32);
                    let search_begin_row_number = cmp::max(row_number as i32 - 1, 0);
                    let search_end_row_number = cmp::min((row_number + 1) as i32, (two_d_diagram.len() - 1) as i32);
                    for search_column in search_begin_column_number..search_end_column_number + 1{
                        for search_row in search_begin_row_number..search_end_row_number + 1{
                            if is_symbol_but_not_period(two_d_diagram[search_row as usize][search_column as usize]){
                                sum_part_numbers += current_number;
                                break;
                            }
                        }
                    }
                    current_number = 0;
                    found_number = false;
                }
            }
        }
    }
    Some(sum_part_numbers)
}

pub fn is_symbol_but_not_period(input: char) -> bool {
    return input.is_ascii_punctuation() && (input != '.')
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_gear_ratios = 0;
    let rows: Vec<&str> = input.split('\n').collect();
    let mut two_d_diagram: Vec<Vec<char>> = Vec::new();
    for input in rows {
        let characters: Vec<char> = input.chars().collect();
        two_d_diagram.push(characters);
    }

    for row_number in 0..two_d_diagram.len(){
        let mut found_number = false;
        let mut current_number = 0;
        for column_number in 0..two_d_diagram.get(row_number).unwrap().len(){
            if two_d_diagram[row_number][column_number] == '*' {
                println!("{}, {}", column_number, row_number);
                let search_begin_column_number = cmp::max(column_number as i32 - 1, 0);
                let search_end_column_number = cmp::min((column_number + 1) as i32, (two_d_diagram.get(row_number).unwrap().len() - 1) as i32);
                let search_begin_row_number = cmp::max(row_number as i32 - 1, 0);
                let search_end_row_number = cmp::min((row_number + 1) as i32, (two_d_diagram.len() - 1) as i32);
                for search_row in search_begin_row_number..search_end_row_number + 1{
                    for search_column in search_begin_column_number..search_end_column_number + 1{
                        println!("Searching {}, {} and found {}", search_column, search_row, two_d_diagram[search_row as usize][search_column as usize]);
                        if two_d_diagram[search_row as usize][search_column as usize].is_digit(10) {
                            current_number += two_d_diagram[search_row as usize][search_column as usize].to_digit(10).unwrap();
                            
                        }
                    }
                }
            }
        }
    }
    Some(sum_gear_ratios)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_punctuation() {
        assert_eq!(is_symbol_but_not_period('$'), true);
        assert_eq!(is_symbol_but_not_period('.'), false);
        assert_eq!(is_symbol_but_not_period('/'), true);
        assert_eq!(is_symbol_but_not_period(','), true);
    }

}
