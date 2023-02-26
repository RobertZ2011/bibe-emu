use std::fmt;

use bibe_instr::{
	BinOp,
	Encode,
	Instruction,
	Register,
	Width,
};

use crate::memory::Memory;

use bitfield::bitfield;
use log::debug;
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::ToPrimitive;

mod memory;
mod model;
mod rrr;
mod rri;

use model::Msr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub(crate) enum CmpResult {
	None,
	Lt,
	Gt,
	Eq,
}

bitfield! {
	pub struct Psr(u32);
	impl Debug;
	pub cmp_res, set_cmp_res : 1, 0;
	pub msr_quiet, set_msr_quet : 2, 2;
	pub msr_err, set_msr_err : 3, 3;
}

pub struct State {
	regs: [u32; 31],
	memory: Box<dyn Memory>,

	psr: Psr,
	isr_base: u32,
	isr_sp: u32,
	isr_old_sp: u32,
	isr_old_ip: u32,
	isr_err1: u32,
	isr_err2: u32
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
			memory: memory,

			psr: Psr(0),
			isr_base: 0,
			isr_sp: 0,
			isr_old_sp: 0,
			isr_old_ip: 0,
			isr_err1: 0,
			isr_err2: 0,
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

	pub fn read_psr(&self) -> Psr {
		Psr(self.psr.0)
	}

	pub fn write_psr(&mut self, value: Psr) {
		self.psr = value;
	}

	pub fn read_msr(&self, reg: Msr) -> Option<u32> {
		match reg {
			Msr::Psr => Some(self.read_psr().0),
			Msr::IsrBase => Some(self.isr_base),
			Msr::IsrSp => Some(self.isr_sp),
			Msr::IsrOldSp => Some(self.isr_old_sp),
			Msr::IsrOldIp => Some(self.isr_old_ip),
			Msr::IsrErr1 => Some(self.isr_err1),
			Msr::IsrErr2 => Some(self.isr_err2),
			_ => None,
		}
	}

	pub fn write_msr(&mut self, reg: Msr, value: u32) -> Option<()> {
		match reg {
			Msr::Psr => {
				self.write_psr(Psr(value));
				Some(())
			},
			Msr::IsrBase => {
				self.isr_base = value;
				Some(())
			},
			Msr::IsrSp => {
				self.isr_sp = value;
				Some(())
			},
			Msr::IsrOldSp => {
				self.isr_old_sp = value;
				Some(())
			},
			Msr::IsrOldIp => {
				self.isr_old_ip = value;
				Some(())
			},
			Msr::IsrErr1 => {
				self.isr_err1 = value;
				Some(())
			},
			Msr::IsrErr2 => {
				self.isr_err2 = value;
				Some(())
			},
			_ => None,
		}
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
			Instruction::Model(i) => model::execute(self, i),
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
		write!(formatter, "\tpsr: 0x{:08x}\n", self.psr.0).unwrap();
		Ok(())
	}
}
