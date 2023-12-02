pub mod part_1 {
    pub fn parse_line(input: String) -> String {
        let mut digits = input.chars().filter(|x| x.is_ascii_digit());
        let first = digits.next().unwrap_or('0');
        let second = digits.last().unwrap_or(first);
        format!("{first}{second}")
    }

    pub fn trebuchet(content: String) -> u32 {
        let mut sum: u32 = 0;
        for line in content.lines() {
            println!("we have {line}");
            sum += parse_line(line.to_string()).as_str().parse::<u32>().unwrap();
            println!("sum is {sum}");
        }
        sum
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(String::from("1abc2")), "12");
        assert_eq!(parse_line(String::from("pqr3stu8vwx")), "38");
        assert_eq!(parse_line(String::from("a1b2c3d4e5f")), "15");
        assert_eq!(parse_line(String::from("treb7uchet")), "77");
    }

    #[test]
    fn test_trebuchet() {
        assert_eq!(trebuchet(std::fs::read_to_string("src/test_trebuchet.txt").unwrap()), 142);
    }
}
