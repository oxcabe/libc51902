/// Instruction opcode representation
pub enum Opcode {
    /// no-operation
    Nop,
    /// unconditional jump
    Jmp,
    /// conditional jump
    Jz,
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
    NegH,
    NegL,
    /// load immediate into register
    Li,     
}

impl Opcode {
    // [u8] deallocates in parent scope
    pub fn to_binary<'a>(&self) -> &'a[u8] {
        use Opcode::*;
        match self {
            Nop     => &[0, 0, 0, 0, 0, 0],
            Jmp     => &[0, 0, 0, 1, 0, 0],
            Jz      => &[0, 0, 0, 1, 0, 1],
            Mov     => &[1, 0, 0, 0],
            Not     => &[1, 0, 0, 1],
            Add     => &[1, 0, 1, 0],
            Sub     => &[1, 0, 1, 1],
            And     => &[1, 1, 0, 0],
            Or      => &[1, 1, 0, 1],
            NegH    => &[1, 1, 1, 0],
            NegL    => &[1, 1, 1, 1],
            Li      => &[0, 0, 1, 0],
        }
    }
}
