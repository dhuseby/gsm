# Generic Stack Machine

This crates implements a generic stack machine that will execute any client
defined set of instructions and track state for you. All classes are generic on
a type that implements the `Instruction` and `Clone` traits. The Machine takes
a Script made up of objects that implement those traits and when executing, the
Machine calls the `execute` function for each `Instruction` so that the Machine
state can change and other side effects may happen.

The source code file `test/simple.rs` demonstrates how a client would implement
`Instruction` objects for use in scripts.
