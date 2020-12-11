

mod day2 {
    #[derive(Debug)]
    pub struct PolicyWithPassword {
        pub min_count: usize,
        pub max_count: usize,
        pub pass_char: char,
        pub existing_password: String
    }

    impl PolicyWithPassword {
        pub fn from_slice(line: &str) -> PolicyWithPassword {
            PolicyWithPassword::from_string(line.to_string())
        }

        pub fn from_string(line: String) -> PolicyWithPassword {
            let parts: Vec<&str> = line.split(
                |c| c==' ' || c== '-' || c==':'
            ).filter(
                |part| !part.is_empty()
            ).collect();

            PolicyWithPassword {
                min_count: parts.get(0).unwrap().parse::<usize>().unwrap(),
                max_count: parts.get(1).unwrap().parse::<usize>().unwrap(),
                pass_char: parts.get(2).unwrap().parse::<char>().unwrap(),
                existing_password: parts.get(3).unwrap().to_string(),
            }
        }

        pub fn is_valid(&self) -> bool {
            let char_count = self.existing_password.chars().filter(|c| c.eq(&self.pass_char)).count();
            char_count>=self.min_count && char_count<=self.max_count
        }
    }

}

mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use crate::day2::{PolicyWithPassword};

    #[test]
    fn policy_is_extracted_from_string() {
        let line = String::from("1-2 t: test");
        let policy = PolicyWithPassword::from_string(line);
        assert_eq!(1, policy.min_count);
        assert_eq!(2, policy.max_count);
        assert_eq!('t', policy.pass_char);
        assert_eq!(String::from("test"), policy.existing_password);
    }

    #[test]
    fn policy_is_extracted_from_slice() {
        let line: &str = "1-2 t: test";
        let policy = PolicyWithPassword::from_slice(line);
        assert_eq!(1, policy.min_count);
        assert_eq!(2, policy.max_count);
        assert_eq!('t', policy.pass_char);
        assert_eq!(String::from("test"), policy.existing_password);
    }

    #[test]
    fn policy_validates_password() {
        let line: &str = "1-2 t: test";
        let policy = PolicyWithPassword::from_slice(line);
        assert!(policy.is_valid());
    }

    #[test]
    fn policy_invalidates_password() {
        let line: &str = "1-2 f: anothertest";
        let policy = PolicyWithPassword::from_slice(line);
        assert!(!policy.is_valid());
    }

    #[test]
    fn input_is_valid() {
        let file = File::open("resources/day-2-input").unwrap();
        let reader = BufReader::new(file);

        let mut policies : Vec<PolicyWithPassword> = Vec::new();
        for line in reader.lines() {
            policies.push(PolicyWithPassword::from_string(line.unwrap()))
        }

        assert_eq!(1000, policies.len(), "Input file is expected to have 200 lines");
    }

    #[test]
    fn count_valid_passwords() {
        let file = File::open("resources/day-2-input").unwrap();
        let reader = BufReader::new(file);

        let mut policies : Vec<PolicyWithPassword> = Vec::new();
        for line in reader.lines() {
            policies.push(PolicyWithPassword::from_string(line.unwrap()))
        }

        let valid_passwords = policies.iter().filter(|p| p.is_valid()).count();
        print!("Valid passwords found={}", valid_passwords);
        assert!(valid_passwords>0);
    }
}