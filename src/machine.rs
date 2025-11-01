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
}
