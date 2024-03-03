pub mod part_1 {
    pub struct Code {
        position: [usize; 2],  //starting index and length
        
    }

    pub fn gear_ratios(input: String) ->  u32 {
        //let lines_iter = 
        8
    }

    struct Number {
        value: usize,
        head: usize,
        span: usize,
        line: usize,
        adjacents: Vec<char>,
    }
    
    struct Multi {
        content: Vec<String>,
        numbers: Vec<Number>,
    }
    
    impl Multi {
        fn new(content: Vec<String>) -> Multi {
            Multi {
                content,
                numbers: vec![],
            }
        } 
    
        fn get_numbers(&self) {
            for (index, line) in self.content.iter().enumerate() {
                for character in line.chars().enumerate() {
                    
                }
            }
        }
    
    }


}

pub mod fsa {
    pub struct Fsa {
        edges: Vec<u8>,
        state: State,
    }

    pub enum State {
        Start,
        F,
        G,
    }

    impl Fsa {
        pub fn new() -> Fsa {
            Fsa {
                edges: vec![],
                state: State::Start,
            }
        }

        pub fn edge_detector(&self, input: String) -> Vec<u8> {
            let input: Vec<char> = input.chars().collect();
            for (index, i) in input.iter().en() {
                if index == 0 {
                    f();
                } else {

                }
            }
            //for first element in the sequeunce only
            fn start() {
                self.edges.append
            }
            //called if previous element is a digit
            fn f() {

            }
            //called if previous element is not a digit
            fn g() {

            }
        }
    }
}


