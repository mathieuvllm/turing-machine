# Examples of Turing machines
## Machine 1
This machine has the alphabet {0, 1}, with 0 being its blank character,
and starts with 11 in the middle of the tape.
It basically duplicated the sequence of 1s, separating the two sequences by a 0.
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
