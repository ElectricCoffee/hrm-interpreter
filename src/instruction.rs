#![allow(unused)]
use crate::aliases::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Instruction {
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
