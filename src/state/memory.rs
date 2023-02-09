use bibe_instr::memory::{
	Instruction,
	OpType,
};

use crate::{
	state::State,
	Result
};

pub fn execute(s: &mut State, instr: &Instruction) -> Result<()> {
	let (kind, width) = instr.op.parts();
	let mut addr = (s.read_reg(instr.src) << instr.shift) as i32;
	addr += instr.immediate as i32;

	match kind {
		OpType::Load => {
			let v = s.mem().read(addr as u32, width)?;
			s.write_reg(instr.dest, v);
		},
		OpType::Store => {
			let v = s.read_reg(instr.dest);
			s.mem_mut().write(addr as u32, width, v)?;
		},
	};
	Ok(())
}