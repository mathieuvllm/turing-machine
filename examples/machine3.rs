use turing::machine::*;
use std::collections::HashMap;

fn main() {
    let initial = 1;
    let transitions = HashMap::from([
        ((1, '0'), (1, '1', Direction::Right)),
        ((1, '1'), (1, '0', Direction::Right)),
        ((1, '_'), (2, '_', Direction::Left)),
        ((2, '0'), (2, '1', Direction::Halt)),
        ((2, '1'), (2, '0', Direction::Left)),
        ((2, '_'), (2, '_', Direction::Halt))
    ]);
    let blank = '_';
    let mut tape = [blank; TAPE_SIZE];
    let index = TAPE_SIZE/2;
    tape[index] = '1';
    tape[index+1] = '1';
    tape[index+2] = '0';
    tape[index+3] = '0';
    let mut m = Machine::new(initial, transitions, blank, tape, index);
    m.execute(true);
}
