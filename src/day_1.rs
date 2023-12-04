/*
decoupled functionality cuz i want my API to be this ugly
maybe u would want to read a single line, maybe line n°42069, who knows ?
 */
pub mod part_1 {
    pub fn parse_line(input: String) -> String {
        let mut digits = input.chars().filter(|x| x.is_ascii_digit());
        let first = digits.next().unwrap_or('0');
        let second = digits.last().unwrap_or(first);
        format!("{first}{second}")
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(String::from("1abc2")), "12");
        assert_eq!(parse_line(String::from("pqr3stu8vwx")), "38");
        assert_eq!(parse_line(String::from("a1b2c3d4e5f")), "15");
        assert_eq!(parse_line(String::from("treb7uchet")), "77");
    }

    pub fn trebuchet(content: String) -> u32 {
        let mut sum: u32 = 0;
        for line in content.lines() {
            sum += parse_line(line.to_string())
                .as_str()
                .parse::<u32>()
                .unwrap();
        }
        sum
    }

    #[test]
    fn test_trebuchet() {
        assert_eq!(
            trebuchet(std::fs::read_to_string("src/txts/testxt/test_trebuchet.txt").unwrap()),
            142
        );
    }
}

pub mod part_2 {
    use std::collections::HashMap;

    pub fn parse_line_2(input: String) -> String {
        let calibrations: HashMap<&str, char> = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        let mut first: char = '0';
        let mut second: char = '0';

        'check_first: for (index, i) in input.chars().enumerate() {
            if i.is_ascii_digit() {
                first = i;
                break 'check_first;
            } else {
                for (key, value) in calibrations.iter() {
                    if input[index..input.len()].starts_with(*key) {
                        first = *value;
                        break 'check_first;
                    } 
                }
            }
        }

        'check_last: for (index, i) in input.chars().rev().enumerate() {
            if i.is_ascii_digit() {
                second = i;
                break 'check_last;
            } else {
                for (key, value) in calibrations.iter() {
                    if input[input.len() - index - 1..input.len()].starts_with(*key) {
                        second = *value;
                        break 'check_last;
                    }
                }
            }
            if second == '0' {
                second = first;
            }
        }
        format!("{first}{second}")
    }

    #[test]
    fn test_parse_line_2() {
        assert_eq!(parse_line_2(String::from("two1nine")), String::from("29"));
        assert_eq!(
            parse_line_2(String::from("eightwothree")),
            String::from("83")
        );
        assert_eq!(
            parse_line_2(String::from("abcone2threexyz")),
            String::from("13")
        );
        assert_eq!(
            parse_line_2(String::from("xtwone3four")),
            String::from("24")
        );
        assert_eq!(
            parse_line_2(String::from("4nineeightseven2")),
            String::from("42")
        );
        assert_eq!(
            parse_line_2(String::from("zoneight234")),
            String::from("14")
        );
        assert_eq!(
            parse_line_2(String::from("7pqrstsixteen")),
            String::from("76")
        );
    }

    pub fn trebuchet_2(content: String) -> u32 {
        let mut sum: u32 = 0;
        for line in content.lines() {
            sum += parse_line_2(line.to_string())
                .as_str()
                .parse::<u32>()
                .unwrap();
        }
        sum
    }

    #[test]
    fn test_trebuchet_2() {
        assert_eq!(
            trebuchet_2(std::fs::read_to_string("src/txts/testxt/test_trebuchet_2.txt").unwrap()),
            281
        );
    }
}
