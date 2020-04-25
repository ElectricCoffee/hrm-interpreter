#![allow(unused)]
type Loc = usize;
type Line = usize;
type Value = i64;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Instruction {
    Inbox,
    Outbox,
    CopyFrom(Loc),
    CopyTo(Loc),
    Add(Loc),
    Sub(Loc),
    Inc(Loc), // NB: bumping also picks up the bumped value
    Dec(Loc),
    Jump(Line),
    JumpIfZero(Line),
    JumpIfNeg(Line),
    // Special halt instruction required because the vector never gets emptied.
    // There's no other way of knowing when the program has reached its conclusion.
    __Halt,
}

/// Decides what happens next with the program.
/// This was added to make halting when taking from an empy inbox nicer.
/// i.e. I didn't want any if-statements in the body of the big match expression.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Next {
    Continue,
    Jump(usize),
    Halt,
}

// Hrm is short for Human Resource Machine
#[derive(Default, Debug)]
struct Hrm {
    inbox: Vec<Value>,
    outbox: Vec<Value>,
    current_value: Value,
    instruction_ptr: usize,
    instructions: Vec<Instruction>,
    memory: [Value; 16],
}

impl Hrm {
    /// Creates a new Resource Machine with the given instructions.
    /// Also pushes __Halt to the end
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut this = Hrm::default();
        this.instructions = instructions;
        this.instructions.push(Instruction::__Halt);
        this
    }

    /// Grab the next value in the inbox, halt if empty.
    fn inbox(&mut self) -> Next {
        if let Some(value) = self.inbox.pop() {
            self.current_value = value;
            Next::Continue
        } else {
            Next::Halt
        }
    }

    /// Deposit the currently held value into the outbox
    fn outbox(&mut self) -> Next {
        self.outbox.push(self.current_value);

        Next::Continue
    }

    /// Makes the currently held value a copy of a location in memory
    fn copy_from(&mut self, location: Loc) -> Next {
        self.current_value = self.memory[location];

        Next::Continue
    }

    /// Copies the currently held value to a location in memory
    fn copy_to(&mut self, location: Loc) -> Next {
        self.memory[location] = self.current_value;

        Next::Continue
    }

    /// Adds the a value in memory to the currently held value
    fn add(&mut self, location: Loc) -> Next {
        self.current_value += self.memory[location];

        Next::Continue
    }

    /// Subtracts a value in memory from the currently held value
    fn sub(&mut self, location: Loc) -> Next {
        self.current_value -= self.memory[location];

        Next::Continue
    }

    /// Increments a value in memory and copies it
    /// NB: This is known as Bump+ in the game
    fn inc(&mut self, location: Loc) -> Next {
        self.memory[location] += 1;
        self.copy_from(location);

        Next::Continue
    }

    /// Decrements a value in memory and copies it
    /// NB: This is known as Bump- in the game
    fn dec(&mut self, location: Loc) -> Next {
        self.memory[location] -= 1;
        self.copy_from(location);

        Next::Continue
    }

    /// Jumps to a line number
    /// NB: Line numbers are 1-indexed
    fn jump(&mut self, line: Line) -> Next {
        assert!(self.current_value > 0);
        self.instruction_ptr = line;

        Next::Jump(line)
    }

    /// Jumps to a line number if the currently held value is zero
    /// NB: Line numbers are 1-indexed
    fn jump_if_zero(&mut self, line: Line) -> Next {
        if self.current_value == 0 {
            self.jump(line)
        } else {
            Next::Continue
        }
    }

    /// Jumps to a line number if the currently held value is negative
    /// NB: Line numbers are 1-indexed
    fn jump_if_neg(&mut self, line: Line) -> Next {
        if self.current_value < 0 {
            self.jump(line)
        } else {
            Next::Continue
        }
    }

    /// Runs the program based on the values in the `inbox`.
    fn run(&mut self, inbox: Vec<Value>) -> Vec<Value> {
        self.inbox = inbox;
        use Instruction::*;

        loop {
            let current_instruction = self.instructions[self.instruction_ptr];

            let behaviour = match current_instruction {
                Inbox => self.inbox(),
                Outbox => self.outbox(),
                CopyFrom(location) => self.copy_from(location),
                CopyTo(location) => self.copy_to(location),
                Add(location) => self.add(location),
                Sub(location) => self.sub(location),
                Inc(location) => self.inc(location),
                Dec(location) => self.dec(location),
                Jump(line) => self.jump(line - 1),
                JumpIfZero(line) => self.jump_if_zero(line),
                JumpIfNeg(line) => self.jump_if_neg(line),
                __Halt => Next::Halt,
            };

            match behaviour {
                Next::Continue => self.instruction_ptr += 1,
                Next::Jump(loc) => self.instruction_ptr = loc,
                Next::Halt => break,
            }
        }

        self.outbox.clone()
    }
}

fn main() {
    use Instruction::*;
    // my solution from Level 16: "Absolute Positivity"
    let instructions = vec![
        Inbox,
        JumpIfNeg(4),
        Jump(7),
        CopyTo(0),
        Sub(0),
        Sub(0),
        Outbox,
        Jump(1),
    ];

    let inbox = vec![3, 6, -2, 0, 7, -9, 7];

    let mut hrm = Hrm::new(instructions);
    let result = hrm.run(inbox);

    println!("{:?}", result);
}
