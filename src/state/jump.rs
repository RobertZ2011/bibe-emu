use crate::{memory::Memory, state::Execute};
use bibe_instr::jump::Instruction;

pub struct Jump;

impl<M> Execute<M> for Jump 
where
	M: Memory
{
    type I = Instruction;

    fn execute(s: &mut super::State<M>, i: &Self::I) -> crate::Result<()> {
        s.write_pc((i.imm as u32) << 2);
        Ok(())
    }
}