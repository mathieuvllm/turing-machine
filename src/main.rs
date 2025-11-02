mod machine;
use crate::machine::TAPE_SIZE;
use machine::Direction;
use machine::Machine;
use std::collections::HashMap;

fn main() {
    let initial = 1;
    let transitions = HashMap::from([
        ((1, '0'), (1, '0', Direction::Right)),
        ((1, '1'), (1, '1', Direction::Right)),
        ((1, '2'), (1, '2', Direction::Right)),
        ((1, '3'), (1, '3', Direction::Right)),
        ((1, '4'), (1, '4', Direction::Right)),
        ((1, '5'), (1, '5', Direction::Right)),
        ((1, '6'), (1, '6', Direction::Right)),
        ((1, '7'), (1, '7', Direction::Right)),
        ((1, '8'), (1, '8', Direction::Right)),
        ((1, '9'), (1, '9', Direction::Right)),
        ((1, '_'), (2, '_', Direction::Left)),
        ((2, '0'), (2, '1', Direction::Halt)),
        ((2, '1'), (2, '2', Direction::Halt)),
        ((2, '2'), (2, '3', Direction::Halt)),
        ((2, '3'), (2, '4', Direction::Halt)),
        ((2, '4'), (2, '5', Direction::Halt)),
        ((2, '5'), (2, '6', Direction::Halt)),
        ((2, '6'), (2, '7', Direction::Halt)),
        ((2, '7'), (2, '8', Direction::Halt)),
        ((2, '8'), (2, '9', Direction::Halt)),
        ((2, '9'), (2, '0', Direction::Left)),
        ((2, '_'), (2, '1', Direction::Halt)),
    ]);
    let blank = '_';
    let mut tape = [blank; TAPE_SIZE];
    let index = TAPE_SIZE / 2;
    tape[index] = '7';
    tape[index + 1] = '0';
    tape[index + 2] = '9';
    tape[index + 3] = '9';
    tape[index + 4] = '9';
    let mut m = Machine::new(initial, transitions, blank, tape, index);
    m.execute(true);
}
