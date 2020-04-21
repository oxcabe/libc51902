extern crate strum;

use strum_macros::EnumString;

/// Instruction opcode representation
#[derive(EnumString, Debug)]
pub enum Opcode {
    /// no-operation
    Nop,
    /// load immediate into register
    Li,
    /// unconditional jump
    Jmp,
    /// conditional jump (zero)
    Jz,
    /// conditional jump (not zero)
    Jnz,
    /// move data between regs
    Mov,
    /// bitwise negation
    Not,
    /// add two registers and store in dest
    Add,
    /// substract two registers and store in dest
    Sub,
    /// bitwise AND
    And,
    /// bitwise OR
    Or,
    /// call subroutine
    Call,
    /// return to stack address
    Ret,
    /// mem to reg
    Lw,
    /// reg to mem
    Sw,
    /// io to mem
    In,
    /// mem to io
    Out,
}

impl Opcode {
    // [u8] deallocates in parent scope
    pub fn to_binary<'a>(&self) -> &'a[u8] {
        use Opcode::*;
        match self {
            Nop     => &[0, 0, 0, 0, 0, 0],
            Li      => &[0, 0, 1, 0],
            Jmp     => &[0, 0, 0, 1, 0, 0],
            Jz      => &[0, 0, 0, 1, 0, 1],
            Jnz     => &[0, 0, 0, 1, 1, 0],
            Mov     => &[1, 0, 0, 0],
            Not     => &[1, 0, 0, 1],
            Add     => &[1, 0, 1, 0],
            Sub     => &[1, 0, 1, 1],
            And     => &[1, 1, 0, 0],
            Or      => &[1, 1, 0, 1],
            Call    => &[0, 0, 0, 0, 0, 1],
            Ret     => &[0, 0, 0, 0, 1, 0],
            Lw      => &[0, 0, 1, 1, 0, 0],
            Sw      => &[0, 0, 1, 1, 0, 1],
            In      => &[0, 0, 1, 1, 1, 0],
            Out     => &[0, 0, 1, 1, 1, 1],
        }
    }
}
