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
                    if ball.contains(color)
                        && ball[1..3].trim().parse::<u32>().unwrap() > *colors.get(color).unwrap()
                    {
                        possibility = false;
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

pub mod part_2 {
    use std::fmt;
    #[derive(Debug, PartialEq)]
    pub struct Game {
        pub green: u32,
        pub red: u32,
        pub blue: u32,
    }

    impl Game {
        pub fn power(&self) -> u32 {
            self.red * self.green * self.blue
        }
    }

    impl fmt::Display for Game {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "this game requires at minimum: {} green balls, {} red balls, {} blue balls",
                self.green, self.red, self.blue
            )
        }
    }

    impl From<&str> for Game {
        fn from(items: &str) -> Game {
            let items: Vec<&str> = items.split_whitespace().collect();
            let mut fields: (u32, u32, u32) = (0, 0, 0);
            for (index, item) in items.iter().enumerate() {
                if let Ok(u) = item.parse::<u32>() {
                    if items.get(index + 1).unwrap().contains("green") && u > fields.0 {
                        fields.0 = u;
                    } else if items.get(index + 1).unwrap().contains("red") && u > fields.1 {
                        fields.1 = u;
                    } else if items.get(index + 1).unwrap().contains("blue") && u > fields.2 {
                        fields.2 = u;
                    } //that entire match arm is really fucking stupid
                } //i'm sure there's a thousand ways to do this but better
            }
            Game {
                green: fields.0,
                red: fields.1,
                blue: fields.2,
            }
        }
    } //holy fucking did i write all of that and it worked first try without testing it in between even once ??

    #[test]
    fn test_game() {
        assert_eq!(
            Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                green: 2,
                red: 4,
                blue: 6
            }
        );
        assert_eq!(
            Game::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Game {
                green: 3,
                red: 1,
                blue: 4
            }
        );
        assert_eq!(
            Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            Game {
                green: 13,
                red: 20,
                blue: 6
            }
        );
        assert_eq!(
            Game::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            Game {
                green: 3,
                red: 14,
                blue: 15
            }
        );
        assert_eq!(
            Game::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Game {
                green: 3,
                red: 6,
                blue: 2
            }
        );
    }

    pub fn cube_conundrum_2(content: String) -> u32 {
        let mut sum: u32 = 0;
        for line in content.lines() {
            sum += Game::from(line).power();
        }
        sum
    }

    #[test]
    fn test_cube_conundrum_2() {
        assert_eq!(
            cube_conundrum_2(
                std::fs::read_to_string("src/txts/testxt/test_cube_conundrum.txt").unwrap()
            ),
            2286
        );
    }
}
