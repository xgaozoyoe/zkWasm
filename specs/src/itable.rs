use super::mtable::VarType;
use num_bigint::BigUint;

pub enum OpcodeClass {
    LocalGet = 1,
    Const,
    Drop,
    Return,
}

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    LocalGet { offset: u64 },
    Const { vtype: VarType, value: u64 },
    Drop,
    Return,
}

pub const OPCODE_CLASS_SHIFT: usize = 96;
pub const OPCODE_CONST_VTYPE_SHIFT: usize = 64;

impl Into<BigUint> for Opcode {
    fn into(self) -> BigUint {
        let bn = match self {
            Opcode::LocalGet { offset } => {
                (BigUint::from(OpcodeClass::LocalGet as u64) << OPCODE_CLASS_SHIFT) + offset
            }
            Opcode::Const { vtype, value } => {
                (BigUint::from(OpcodeClass::Const as u64) << OPCODE_CLASS_SHIFT)
                    + (BigUint::from(vtype as u64) << OPCODE_CONST_VTYPE_SHIFT)
                    + value
            }
            Opcode::Drop => BigUint::from(OpcodeClass::Drop as u64) << OPCODE_CLASS_SHIFT,
            Opcode::Return => BigUint::from(OpcodeClass::Return as u64) << OPCODE_CLASS_SHIFT,
        };
        assert!(bn < BigUint::from(1u64) << 128usize);
        bn
    }
}

#[derive(Clone)]
pub struct InstructionTableEntry {
    pub moid: u16,
    pub mmid: u16,
    pub fid: u16,
    pub bid: u16,
    pub iid: u16,
    pub opcode: Opcode,
}