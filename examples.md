# Examples of Turing machines
## Machine 1
This machine has the alphabet {0, 1}, with 0 being its blank character,
and starts with 11 in the middle of the tape.
It basically duplicates the sequence of 1s, separating the two sequences by a 0.
For more info, see [https://fr.wikipedia.org/wiki/Machine_de_Turing#Doubler_le_nombre_de_'1'] (in French).

```Rust
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
```

## Machine 2
This machine adds 1 to a number, using _ as its blank character. In this example, we use 27999 as the initial number.
```Rust

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
    tape[index] = '2';
    tape[index + 1] = '7';
    tape[index + 2] = '9';
    tape[index + 3] = '9';
    tape[index + 4] = '9';
    let mut m = Machine::new(initial, transitions, blank, tape, index);
```
