use crate::state::Execute;
use bibe_instr::jump::Instruction;

pub struct Jump;

impl Execute for Jump {
    type I = Instruction;

    fn execute(s: &mut super::State, i: &Self::I) -> crate::Result<()> {
        s.write_pc((i.imm as u32) << 2);
        Ok(())
    }
}