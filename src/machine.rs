use std::collections::HashMap;
use std::collections::HashSet;

pub const TAPE_SIZE: usize = 64;

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Halt,
}

#[derive(Debug)]
pub struct Machine {
    states: HashSet<u32>,
    alphabet: HashSet<char>,
    initial: u32,
    transitions: HashMap<(u32, char), (u32, char, Direction)>,
    blank: char,
    tape: [char; TAPE_SIZE],
    index: usize,
}

impl Machine {
    /// states: all states of the Turing machine
    /// alphabet: characters that are writable on the tape
    /// initial: state in which the machine starts
    /// transitions: key = (source state, symbol read), value = (destination state, symbol written, direction (right, left, halt))
    /// blank: blank symbol
    /// tape: array of symbols used as tape (leave blank characters at beginning/end of the array)
    /// index: current symbol to be read
    pub fn new(
        states: HashSet<u32>,
        alphabet: HashSet<char>,
        initial: u32,
        transitions: HashMap<(u32, char), (u32, char, Direction)>,
        blank: char,
        tape: [char; TAPE_SIZE],
        index: usize,
    ) -> Self {
        Machine {
            states,
            alphabet,
            initial,
            transitions,
            blank,
            tape,
            index,
        }
    }

    /// Prints the tape of the machine, without the blank characters at the beginning and
    /// at the end
    pub fn print_tape(&self) {
        let mut start: usize = 0;

        for (index, c) in self.tape.iter().enumerate() {
            if *c != self.blank {
                start = index;
                break;
            }
        }

        let mut end: usize = start;

        for (index, c) in self.tape.iter().rev().enumerate() {
            if *c != self.blank {
                end = TAPE_SIZE - index - 1;
                break;
            }
        }

        for i in start..=end {
            print!("{}", self.tape[i]);
        }
        println!();
    }
}
