mod aliases;
mod hrm;
mod instruction;

use hrm::*;
use instruction::Instruction;
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
