pub const MAX_OPCODE_LENGTH: usize = 6;

#[derive(Debug)]
enum JumpOpcode {
    Jmp,
    Jz,
}

#[derive(Debug)]
enum AluOpcode {
    Mov,
    Not,
    Add,
    Sub,
    And,
    Or,
    NegH,
    NegL
}

#[derive(Debug)]
enum OpcodeType {
    Nop,
    Jump(JumpOpcode),
    Alu(AluOpcode),
    Li,
}

#[derive(Debug)]
pub struct Opcode {
    op_size: usize,
    op: [u8; MAX_OPCODE_LENGTH],
    op_type: OpcodeType,
}

fn __build_opcode(partial_opcode: &[u8],
                  partial_size: usize) -> [u8; MAX_OPCODE_LENGTH] {
    let mut ret: [u8; 6] = [0; 6];
    let ret_slice = &mut ret[..partial_size];

    for (idx, val) in partial_opcode.iter().enumerate() {
        ret_slice[idx] = *val;
    }

    ret
}

impl Opcode {

    fn new(op_type: OpcodeType) -> Opcode {
        let op_size: usize;
        let op: [u8; MAX_OPCODE_LENGTH];
        let type_borrow = &op_type;

        match type_borrow {
            OpcodeType::Nop => {
                op_size = 6;
                let nop_opcode: [u8; 6] = [0; 6];
                op = __build_opcode(&nop_opcode, op_size);
            },
            OpcodeType::Jump(jump_opcode) => {
                op_size = 6;
                match jump_opcode {
                    JumpOpcode::Jmp => {
                        op = [0, 0, 0, 1, 0, 0];
                    },
                    JumpOpcode::Jz => {
                        op = [0, 0, 0, 1, 0, 1];
                    },
                }
            },
            OpcodeType::Alu(alu_opcode) => {
                op_size = 4;
                match alu_opcode {
                    AluOpcode::Mov => {
                        op = [1, 0, 0, 0, 0, 0];
                    },
                    AluOpcode::Not => {
                        op = [1, 0, 0, 1, 0, 0];
                    },
                    AluOpcode::Add => {
                        op = [1, 0, 1, 0, 0, 0];
                    },
                    AluOpcode::Sub => {
                        op = [1, 0, 1, 1, 0, 0];
                    },
                    AluOpcode::And => {
                        op = [1, 1, 0, 0, 0, 0];
                    },
                    AluOpcode::Or => {
                        op = [1, 1, 0, 1, 0, 0];
                    },
                    AluOpcode::NegH => {
                        op = [1, 1, 1, 0, 0, 0];
                    },
                    AluOpcode::NegL => {
                        op = [1, 1, 1, 1, 0, 0];
                    },
                }
            },
            OpcodeType::Li => {
                op_size = 4;
                op = [0, 0, 1, 0, 0, 0];
            }
        }
        Opcode { op_size, op, op_type }
    }
}

lazy_static!{
    pub static ref OPCODES: [Opcode; 12] = [
        Opcode::new(OpcodeType::Nop),
        Opcode::new(OpcodeType::Jump(JumpOpcode::Jmp)),
        Opcode::new(OpcodeType::Jump(JumpOpcode::Jz)),
        Opcode::new(OpcodeType::Alu(AluOpcode::Mov)),
        Opcode::new(OpcodeType::Alu(AluOpcode::Not)),
        Opcode::new(OpcodeType::Alu(AluOpcode::Add)),
        Opcode::new(OpcodeType::Alu(AluOpcode::Sub)),
        Opcode::new(OpcodeType::Alu(AluOpcode::And)),
        Opcode::new(OpcodeType::Alu(AluOpcode::Or)),
        Opcode::new(OpcodeType::Alu(AluOpcode::NegH)),
        Opcode::new(OpcodeType::Alu(AluOpcode::NegL)),
        Opcode::new(OpcodeType::Li),
    ];
}
