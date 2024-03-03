pub mod part_1 {

    #[derive(Debug)]
    struct Number {
        value: usize,
        adjacents: Vec<char>,
    }

    impl Number {
        fn new(value: usize, adjacents: Vec<char>) -> Number {
            Number { value, adjacents }
        }
    }

    pub struct Multi {
        content: String,
        numbers: Vec<Number>,
    }

    impl Multi {
        pub fn new(content: String) -> Multi {
            Multi {
                content,
                numbers: vec![],
            }
        }

        pub fn scan_edges(&mut self) {
            let lines_vector = &self
                .content
                .lines()
                .map(|x| String::from(".") + x + ".")
                .collect::<Vec<String>>();
            for (line_number, line) in (self.content).lines().enumerate() {
                /*
                "you don't have to solve every problem by creating a
                new abstraction. not every problem is worth solving."
                ~ The Primeagen
                */
                let neo_line = String::from(".") + line + ".";
                let fsa = fsa::Fsa::new(&neo_line);
                let mut construction_values: Vec<usize> = Vec::new();
                let mut indexes: Vec<usize> = Vec::new();

                for (index, value) in fsa.edges.iter().enumerate() {
                    if *value == 1 {
                        indexes.push(index);
                        // . . . . 2 3 . . 1 2  3  .  .  2  9  3  .  .  2  .  .
                        // 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20
                        // 0,0,0,0,1,0,1,0,1,0,0 ,1 ,0 ,1 ,0 ,0 ,1 ,0 ,1 ,1 ,0
                        // 4-5 8-10 13-15 18
                    }
                }
                if !fsa.edges.contains(&1) {
                    continue;
                } else {
                    let v_indexes_pairs = {
                        let mut vv: Vec<Vec<usize>> = Vec::new();
                        while !indexes.is_empty() {
                            let drained = indexes.drain(..=1).collect::<Vec<_>>();
                            vv.push(drained);
                        }
                        vv
                    };

                    for pair in &v_indexes_pairs {
                        let a = &neo_line[*pair.first().expect("lower index out of range")
                            ..*pair.get(1).expect("upper index out of range")]
                            .parse::<usize>()
                            .expect("{a} is not a number");
                        construction_values.push(*a);
                    }

                    let mut actual_indexs_pairs: Vec<Vec<usize>> = vec![];

                    for sub_vec in &v_indexes_pairs {
                        actual_indexs_pairs.push(sub_vec.iter().map(|x| x - 1).collect())
                    }

                    let neo_line = neo_line.chars().collect::<Vec<char>>();
                    let mut adjacents: Vec<Vec<char>> = vec![];

                    for (index, value) in construction_values.iter().enumerate() {
                        let mut v: Vec<char> = vec![];

                        v.push(neo_line[(v_indexes_pairs[index][0]) - 1]);
                        v.push(neo_line[v_indexes_pairs[index][1]]);

                        if line_number != 0 {
                            match lines_vector.get(line_number - 1) {
                                None => continue,
                                Some(t) => {
                                    v.append(
                                        &mut (t[(v_indexes_pairs[index][0]) - 1
                                            ..(v_indexes_pairs[index][1]) + 1])
                                            .chars()
                                            .collect::<Vec<char>>(),
                                    );
                                }
                            };
                        }

                        if line_number != lines_vector.len() - 1 {
                            match lines_vector.get(line_number + 1) {
                                None => continue,
                                Some(t) => {
                                    v.append(
                                        &mut (t[(v_indexes_pairs[index][0]) - 1
                                            ..(v_indexes_pairs[index][1]) + 1])
                                            .chars()
                                            .collect::<Vec<char>>(),
                                    );
                                }
                            };
                        }

                        adjacents.push(v);
                        self.numbers
                            .push(Number::new(*value, adjacents[index].clone()));
                    }
                }
            }
        }

        pub fn gear_ratios(&mut self) -> usize {
            let mut sum: usize = 0;

            for number in self.numbers.iter_mut() {
                number.adjacents.retain(|x| *x != '.');

                if !number.adjacents.is_empty() {
                    sum += number.value;
                }
            }
            sum
        }
    }
    pub mod fsa {
        enum State {
            Start,
            Nm,
            Chr,
        }
        pub struct Fsa<'a> {
            content: &'a str,
            pub edges: Vec<u8>,
            state: State,
        }

        impl Fsa<'_> {
            pub fn new(content: &str) -> Fsa {
                let mut place_holder = Fsa {
                    content,
                    edges: vec![],
                    state: State::Start,
                };

                place_holder.edge_detect();
                place_holder
            }

            fn edge_detect(&mut self) {
                let chars_iter = self.content.chars();

                for c in chars_iter {
                    match self.state {
                        State::Start => {
                            if c.is_ascii_digit() {
                                self.state = State::Nm;
                                self.edges.push(0);
                            } else {
                                self.state = State::Chr;
                                self.edges.push(0);
                            }
                        }
                        State::Nm => {
                            if c.is_ascii_digit() {
                                self.edges.push(0);
                            } else {
                                self.edges.push(1);
                                self.state = State::Chr;
                            }
                        }
                        State::Chr => {
                            if c.is_ascii_digit() {
                                self.state = State::Nm;
                                self.edges.push(1);
                            } else {
                                self.edges.push(0);
                            }
                        }
                    }
                }
            }
        }

        #[test]
        fn lol() {
            let f = Fsa::new("....23..123..293..2..");
            assert_eq!(
                vec![0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0],
                f.edges
            );
        }
    }

    #[test]
    fn test_gear_ratios() {
        let mut multi =
            Multi::new(std::fs::read_to_string("src/txts/testxt/test_gear_ratios.txt").unwrap());
        multi.scan_edges();
        assert_eq!(4361, multi.gear_ratios());
    }
}
