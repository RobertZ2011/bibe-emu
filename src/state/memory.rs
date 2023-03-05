/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_instr::{
	memory::{
		rr,
		ri,
		Instruction,
		OpType,
	}
};

use crate::Result;
use super::{
	Execute,
	State,
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

pub struct Memory;

impl Execute for Memory {
	type I = Instruction;

	fn execute(s: &mut State, instr: &Self::I) -> Result<()> {
		match instr {
			Instruction::Rr(i) => execute_rr(s, i),
			Instruction::Ri(i) => execute_ri(s, i),
		}
	}
}