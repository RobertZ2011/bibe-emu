/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_instr::{
	memory::{
		rr,
		ri,
		Instruction,
	},
	LoadStore,
};

use crate::{
	memory::Memory as MemTrait,
	Result,
};

use super::{
	Execute,
	State,
	shift
};

fn execute_rr<M>(s: &mut State<M>, instr: &rr::Instruction) -> Result<()>
where
	M: MemTrait
{
	let rs = s.read_reg(instr.rs);
	let rq = s.read_reg(instr.rq);
	let addr = rs + shift(&instr.shift, rq);
	match instr.op.op {
		LoadStore::Load => {
			let value = s.read(addr, instr.op.width)?;
			s.write_reg(instr.rd, value);
		},
		LoadStore::Store => {
			let value = s.read_reg(instr.rd);
			s.write(addr, instr.op.width, value)?;
		},
	}
	Ok(())
}

fn execute_ri<M>(s: &mut State<M>, instr: &ri::Instruction) -> Result<()> 
where
	M: MemTrait
{
	let rs = s.read_reg(instr.rs);
	let addr = rs.wrapping_add(instr.imm as u32);
	match instr.op.op {
		LoadStore::Load => {
			let value = s.read(addr, instr.op.width)?;
			s.write_reg(instr.rd, value);
		},
		LoadStore::Store => {
			let value = s.read_reg(instr.rd);
			s.write(addr, instr.op.width, value)?;
		}
	}
	Ok(())
}

pub struct Memory;

impl<M> Execute<M> for Memory
where
	M: MemTrait
{
	type I = Instruction;

	fn execute(s: &mut State<M>, instr: &Self::I) -> Result<()> {
		match instr {
			Instruction::Rr(i) => execute_rr(s, i),
			Instruction::Ri(i) => execute_ri(s, i),
		}
	}
}