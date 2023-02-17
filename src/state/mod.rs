use std::fmt;

use bibe_instr::{
	BinOp,
	Encode,
	Instruction,
	Register,
	Width,
};

use crate::memory::Memory;

use log::debug;
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

mod memory;
mod rrr;
mod rri;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub(crate) enum CmpResult {
	None,
	Lt,
	Gt,
	Eq,
}

impl CmpResult {
	pub fn from_psr(psr: u32) -> CmpResult {
		Self::from_u32(psr & 0x3).unwrap()
	}
}

pub struct State {
	regs: [u32; 31],
	psr: u32,
	memory: Box<dyn Memory>,
}

// 30 because we don't reserved space for r0
const PC: usize = 30;

pub(crate) fn execute_binop(op: BinOp, lhs: u32, rhs: u32) -> u32 {
	match op {
		BinOp::Add => lhs.wrapping_add(rhs),
		BinOp::Sub => lhs.wrapping_sub(rhs),
		BinOp::Mul => lhs.wrapping_mul(rhs),
		BinOp::Div => lhs.wrapping_div(lhs / rhs),
		BinOp::Mod => lhs.wrapping_rem(rhs),

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
	pub fn new(memory: Box<dyn Memory>) -> State {
		State {
			regs: [0u32; 31],
			psr: 0,
			memory: memory,
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

	pub fn mem<'a>(&'a self) -> &'a dyn Memory {
		self.memory.as_ref()
	}

	pub fn mem_mut<'a>(&'a mut self) -> &'a mut dyn Memory {
		self.memory.as_mut()
	}

	fn execute_one(&mut self, instr: &Instruction) {
		debug!("Executing {:08x} {:?}", instr.encode(), instr);
		let pc_prev = self.pc();

		match instr {
			Instruction::Rrr(i) => rrr::execute(self, i),
			Instruction::Rri(i) => rri::execute(self, i),
			Instruction::Memory(i) => memory::execute(self, i),
			_ => panic!("Unsupported instruction type")
		}.expect("Exception during execution");

		// If pc wasn't updated by a jump, advance to next instruction
		if pc_prev == self.pc() {
			*self.pc_mut() += 4;
		}

		debug!("{}", self);
	}

	pub fn execute_instructions(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			self.execute_one(instr)
		}
	}

	pub fn execute(&mut self) {
		let value = self.mem().read(self.pc(), Width::Word).expect("Failed to fetch instruction");
		debug!("Fetching instruction at {:08x}", self.pc());
		let instruction = Instruction::decode(value).expect("Failed to decode instruction");

		self.execute_one(&instruction);
	}
 }

impl fmt::Display for State {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("Core State\n")?;
		for i in 0..32 {
			write!(formatter, "\tr{}: 0x{:08x}\n", i,self.read_reg(Register::new(i).unwrap())).unwrap();
		}
		write!(formatter, "\tpsr: 0x{:08x}\n", self.psr).unwrap();
		Ok(())
	}
}
