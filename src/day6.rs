mod day6 {
    use std::collections::HashMap;

    pub fn group_count(input: &str) -> usize {
        let mut entry_count: HashMap<char, bool> = HashMap::new();
        input
            .chars()
            .filter(char::is_ascii_alphabetic)
            .for_each(|c| { entry_count.entry(c).or_insert(true); });

        return entry_count.values().fold(0, |acc, &exists| {
            if exists {
                acc + 1
            } else {
                acc
            }
        });
    }

    pub fn group_count_all_in_common(input: &str) -> usize {
        let nr_people_in_group =
            input
                .split("\n")
                .filter(|&line| !line.is_empty())
                .count();

        let mut entry_count: HashMap<char, usize> = HashMap::new();
        input
            .chars()
            .filter(char::is_ascii_alphabetic)
            .for_each(|c| {
                let entry = entry_count.entry(c).or_insert(0);
                *entry += 1;
            });

        return entry_count.values().fold(0, |acc, &count| {
            if count == nr_people_in_group {
                acc + 1
            } else {
                acc
            }
        });
    }

    pub fn group_chunk(input: &str) -> Vec<&str> {
        input.split("\n\n").filter(|&line| !line.is_empty()).collect()
    }
}

mod day6_tests {
    use std::fs;
    use crate::day6::{group_count, group_count_all_in_common, group_chunk};

    #[test]
    fn group_count_is_correct() {
        let input_line = "abc";
        let count = group_count(input_line);
        assert_eq!(count, 3);
    }

    #[test]
    fn group_count_for_duplicates_does_not_count() {
        let input_line = "aaa";
        let count = group_count(input_line);
        assert_eq!(count, 1);
    }

    #[test]
    fn group_count_is_zero() {
        let input_line = "123";
        let count = group_count(input_line);
        assert_eq!(count, 0);
    }

    #[test]
    fn group_chunk_works() {
        let input = "a\n\na";
        let groups = group_chunk(input).len();
        assert_eq!(groups, 2);
    }

    #[test]
    fn multi_group_chunk_and_count_properly() {
        let input = "a\n\nabc\n\n123\n\ncd";

        let count = group_chunk(input)
            .iter()
            .map(|&line| group_count(line))
            .fold(0, |v, acc| acc + v);

        assert_eq!(count, 6);
    }

    #[test]
    fn check_if_input_only_contains_letters_a_to_z_and_new_lines() {
        let data = fs::read_to_string("resources/day-6-input").unwrap();
        let invalid_chars_found = data.chars().filter(|c| c.is_ascii_alphabetic() && c.is_whitespace()).count();
        assert_eq!(0, invalid_chars_found)
    }

    #[test]
    fn solve_part_1() {
        let data = fs::read_to_string("resources/day-6-input").unwrap();

        let sum = group_chunk(data.as_str())
            .iter()
            .map(|&line| group_count(line))
            .fold(0, |acc, x| acc + x);

        assert!(sum > 0);
        println!("Sum of positive answers in groups is {}", sum);
    }

    #[test]
    fn group_commons_count_is_correct() {
        let input_line = "abc\n";
        let count = group_count_all_in_common(input_line);
        assert_eq!(count, 3);
    }

    #[test]
    fn group_commons_count_is_none_for_three_people() {
        let input_line = "a\nb\nc\n";
        let count = group_count_all_in_common(input_line);
        assert_eq!(count, 0);
    }

    #[test]
    fn group_commons_count_is_one_for_two_people() {
        let input_line = "ab\nac\n";
        let count = group_count_all_in_common(input_line);
        assert_eq!(count, 1);
    }

    #[test]
    fn group_commons_count_is_one_for_all_people() {
        let input_line = "a\na\na\na\n";
        let count = group_count_all_in_common(input_line);
        assert_eq!(count, 1);
    }

    #[test]
    fn group_commons_count_is_zero_for_all_people_and_non_correct_answers() {
        let input_line = "a\n1\n2\na\n";
        let count = group_count_all_in_common(input_line);
        assert_eq!(count, 0);
    }

    #[test]
    fn group_commons_count_is_one_for_single_person() {
        let input_line = "b\n";
        let count = group_count_all_in_common(input_line);
        assert_eq!(count, 1);
    }

    #[test]
    fn solve_part_2() {
        let data = fs::read_to_string("resources/day-6-input").unwrap();

        let sum = group_chunk(data.as_str())
            .iter()
            .map(|&line| group_count_all_in_common(line))
            .fold(0, |acc, x| acc + x);

        assert!(sum > 0);
        println!("Sum of positive _common_ answers in groups is {}", sum);
    }
}