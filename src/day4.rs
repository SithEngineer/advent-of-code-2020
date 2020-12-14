mod day4 {
    use std::collections::HashMap;

    pub struct Passport {
        original_entries: HashMap<String, String>,
    }

    impl Passport {
        pub fn from_slice(input: &str) -> Passport {
            Passport::from(input.to_string())
        }

        /// Parsed fields to build the Passport
        /// byr (Birth Year)
        /// iyr (Issue Year)
        /// eyr (Expiration Year)
        /// hgt (Height)
        /// hcl (Hair Color)
        /// ecl (Eye Color)
        /// pid (Passport ID)
        /// cid (Country ID) // optional
        pub fn from(input: String) -> Passport {
            let mut entries: HashMap<String, String> = HashMap::new();

            for entry in input.split(|c| c == ' ' || c == '\n') {
                if !entry.is_empty() {
                    let key_value: Vec<&str> = entry.split(':').collect();
                    let key = key_value.get(0).unwrap();
                    let value = key_value.get(1).unwrap();
                    entries.insert(key.to_string(), value.to_string());
                }
            }

            Passport { original_entries: entries }
        }

        pub fn has_valid_fields(&self) -> bool {
            (self.original_entries.len() >= 7 && !self.original_entries.contains_key("cid")) ||
                self.original_entries.len() == 8
        }

        pub fn has_valid_data_in_fields(&self) -> bool {
            return self.has_valid_fields()
                && self.has_valid_birth_year()
                && self.has_valid_issued_year()
                && self.has_valid_expiration_year()
                && self.has_valid_height()
                && self.has_valid_hair_color()
                && self.has_valid_eye_color()
                && self.has_valid_passport_id();
        }

        /// byr - four digits; at least 1920 and at most 2002.
        fn has_valid_birth_year(&self) -> bool {
            let birth_year = self.original_entries.get("byr").unwrap().parse::<u16>().unwrap_or(0);
            return birth_year >= 1920 && birth_year <= 2002;
        }

        /// iyr - four digits; at least 2010 and at most 2020.
        fn has_valid_issued_year(&self) -> bool {
            let issued_year = self.original_entries.get("iyr").unwrap().parse::<u16>().unwrap_or(0);
            return issued_year >= 2010 && issued_year <= 2020;
        }

        /// eyr - four digits; at least 2020 and at most 2030.
        fn has_valid_expiration_year(&self) -> bool {
            let expiration_year = self.original_entries.get("eyr").unwrap().parse::<u16>().unwrap_or(0);
            return expiration_year >= 2020 && expiration_year <= 2030;
        }

        /// hgt - a number followed by either cm or in:
        /// If cm, the number must be at least 150 and at most 193.
        /// If in, the number must be at least 59 and at most 76.
        fn has_valid_height(&self) -> bool {
            let default_height = &String::from("0cm");
            let height = self.original_entries.get("hgt").unwrap_or(default_height);
            let (height_value, height_unit) = height.split_at(height.len() - 2);
            return match height_unit {
                "in" => {
                    let height: u8 = height_value.parse::<u8>().unwrap_or(0);
                    height >= 59 && height <= 76
                }
                "cm" => {
                    let height: u8 = height_value.parse::<u8>().unwrap_or(0);
                    height >= 150 && height <= 193
                }
                _ => false
            };
        }

        /// hcl - a # followed by exactly six characters 0-9 or a-f.
        fn has_valid_hair_color(&self) -> bool {
            let default_hair_color = &String::from("");
            let hair_color = self.original_entries.get("hcl").unwrap_or(default_hair_color);
            let nr_hex_color_chars = hair_color.chars().filter(char::is_ascii_alphanumeric).count();
            let control_char = hair_color.chars().nth(0).unwrap_or(' ');
            return control_char == '#' && nr_hex_color_chars == 6;
        }

        const VALID_EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        /// ecl - exactly one of: amb blu brn gry grn hzl oth
        fn has_valid_eye_color(&self) -> bool {
            let default_eye_color = &String::from("");
            let eye_color = self.original_entries.get("ecl").unwrap_or(default_eye_color);
            return Passport::VALID_EYE_COLORS.contains(&eye_color.as_str());
        }

        /// pid - a nine-digit number, including leading zeroes.
        fn has_valid_passport_id(&self) -> bool {
            let default_passport_id = &String::from("0");
            let passport_id = self.original_entries.get("pid").unwrap_or(default_passport_id);
            return passport_id.chars().filter(char::is_ascii_digit).count() == 9;
        }
    }
}

