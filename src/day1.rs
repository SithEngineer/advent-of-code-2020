#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use std::fs;

    #[test]
    fn input_is_valid() {
        let file = File::open("resources/day-1-input").unwrap();
        let reader = BufReader::new(file);

        let mut lines = 0;
        for line in reader.lines() {
            assert!(line.unwrap().parse::<u32>().is_ok(), "Each line should be valid integer");
            lines += 1;
        }

        assert_eq!(200, lines, "Input file is expected to have 200 lines");
    }

    #[test]
    fn solve_for_input() {
        let items = fs::read_to_string("resources/day-1-input")
            .expect("file not found")
            .lines()
            .map(|item| item.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for &current_value in items.iter() {
            let expected = 2020 - current_value;
            if items.contains(&expected) {
                println!(
                    "found a={} and b={} where a+b=2020. a*b={}",
                    current_value, expected,
                    current_value * (expected)
                );
                return
            }
        }
    }
}
