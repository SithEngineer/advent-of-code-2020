mod day3 {
    pub struct Line {
        landscape: Vec<char>,
    }

    impl Line {
        pub fn from_string(input: String) -> Line {
            Line {
                landscape: input.chars().collect()
            }
        }
        pub fn from_slice(input: &str) -> Line {
            Line::from_string(input.to_string())
        }

        pub fn is_tree(&self, index: usize) -> bool {
            self.landscape.get(index) == Some(&'#')
        }

        pub fn length(&self) -> usize {
            self.landscape.len()
        }
    }

    const SLOPE_RIGHT: usize = 3;
    const SLOPE_DOWN: usize = 1;

    pub struct Grid {
        current_line: usize,
        current_column: usize,
        pub tree_count: usize,
    }

    impl Grid {
        pub fn new() -> Grid {
            Grid {
                current_line: 0,
                current_column: 0,
                tree_count: 0,
            }
        }

        pub fn digest(&mut self, line: &Line) {
            if line.is_tree(self.current_column % line.length()) {
                self.tree_count += 1;
            }
            self.current_line += SLOPE_DOWN;
            self.current_column += SLOPE_RIGHT;
        }
    }
}

mod day3_tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use crate::day3::{Grid, Line};

    #[test]
    fn line_is_created() {
        let input = String::from("#...#");
        let line = Line::from_string(input);
        assert!(line.is_tree(0));
        assert!(!line.is_tree(1));
        assert!(!line.is_tree(2));
        assert!(!line.is_tree(3));
        assert!(line.is_tree(4));
        assert_eq!(5, line.length());
    }

    #[test]
    fn grid_initial_state() {
        let grid = Grid::new();
        assert_eq!(0, grid.tree_count);
    }

    #[test]
    fn grid_with_line() {
        let mut grid = Grid::new();
        let line = Line::from_slice("#...#");

        grid.digest(&line);
        assert_eq!(1, grid.tree_count);

        grid.digest(&line);
        assert_eq!(1, grid.tree_count);
    }

    #[test]
    fn input_is_valid() {
        let file = File::open("resources/day-3-input").unwrap();
        let reader = BufReader::new(file);

        let mut valid_lines: usize = 0;
        for line in reader.lines() {
            let invalid_chars =
                line.unwrap().chars().filter(|&c| c != '#' && c != '.').count();
            if invalid_chars == 0 { valid_lines += 1; }
        }

        assert_eq!(323, valid_lines, "Input file is expected to have 323 lines");
    }

    #[test]
    fn count_trees() {
        let file = File::open("resources/day-3-input").unwrap();
        let reader = BufReader::new(file);

        let mut grid = Grid::new();
        for line in reader.lines() {
            grid.digest(&Line::from_string(line.unwrap()))
        }

        println!("Number of trees encountered {}", grid.tree_count);
        assert!(grid.tree_count > 0);
    }
}