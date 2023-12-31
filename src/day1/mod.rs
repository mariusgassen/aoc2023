use crate::aoc::{
    Day,
    tools::{
        find_char, find_substrings, read_lines, rfind_char, rfind_substrings, string_to_digit,
    },
};

pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        Day1 { input }
    }
}

impl Day for Day1 {
    fn part1(&self) -> String {
        let lines = read_lines(&self.input);

        let mut sum = 0;
        for l in lines.iter() {
            let first = find_char(l, |c| c.is_numeric()).unwrap_or('0');
            let last = rfind_char(l, |c| c.is_numeric()).unwrap_or('0');
            let s = format!("{}{}", first, last);
            sum += s.parse::<u32>().unwrap();
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let lines = read_lines(&self.input);

        let mut sum = 0;
        for l in lines.iter() {
            let search = vec![
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
            ];

            let first = find_substrings(l, &search).unwrap_or(String::from("0"));
            let last = rfind_substrings(l, &search).unwrap_or(String::from("0"));

            let first_digit = string_to_digit(&first).unwrap_or(0);
            let last_digit = string_to_digit(&last).unwrap_or(0);
            let s = format!("{}{}", first_digit, last_digit);

            sum += s.parse::<u32>().unwrap();
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    #[test]
    fn test_part_1() {
        let input = r#"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;

        let day1 = super::Day1::new(input.to_string());
        assert_eq!(day1.part1(), "142");
    }

    #[test]
    fn test_part_2() {
        let input = r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#;

        let day1 = super::Day1::new(input.to_string());
        assert_eq!(day1.part2(), "281");
    }

    #[test]
    fn test_no_digit() {
        let input = "abc";

        let day1 = super::Day1::new(input.to_string());
        assert_eq!(day1.part1(), "0");
    }

    #[test]
    fn test_no_digit_2() {
        let input = "abc";

        let day1 = super::Day1::new(input.to_string());
        assert_eq!(day1.part2(), "0");
    }

    // Assuming each line contains exactly two digits or none
    // #[test]
    // fn test_one_digit() {
    //     let input = "1abc";
    //
    //     let day1 = super::Day1::new(input.to_string());
    //     assert_eq!(day1.run(), "1");
    // }
}
