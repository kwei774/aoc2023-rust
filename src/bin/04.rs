advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum_points = 0;
    let cards: Vec<&str> = input.split('\n').collect();
    for card in cards {
        let card_number: Vec<&str> = card.split(':').collect();
        let card_information: Vec<&str> = card_number.get(1).unwrap().split('|').collect();
        let winning_strings: Vec<&str> = card_information.get(0).unwrap().split(' ').collect();
        let hand_strings: Vec<&str> = card_information.get(1).unwrap().split(' ').collect();
        let mut winning_numbers: Vec<i32> = Vec::new();
        let mut hand_numbers: Vec<i32> = Vec::new();
        print!("winning: ");
        for win in winning_strings{
            let parse = win.parse::<i32>();
            if parse.is_ok() {
                winning_numbers.push(parse.unwrap());
                print!("{} ", win)
            }
        }
        println!("");
        print!("hand: ");
        for hand in hand_strings {
            let parse = hand.parse::<i32>();
            if parse.is_ok() {
                hand_numbers.push(parse.unwrap());
                print!("{} ", hand)
            }
        }
        let mut count_winning_numbers = 0;
        for test_number in &hand_numbers {
            for win_number in &winning_numbers {
                if test_number == win_number {
                    println!("match test {} with win {}", test_number, win_number);
                    count_winning_numbers += 1;
                    break;
                }
            }
        }
        if count_winning_numbers > 0 {
            sum_points += 2i32.pow(count_winning_numbers - 1)
        }

        println!("");
    }
    Some(sum_points)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total_scratchcards = 0;
    Some(total_scratchcards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
