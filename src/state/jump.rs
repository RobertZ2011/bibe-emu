use crate::memory::Memory;
use bibe_instr::jump::Instruction;

use super::csr::CsrCollection;

pub(super) fn execute<M, C>(s: &mut super::State<M, C>, i: &Instruction) -> crate::Result<()>
where
	M: Memory,
	C: CsrCollection,
{
	let mut core = s.core.borrow_mut();
	core.write_pc((i.imm as u32) << 2);
	Ok(())
}