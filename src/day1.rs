mod day1 {
    use rand;

    /// find a and b where a+b=2020
    pub fn find_double(items: &Vec<i32>) -> [i32; 2] {
        let sum_amount = 2020;
        let mut double: [i32; 2] = [0, 0];

        for &current_value in items.iter() {
            let expected = sum_amount - current_value;
            if items.contains(&expected) {
                double[0] = current_value;
                double[1] = expected;
                break;
            }
        }

        return double;
    }

    /// find a, b and c where a+b+c=2020
    pub fn find_triplet(items: &Vec<i32>) -> [i32; 3] {
        let total_nr_items = items.len();
        let sum_amount = 2020;
        let mut triplet: [usize; 3] = [0, 1, 2];

        while items[triplet[0]] + items[triplet[1]] + items[triplet[2]] != sum_amount {
            triplet = random_triplet(&total_nr_items);
        }

        return [items[triplet[0]], items[triplet[1]], items[triplet[2]]];
    }

    fn random_triplet(max_len: &usize) -> [usize; 3] {
        return [
            rand::random::<usize>() % max_len,
            rand::random::<usize>() % max_len,
            rand::random::<usize>() % max_len
        ];
    }
}

mod day1_tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use std::fs;
    use crate::day1::{find_double, find_triplet};

    #[test]
    fn input_is_valid() {
        let file = File::open("resources/day-1-input").unwrap();
        let reader = BufReader::new(file);

        let mut lines = 0;
        for line in reader.lines() {
            assert!(line.unwrap().parse::<i32>().is_ok(), "Each line should be valid integer");
            lines += 1;
        }

        assert_eq!(200, lines, "Input file is expected to have 200 lines");
    }

    #[test]
    fn solve_part_1() {
        let items = get_items();

        let double: [i32; 2] = find_double(&items);
        let sum = double[0] + double[1];
        let result = double[0] * double[1];

        println!(
            "Found a={} and b={} where a+b={}. a*b={}",
            double[0], double[1], sum, result
        );

        assert_eq!(2020, sum);
        assert!(double[0] > 0);
        assert!(double[1] > 0);
        assert!(result > 0);
    }

    #[test]
    fn solve_part_2() {
        let items = get_items();

        let triplet: [i32; 3] = find_triplet(&items);
        let sum = triplet[0] + triplet[1] + triplet[2];
        let result = triplet[0] * triplet[1] * triplet[2];

        println!(
            "Found a={}, b={} and c={} where a+b+c={}. a*b*c={}",
            triplet[0], triplet[1], triplet[2], sum, result
        );

        assert_eq!(2020, sum);
        assert!(triplet[0] > 0);
        assert!(triplet[1] > 0);
        assert!(triplet[2] > 0);
        assert!(result > 0);
    }

    fn get_items() -> Vec<i32> {
        return fs::read_to_string("resources/day-1-input")
            .expect("file not found")
            .lines()
            .map(|item| item.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
    }
}
