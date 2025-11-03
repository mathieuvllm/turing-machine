# Turing machine simulator
This library lets you create Turing machines and execute them.
The `Machine` structure has 5 fields:
- 1. initial : state in which the machine starts
- 2. transitions: key = (source state, symbol read), value = (destination state, symbol written, direction (right, left, halt))
- 3. blank: blank symbol
- 4. tape: array of symbols used as tape (leave blank characters at beginning/end of the array)
- 5. index: current symbol to be read

The size of the tape is defined in machine.rs and is called `TAPE_SIZE`. Its default value is 64.
To allow for enough room on both ends of the tape, the initial index should be `TAPE_SIZE/2` (unless you know what you are doing, this is merely a recommendation).

The main functions are `new()` and `execute(print_blank: bool)`.
`new()` allows you to create a machine, and `execute()` executes it and prints every step of the execution.
The `print_blank` parameter allows you to choose whether or not to print the blank characters at the beginning/end of the tape.

# Examples of Turing machines
## Machine 1
This machine has the alphabet {0, 1}, with 0 being its blank character,
and starts with 11 in the middle of the tape.
It basically duplicates the sequence of 1s, separating the two sequences by a 0.
For more info, see [https://fr.wikipedia.org/wiki/Machine_de_Turing#Doubler_le_nombre_de_'1'] (in French).

## Machine 2
This machine adds 1 to a number, using _ as its blank character. In this example, we use 27999 as the initial number.
