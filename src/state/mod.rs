use bibe_instr::{
	BinOp,
	Encode,
	Instruction,
	Register,
};

use log::debug;
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

mod rrr;
mod rri;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub(crate) enum CmpResult {
	Eq,
	Lt,
	Gt,
	False,
}

impl CmpResult {
	pub fn from_psr(psr: u32) -> CmpResult {
		Self::from_u32(psr & 0x3).unwrap()
	}
}

#[derive(Clone, Debug)]
pub struct State {
	regs: [u32; 31],
	psr: u32,
}

// 30 because we don't reserved space for r0
const PC: usize = 30;

pub(crate) fn execute_binop(op: BinOp, lhs: u32, rhs: u32) -> u32 {
		match op {
		BinOp::Add => lhs + rhs,
		BinOp::Sub => lhs - rhs,
		BinOp::Mul => lhs * rhs,
		BinOp::Div => lhs / rhs,
		BinOp::Mod => lhs % rhs,

		BinOp::And => lhs & rhs,
		BinOp::Or => lhs | rhs,
		BinOp::Xor => lhs ^ rhs,

		BinOp::Shl => lhs << rhs,
		BinOp::Shr => lhs >> rhs,
		BinOp::Asl => ((lhs as i32) >> rhs) as u32,
		BinOp::Asr => ((lhs as i32) << rhs) as u32,
		BinOp::Rol => lhs.rotate_left(rhs),
		BinOp::Ror => lhs.rotate_right(rhs),

		BinOp::Not => !(lhs + rhs),
		BinOp::Neg => -((lhs + rhs) as i32) as u32,
		BinOp::Cmp => if lhs == rhs {
			CmpResult::Eq
		} else if lhs < rhs {
			CmpResult::Lt
		} else {
			CmpResult::Gt
		}.to_u32().unwrap(),
	}
}

impl State {
	pub fn new() -> State {
		State {
			regs: [0u32; 31],
			psr: 0,
		}
	}

	pub fn read_reg(&self, r: Register) -> u32 {
		if r.as_u8() == 0 {
			0
		} else {
			self.regs[r.as_u8() as usize - 1]
		}
	}

	pub fn write_reg(&mut self, r: Register, value: u32) {
		if r.as_u8() != 0 {
			self.regs[r.as_u8() as usize - 1] = value
		}
	}

	pub fn read_psr(&self) -> u32 {
		self.psr
	}

	pub fn write_psr(&mut self, value: u32) {
		self.psr = value;
	} 

	pub fn pc(&self) -> u32 {
		self.regs[PC]
	}

	pub fn pc_mut(&mut self) -> &mut u32 {
		&mut self.regs[PC]
	}

	fn execute_one(&mut self, instr: &Instruction) {
		debug!("Executing {:08x} {:?}", instr.encode(), instr);
		let pc_prev = self.pc();

		match instr {
			Instruction::Rrr(i) => rrr::execute(self, i),
			Instruction::Rri(i) => rri::execute(self, i),
			_ => panic!("Unsupported instruction type")
		}

		// If pc wasn't updated by a jump, advance to next instruction
		if pc_prev == self.pc() {
			*self.pc_mut() += 4;
		}
	}

	pub fn execute(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			self.execute_one(instr)
		}
	}
 }