mod day4_tests {
    use std::fs;
    use crate::day4::Passport;

    #[test]
    fn valid_passport_format() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
        let passport = Passport::from_slice(input);
        assert!(passport.has_valid_fields())
    }

    #[test]
    fn invalid_passport_format_missing_height() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929";
        let passport = Passport::from_slice(input);
        assert!(!passport.has_valid_fields())
    }

    #[test]
    fn valid_passport_format_for_north_pole() {
        let input = "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm";
        let passport = Passport::from_slice(input);
        assert!(passport.has_valid_fields())
    }

    #[test]
    fn invalid_passport_format_missing_birth_year() {
        let input = "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
        let passport = Passport::from_slice(input);
        assert!(!passport.has_valid_fields())
    }

    #[test]
    fn input_is_valid() {
        let data: String = fs::read_to_string("resources/day-4-input").unwrap();
        let unparsed_passport: Vec<&str> = data.split("\n\n").collect();
        assert_eq!(291, unparsed_passport.len())
    }

    #[test]
    fn valid_passports_found() {
        let data: String = fs::read_to_string("resources/day-4-input").unwrap();
        let valid_passports: usize =
            data.split("\n\n")
                .map(|item| Passport::from_slice(item))
                .filter(|passport| passport.has_valid_fields())
                .count();
        println!("Found {} valid passports", valid_passports);
        assert!(valid_passports > 0)
    }

    #[test]
    fn input_is_valid_but_height_is_missing_the_unit() {
        let input = "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let passport = Passport::from_slice(input);
        assert!(!passport.has_valid_data_in_fields())
    }

    #[test]
    fn input_is_valid_but_expiration_year_is_in_the_past() {
        let input = "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946";
        let passport = Passport::from_slice(input);
        assert!(!passport.has_valid_data_in_fields())
    }

    #[test]
    fn input_is_valid_but_missing_a_char_in_the_hair_color() {
        let input = "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        let passport = Passport::from_slice(input);
        assert!(!passport.has_valid_data_in_fields())
    }

    #[test]
    fn input_is_valid_but_invalid_eye_color() {
        let input = "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007";
        let passport = Passport::from_slice(input);
        assert!(!passport.has_valid_data_in_fields())
    }

    #[test]
    fn input_is_valid_and_data_is_valid_1() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f";
        let passport = Passport::from_slice(input);
        assert!(passport.has_valid_data_in_fields())
    }

    #[test]
    fn input_is_valid_and_data_is_valid_2() {
        let input = "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        let passport = Passport::from_slice(input);
        assert!(passport.has_valid_data_in_fields())
    }

    #[test]
    fn input_is_valid_and_data_is_valid_3() {
        let input = "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022";
        let passport = Passport::from_slice(input);
        assert!(passport.has_valid_data_in_fields())
    }

    #[test]
    fn input_is_valid_and_data_is_valid_4() {
        let input = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passport = Passport::from_slice(input);
        assert!(passport.has_valid_data_in_fields())
    }

    #[test]
    fn valid_passports_found_with_valid_data() {
        let data: String = fs::read_to_string("resources/day-4-input").unwrap();
        let valid_passports: usize =
            data.split("\n\n")
                .map(|item| Passport::from_slice(item))
                .filter(|passport| passport.has_valid_data_in_fields())
                .count();
        println!("Found {} valid passports", valid_passports);
        assert!(valid_passports > 0)
    }
}