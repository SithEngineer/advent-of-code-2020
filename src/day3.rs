mod day3 {
    pub struct Line {
        landscape: Vec<char>,
    }

    impl Line {
        pub fn from_string(input: &String) -> Line {
            Line {
                landscape: input.chars().collect()
            }
        }
        pub fn from_slice(input: &str) -> Line {
            Line::from_string(&input.to_string())
        }

        pub fn is_tree(&self, index: usize) -> bool {
            self.landscape.get(index) == Some(&'#')
        }

        pub fn length(&self) -> usize {
            self.landscape.len()
        }
    }

    pub struct Grid {
        slope_right: usize,
        slope_down: usize,
        current_line: usize,
        current_column: usize,
        ingested_lines: usize,
        pub tree_count: u64,
    }

    impl Grid {
        pub fn new() -> Grid {
            Grid::with_slopes(3, 1)
        }

        pub fn with_slopes(slope_right: usize, slope_down: usize) -> Grid {
            Grid {
                slope_right,
                slope_down,
                current_line: 0,
                current_column: 0,
                ingested_lines: 0,
                tree_count: 0,
            }
        }

        pub fn digest(&mut self, line: &Line) {
            if self.should_process() {
                if line.is_tree(self.current_column % line.length()) {
                    self.tree_count += 1;
                }
                self.current_line += self.slope_down;
                self.current_column += self.slope_right;
            }
            self.ingested_lines += 1;
        }

        fn should_process(&self) -> bool { self.ingested_lines == self.current_line }
    }
}

mod day3_tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use crate::day3::{Grid, Line};

    #[test]
    fn line_is_created() {
        let input = String::from("#...#");
        let line = Line::from_string(&input);
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
            grid.digest(&Line::from_string(&line.unwrap()));
        }

        println!("Number of trees encountered {}", grid.tree_count);
        assert_eq!(294, grid.tree_count);
    }

    #[test]
    fn grid_with_custom_slope() {
        let mut grid = Grid::with_slopes(1, 2);
        let line = Line::from_slice("#.#..");

        grid.digest(&line);
        assert_eq!(1, grid.tree_count);

        grid.digest(&line);
        assert_eq!(1, grid.tree_count);

        grid.digest(&line);
        assert_eq!(1, grid.tree_count);

        grid.digest(&line);
        assert_eq!(1, grid.tree_count);

        grid.digest(&line);
        assert_eq!(2, grid.tree_count);
    }

    #[test]
    fn tree_product_of_multiple_slopes() {
        let file = File::open("resources/day-3-input").unwrap();
        let reader = BufReader::new(file);

        let mut multi_grid: Vec<Grid> = Vec::new();

        multi_grid.push(Grid::with_slopes(1, 1));
        multi_grid.push(Grid::with_slopes(3, 1));
        multi_grid.push(Grid::with_slopes(5, 1));
        multi_grid.push(Grid::with_slopes(7, 1));
        multi_grid.push(Grid::with_slopes(1, 2));

        for line in reader.lines() {
            let parsed_line = Line::from_string(&line.unwrap());
            for grid in multi_grid.iter_mut() {
                grid.digest(&parsed_line)
            }
        }

        for (index, grid) in multi_grid.iter().enumerate() {
            println!("Found {} trees in slope {}", grid.tree_count, index);
        }

        let result = multi_grid.iter().fold(1, |acc, g| acc * g.tree_count);

        println!("Product of number of trees encountered in all slopes {}", result);
        assert!(result > 1);
    }
}