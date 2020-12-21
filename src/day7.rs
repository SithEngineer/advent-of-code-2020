mod day7 {
    use std::borrow::BorrowMut;
    use std::collections::HashMap;

    pub struct Storage {
        bags: HashMap<String, Bag>,
    }

    impl Storage {
        pub fn new() -> Self {
            Storage {
                bags: HashMap::new(),
            }
        }

        pub fn get_bag(&self, bag_name: &str) -> &Bag {
            self.bags.get(bag_name).unwrap()
        }

        fn digest_line(&mut self, line: &str) {
            let parts: Vec<&str> = line.replace('.', "").split(" bags contain ").collect();

            let &mut bag = self
                .bags
                .get(parts[0])
                .get_or_insert(&Bag::new(parts[0].to_string()));

            //bag.contains.
        }

        pub fn extract_bag_name_and_count(input: &str) -> (&str, usize) {
            let (count_slice, name_slice) = input.trim().split_at(input.find(' ').unwrap());
            return match count_slice {
                "no" => ("", 0),
                _ => {
                    let count: usize = count_slice.parse().unwrap();
                    let name: Vec<&str> = name_slice.split("bag").collect();
                    return (name[0].trim(), count);
                }
            };
        }
    }

    pub struct Bag {
        name: String,
        contains: HashMap<Bag, usize>,
        is_contained_by: Vec<Bag>,
    }

    impl Bag {
        fn new(name: String) -> Self {
            Bag {
                name,
                contains: HashMap::new(),
                is_contained_by: Vec::new(),
            }
        }

        pub fn is_contained_by_len(&self) -> usize {
            return self.is_contained_by.len();
        }
    }
}

mod day7_tests {
    use crate::day7::{Bag, Storage};

    #[test]
    fn contained_bag_name_and_count_is_found1() {
        let input = "no other bags";

        let (name, count) = Storage::extract_bag_name_and_count(input);

        assert_eq!("", name);
        assert_eq!(0, count);
    }

    #[test]
    fn contained_bag_name_and_count_is_found2() {
        let input = "1 bright white bag";

        let (name, count) = Storage::extract_bag_name_and_count(input);

        assert_eq!("bright white", name);
        assert_eq!(1, count);
    }

    #[test]
    fn contained_bag_name_and_count_is_found3() {
        let input = "2 muted yellow bags";

        let (name, count) = Storage::extract_bag_name_and_count(input);

        assert_eq!("muted yellow", name);
        assert_eq!(2, count);
    }

    #[test]
    fn main_bag_is_found() {
        let input = "bright white bags contain 1 shiny gold bag.";

        let mut storage = Storage::new();
        storage.digest(input);

        let shiny_gold_bag: &Bag = storage.get_bag("shiny gold");

        assert_eq!(1, shiny_gold_bag.is_contained_by_len());
    }

    #[test]
    fn bag_that_contains_no_more_bags() {
        let input_line1 = "faded blue bags contain no other bags.";
        let input_line2 = "dotted black bags contain no other bags.";

        let mut storage = Storage::new();
        storage.digest(input_line1);
        storage.digest(input_line2);

        let faded_blue_bag: &Bag = storage.get_bag("faded blue");
        let dotted_black_bag: &Bag = storage.get_bag("dotted black");

        assert_eq!(0, faded_blue_bag.is_contained_by_len());
        assert_eq!(0, dotted_black_bag.is_contained_by_len());
    }
}
