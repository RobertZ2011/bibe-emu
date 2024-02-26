use crate::memory::Memory;
use bibe_instr::jump::Instruction;

pub(super) fn execute<M: Memory>(s: &mut super::State<M>, i: &Instruction) -> crate::Result<()> {
	s.write_pc((i.imm as u32) << 2);
	Ok(())
}