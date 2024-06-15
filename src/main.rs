use std::{env, fs, process::exit};

const CELL_SIZE_LIMIT: u32 = 255;
const ARRAY_SIZE_LIMIT: usize = 30000;

// the internal memory
struct Memory {
    bytearray: [u32; ARRAY_SIZE_LIMIT],
    idx: usize,
}

impl Memory {
    // create a new array
    fn new() -> Memory {
        let array = Memory {
            bytearray: [0; ARRAY_SIZE_LIMIT],
            idx: 0,
        };
        array
    }

    // keep the index within range
    fn keep_range(&mut self) {
        if self.idx >= ARRAY_SIZE_LIMIT {
            self.idx = 0;
        }
    }

    // move the array pointer left one byte, and wraps around
    fn move_left(&mut self) {
        if self.idx == 0 {
            self.idx = ARRAY_SIZE_LIMIT - 1;
        } else {
            self.idx -= 1;
        }
        self.keep_range()
    }

    // move the array pointer right one byte, wraps around
    fn move_right(&mut self) {
        self.idx += 1;
        self.keep_range()
    }

    // accept one character of input
    fn accept_in(&mut self, chr: u8) {
        self.bytearray[self.idx] = chr as u32;
    }

    // provide the value at the array pointer
    fn give_out(&mut self) -> u32 {
        self.bytearray[self.idx]
    }

    // increment the value at pointer
    fn increment(&mut self) {
        if self.bytearray[self.idx] >= CELL_SIZE_LIMIT {
            self.bytearray[self.idx] = 0;
        } else {
            self.bytearray[self.idx] += 1;
        }
    }

    // decrement the value at pointer
    fn decrement(&mut self) {
        if self.bytearray[self.idx] == 0 {
            self.bytearray[self.idx] = CELL_SIZE_LIMIT;
        } else {
            self.bytearray[self.idx] -= 1;
        }
    }

    // get the current value at pointer
    fn get_value(&mut self) -> u32 {
        self.bytearray[self.idx]
    }
}

// list of all operations available to perform (including comment, which is ignored)
#[derive(Debug)]
enum Operations {
    Add,
    Subtract,
    MoveLeft,
    MoveRight,
    Input,
    Output,
    BracketLeft,
    BracketRight,
    Comment(char),
}

// the inner state of the turing machine executing the program
struct InnerState {
    operations: Vec<Operations>,
    idx: usize,
    memory: Memory,
    input_str: Vec<u8>,
    input_idx: usize,
}

impl InnerState {
    fn new(ops: Vec<char>, input_str: String) -> InnerState {
        let operations: Vec<Operations> = ops
            .into_iter()
            .map(|c| match c {
                '+' => Operations::Add,
                '-' => Operations::Subtract,
                '>' => Operations::MoveRight,
                '<' => Operations::MoveLeft,
                '.' => Operations::Output,
                ',' => Operations::Input,
                '[' => Operations::BracketLeft,
                ']' => Operations::BracketRight,
                _ => Operations::Comment(c),
            })
            .collect();

        let state = InnerState {
            operations,
            idx: 0,
            memory: Memory::new(),
            input_str: Vec::from(input_str.as_bytes()),
            input_idx: 0,
        };
        state
    }

    // get the idx of the next brace
    fn get_next_rbrack(&self) -> usize {
        let mut idx2 = self.idx + 1;
        let mut othercount = 0; // the count of non relavent braces
        loop {
            match self.operations[idx2] {
                Operations::BracketRight => {
                    if othercount == 0 {
                        break;
                    } else {
                        othercount -= 1;
                    }
                }
                Operations::BracketLeft => {
                    othercount += 1;
                }
                _ => {}
            }
            idx2 += 1;
        }
        idx2
    }

    // get location of previous lbrace
    fn get_prev_lbrack(&self) -> usize {
        let mut idx2 = self.idx - 1;
        let mut othercount = 0;
        loop {
            match self.operations[idx2] {
                Operations::BracketLeft => {
                    if othercount == 0 {
                        break;
                    } else {
                        othercount -= 1;
                    }
                }
                Operations::BracketRight => {
                    othercount += 1;
                }
                _ => {}
            }
            idx2 -= 1;
        }
        idx2
    }

    // actually interpret the program
    fn execute(&mut self) {
        let idx2 = self.idx;
        let oper = &self.operations[idx2];
        // println!("Running operation {:?} at location {}", oper, idx2);
        match oper {
            Operations::Add => self.memory.increment(),
            Operations::Subtract => self.memory.decrement(),
            Operations::MoveLeft => self.memory.move_left(),
            Operations::MoveRight => self.memory.move_right(),
            Operations::Input => {
                if self.input_idx >= self.input_str.len() {
                    self.memory.accept_in(0 as u8); // zero-terminate
                } else {
                    self.memory.accept_in(self.input_str[self.input_idx] as u8);
                }
                self.input_idx += 1;
            }
            Operations::Output => print!("{}", self.memory.give_out() as u8 as char),
            Operations::BracketLeft => {
                // if zero, then directly skip the block between `[` and `]`
                if self.memory.get_value() == 0 {
                    self.idx = self.get_next_rbrack();
                }
            }
            Operations::BracketRight => {
                // if nonzero, then jump back
                if self.memory.get_value() != 0 {
                    self.idx = self.get_prev_lbrack();
                }
            }
            Operations::Comment(_e) => {}
        }
        self.idx += 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [filename]", args[0]);
        exit(1);
    }
    let contents = fs::read_to_string(&args[1]).expect("Unable to read file!");
    let mut parsed = contents.trim().split("!").collect::<Vec<&str>>();

    if parsed.len() < 2 {
        parsed.push("");
    }
    if parsed.len() > 2 {
        println!("Please use only one `!` in the input file!");
        exit(1);
    }

    let mut state = InnerState::new(
        parsed[0].to_string().chars().collect(),
        parsed[1].to_string(),
    );
    while state.idx < parsed[0].len() {
        state.execute();
    }
    println!("");
}
