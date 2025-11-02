use simply_colored::*;
use std::collections::HashMap;

pub const TAPE_SIZE: usize = 64;

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Halt,
}

#[derive(Debug)]
pub struct Machine {
    initial: u32,
    transitions: HashMap<(u32, char), (u32, char, Direction)>,
    blank: char,
    tape: [char; TAPE_SIZE],
    index: usize,
}

impl Machine {
    /// initial: state in which the machine starts
    /// transitions: key = (source state, symbol read), value = (destination state, symbol written, direction (right, left, halt))
    /// blank: blank symbol
    /// tape: array of symbols used as tape (leave blank characters at beginning/end of the array)
    /// index: current symbol to be read
    pub fn new(
        initial: u32,
        transitions: HashMap<(u32, char), (u32, char, Direction)>,
        blank: char,
        tape: [char; TAPE_SIZE],
        index: usize,
    ) -> Self {
        Machine {
            initial,
            transitions,
            blank,
            tape,
            index,
        }
    }

    /// Prints the tape of the machine, including the blank characters at the beginning and
    /// at the end
    pub fn print_tape(&self) {
        for i in 0..TAPE_SIZE {
            if i == self.index {
                print!("{UNDERLINE}{}{NO_UNDERLINE}", self.tape[i]);
            } else {
                print!("{}", self.tape[i]);
            }
        }
    }

    /// Prints the tape of the machine, without the blank characters at the beginning and
    /// at the end
    pub fn print_tape_no_blank(&self) {
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

        if start <= self.index && self.index <= end {
            for i in start..=end {
                if i == self.index {
                    print!("{UNDERLINE}{}{NO_UNDERLINE}", self.tape[i]);
                } else {
                    print!("{}", self.tape[i]);
                }
            }
        } else if self.index < start {
            for i in self.index..=end {
                if i == self.index {
                    print!("{UNDERLINE}{}{NO_UNDERLINE}", self.tape[i]);
                } else {
                    print!("{}", self.tape[i]);
                }
            }
        } else {
            for i in start..=self.index {
                if i == self.index {
                    print!("{UNDERLINE}{}{NO_UNDERLINE}", self.tape[i]);
                } else {
                    print!("{}", self.tape[i]);
                }
            }
        }
    }

    /// Track the execution of the machine
    pub fn execute(&mut self, print_blank: bool) {
        match print_blank {
            true => self.print_tape(),
            false => self.print_tape_no_blank(),
        }
        let mut running = true;
        let mut current_state = self.initial;
        println!(" | state {current_state}");

        while running {
            if let Some(value) = self
                .transitions
                .get(&(current_state, self.tape[self.index]))
            {
                current_state = value.0;
                self.tape[self.index] = value.1;
                match value.2 {
                    Direction::Halt => running = false,
                    Direction::Right => self.index += 1,
                    Direction::Left => self.index -= 1,
                }
                match print_blank {
                    true => self.print_tape(),
                    false => self.print_tape_no_blank(),
                }
                println!(
                    " | {} state {current_state}",
                    match value.2 {
                        Direction::Right => "->",
                        Direction::Left => "<-",
                        Direction::Halt => "HALT",
                    }
                );
            }
        }
    }
}
