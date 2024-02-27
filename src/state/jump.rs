use crate::{memory::Memory, target::Target};
use bibe_instr::jump::Instruction;

use super::csr::CsrCollection;

pub(super) fn execute<T, M, C>(s: &mut super::State<T, M, C>, i: &Instruction) -> crate::Result<()>
where
	T: Target,
	M: Memory,
	C: CsrCollection,
{
	let mut core = s.core.borrow_mut();
	core.write_pc((i.imm as u32) << 2);
	Ok(())
}