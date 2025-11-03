use std::collections::HashMap;
use turing::machine::*;

fn main() {
    let initial = 1;
    let transitions = HashMap::from([
        ((1, '0'), (1, '0', Direction::Halt)),
        ((1, '1'), (2, '0', Direction::Right)),
        ((2, '0'), (3, '0', Direction::Right)),
        ((2, '1'), (2, '1', Direction::Right)),
        ((3, '0'), (4, '1', Direction::Left)),
        ((3, '1'), (3, '1', Direction::Right)),
        ((4, '0'), (5, '0', Direction::Left)),
        ((4, '1'), (4, '1', Direction::Left)),
        ((5, '0'), (1, '1', Direction::Right)),
        ((5, '1'), (5, '1', Direction::Left)),
    ]);
    let blank = '0';
    let mut tape = [blank; TAPE_SIZE];
    let index = TAPE_SIZE / 2;
    tape[index] = '1';
    tape[index + 1] = '1';
    let mut m = Machine::new(initial, transitions, blank, tape, index);
    m.execute(true);
}
