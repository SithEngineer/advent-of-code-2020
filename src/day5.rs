mod day5 {
    pub fn seat_id(input: &str) -> u16 {
        row(input) * 8 + column(input)
    }

    pub fn row(input: &str) -> u16 {
        let transformed_row = input.chars().take(7).map(|c| {
            match c {
                'B' => { 'H' }
                'F' => { 'L' }
                _ => '?'
            }
        }).into_iter().collect();
        binary_search(&transformed_row, 0, 127)
    }

    pub fn column(input: &str) -> u16 {
        let transformed_row = input.chars().skip(7).map(|c| {
            match c {
                'R' => { 'H' }
                'L' => { 'L' }
                _ => '?'
            }
        }).into_iter().collect();
        binary_search(&transformed_row, 0, 7)
    }

    fn binary_search(input: &String, lower_bound: u16, upper_bound: u16) -> u16 {
        let mut lower: u16 = lower_bound;
        let mut upper: u16 = upper_bound;
        let all_but_last = input.len() - 1;
        input.chars().take(all_but_last).for_each(|c| {
            match c {
                'H' => {
                    let remainder = (upper - lower) % 2;
                    lower += (upper - lower) / 2;
                    lower += remainder;
                }
                'L' => {
                    let remainder = (upper - lower) % 2;
                    upper -= (upper - lower) / 2;
                    upper -= remainder;
                }
                _ => {}
            }
        });
        return match input.chars().last().unwrap() {
            'H' => { upper }
            'L' => { lower }
            _ => 0
        };
    }
}

mod day5_tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use crate::day5::{seat_id, row, column};

    #[test]
    fn solve_part1() {
        let file = File::open("resources/day-5-input").unwrap();
        let reader = BufReader::new(file);

        let biggest_seat_id = reader.lines()
            .map(|line| seat_id(&line.unwrap()))
            .max()
            .unwrap_or(0);

        assert!(biggest_seat_id > 0);
        assert!(biggest_seat_id <= 1023);
        println!("Biggest seat id is {}", biggest_seat_id);
    }

    #[test]
    fn check_specific_seat_id1() {
        let input = "BFFFBBFRRR";

        let row = row(input);
        assert_eq!(70, row);

        let column = column(input);
        assert_eq!(7, column);

        let seat_id = seat_id(input);
        assert_eq!(567, seat_id);
    }

    #[test]
    fn check_specific_seat_id2() {
        let input = "FFFBBBFRRR";

        let row = row(input);
        assert_eq!(14, row);

        let column = column(input);
        assert_eq!(7, column);

        let seat_id = seat_id(input);
        assert_eq!(119, seat_id);
    }

    #[test]
    fn check_specific_seat_id3() {
        let input = "BBFFBBFRLL";

        let row = row(input);
        assert_eq!(102, row);

        let column = column(input);
        assert_eq!(4, column);

        let seat_id = seat_id(input);
        assert_eq!(820, seat_id);
    }

    #[test]
    fn solve_part2() {
        let file = File::open("resources/day-5-input").unwrap();
        let reader = BufReader::new(file);

        let mut existing_ids: Vec<u16> = reader.lines()
            .map(|line| seat_id(&line.unwrap()))
            .collect();
        existing_ids.sort();

        let lowest_existing_id: u16 = *existing_ids.first().unwrap();
        let highest_existing_id: u16 = *existing_ids.last().unwrap();

        let remaining_ids: Vec<u16> =
            (lowest_existing_id..highest_existing_id)
                .step_by(1)
                .filter(|&id| {
                    !existing_ids.contains(&id)
                        && existing_ids.contains(&(id - 1))
                        && existing_ids.contains(&(id + 1))
                })
                .collect();

        assert_eq!(remaining_ids.len(), 1);
        println!("Remaining seat ids: {:?}", remaining_ids);
    }
}