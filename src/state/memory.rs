use bibe_instr::{
	memory::{
		rr,
		ri,
		Instruction,
		OpType,
	},
	Width
};

use crate::{
	state::{
		Memory,
		State,
	},
	Result
};

fn execute_rr(s: &mut State, instr: &rr::Instruction) -> Result<()> {
	let (op, width) = instr.op;
	let rs = s.read_reg(instr.rs);
	let rq = s.read_reg(instr.rq);
	let addr = rs + (rq << instr.shift);
	match op {
		OpType::Load => {
			let value = s.mem().read(addr, width)?;
			s.write_reg(instr.rd, value);
		},
		OpType::Store => {
			let value = s.read_reg(instr.rd);
			s.mem_mut().write(addr, width, value)?;
		},
	}
	Ok(())
}

fn execute_ri(s: &mut State, instr: &ri::Instruction) -> Result<()> {
	let (op, width) = instr.op;
	let rs = s.read_reg(instr.rs);
	let addr = rs.wrapping_add(instr.imm as u32);
	match op {
		OpType::Load => {
			let value = s.mem().read(addr, width)?;
			s.write_reg(instr.rd, value);
		},
		OpType::Store => {
			let value = s.read_reg(instr.rd);
			s.mem_mut().write(addr, width, value)?;
		}
	}
	Ok(())
}

pub fn execute(s: &mut State, instr: &Instruction) -> Result<()> {
	match instr {
		Instruction::Rr(i) => execute_rr(s, i),
		Instruction::Ri(i) => execute_ri(s, i),
	}
}