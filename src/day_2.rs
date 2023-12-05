pub mod part_1 {
    use std::collections::HashMap;

    pub fn parse_line(input: String) -> u32 {
        let mut possibility: bool = true;
        let colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        let cln = input.find(|i| i == ':').unwrap();
        let throws = &input[cln + 1..input.len()];
        let sets = throws.split(';').collect::<Vec<&str>>();
        for set in sets {
            let balls = set.split(',');
            for ball in balls {
                for (color, _count) in colors.iter() {
                    if ball.contains(color) {
                        if ball[1..3].trim().parse::<u32>().unwrap() > *colors.get(color).unwrap() {
                            possibility = false;
                        }
                    }
                }
            }
        }
        match possibility {
            true => input[5..cln].parse::<u32>().unwrap(),
            false => 0,
        }
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line(String::from(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            )),
            1
        );
        assert_eq!(
            parse_line(String::from(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            )),
            2
        );
        assert_eq!(
            parse_line(String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )),
            0
        );
        assert_eq!(
            parse_line(String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            )),
            0
        );
        assert_eq!(
            parse_line(String::from(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            5
        );
    }

    pub fn cube_conundrum(content: String) -> u32 {
        let mut sum: u32 = 0;
        for line in content.lines() {
            sum += parse_line(line.to_string());
        }
        sum
    }

    #[test]
    fn test_cube_conundrum() {
        assert_eq!(
            cube_conundrum(
                std::fs::read_to_string("src/txts/testxt/test_cube_conundrum.txt").unwrap()
            ),
            8
        );
    }
}

pub mod part_2 {}
