use std::fmt;

use bibe_instr::{
	BinOp,
	Encode,
	Instruction,
	Register,
	Width,
};

use crate::{
	memory::Memory, 
	Exception, 
	ExceptionKind,
	Result
};

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
	pub exception_mode, set_exception_mode : 4, 4;
	pub exception_enabled, set_exception_enabled : 5, 5;
}

pub struct State {
	regs: [u32; 31],
	memory: Box<dyn Memory>,
	pc_changed: bool,

	psr: Psr,
	isr_base: u32,
	isr_sp: u32,
	isr_old_sp: u32,
	isr_old_pc: u32,
	isr_err1: u32,
	isr_err2: u32
}

// 30 because we don't reserved space for r0
const PC: usize = 30;

pub(crate) fn execute_binop(op: BinOp, lhs: u32, rhs: u32) -> Result<u32> {
	match op {
		BinOp::Add => Ok(lhs.wrapping_add(rhs)),
		BinOp::Sub => Ok(lhs.wrapping_sub(rhs)),
		BinOp::Mul => Ok(lhs.wrapping_mul(rhs)),
		BinOp::Div => if rhs == 0 {
			Err(Exception::div_zero())
		}
		else {
			Ok(lhs.wrapping_div(lhs / rhs))
		},
			
		BinOp::Mod => Ok(lhs.wrapping_rem(rhs)),

		BinOp::And => Ok(lhs & rhs),
		BinOp::Or => Ok(lhs | rhs),
		BinOp::Xor => Ok(lhs ^ rhs),

		BinOp::Shl => Ok(lhs << rhs),
		BinOp::Shr => Ok(lhs >> rhs),
		BinOp::Asl => Ok(((lhs as i32) >> rhs) as u32),
		BinOp::Asr => Ok(((lhs as i32) << rhs) as u32),
		BinOp::Rol => Ok(lhs.rotate_left(rhs)),
		BinOp::Ror => Ok(lhs.rotate_right(rhs)),

		BinOp::Not => Ok(!(lhs + rhs)),
		BinOp::Neg => Ok(-((lhs + rhs) as i32) as u32),
		BinOp::Cmp => Ok(if lhs == rhs {
			CmpResult::Eq
		} else if lhs < rhs {
			CmpResult::Lt
		} else {
			CmpResult::Gt
		}.to_u32().unwrap()),
	}
}

pub(self) trait Execute {
	type I;
	fn execute(s: &mut State, i: &Self::I) -> Result<()>;
}

impl State {
	pub fn new(memory: Box<dyn Memory>) -> State {
		State {
			regs: [0u32; 31],
			memory: memory,
			pc_changed: false,

			psr: Psr(0),
			isr_base: 0,
			isr_sp: 0,
			isr_old_sp: 0,
			isr_old_pc: 0,
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
		if r.as_u8() == PC as u8 {
			self.pc_changed = true;
		}

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
			Msr::IsrOldPc => Some(self.isr_old_pc),
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
			Msr::IsrOldPc => {
				self.isr_old_pc = value;
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

	pub fn read_pc(&self) -> u32 {
		self.read_reg(Register::pc())
	}

	pub fn write_pc(&mut self, value: u32) {
		self.write_reg(Register::pc(), value)
	}

	pub fn read_sp(&self) -> u32 {
		self.read_reg(Register::sp())
	}

	pub fn write_sp(&mut self, value: u32) {
		self.write_reg(Register::sp(), value)
	}

	pub fn mem<'a>(&'a self) -> &'a dyn Memory {
		self.memory.as_ref()
	}

	pub fn mem_mut<'a>(&'a mut self) -> &'a mut dyn Memory {
		self.memory.as_mut()
	}

	pub fn reset(&mut self) {
		debug!("Reset");
		for reg in &mut self.regs {
			*reg = 0;
		}

		self.psr = Psr(0);
		self.isr_base = 0;
		self.isr_err1 = 0;
		self.isr_err2 = 0;
		self.isr_old_pc = 0;
		self.isr_old_sp = 0;
		self.isr_sp = 0;
	}

	fn handle_exception(&mut self, e: &Exception) {
		if self.psr.exception_mode() == 1 {
			if e.kind == ExceptionKind::IsrExit {
				self.write_reg(Register::sp(), self.read_msr(Msr::IsrOldSp).unwrap());
				self.write_reg(Register::pc(), self.read_msr(Msr::IsrOldPc).unwrap());
				self.write_msr(Msr::IsrOldSp, 0);
				self.write_msr(Msr::IsrOldPc, 0);
				self.write_msr(Msr::IsrErr1, 0);
				self.write_msr(Msr::IsrErr2, 0);

				self.psr.set_exception_enabled(1);
				self.psr.set_exception_mode(0);

				debug!("ISR exit sp: {:08x}, pc: {:08x}", self.read_sp(), self.read_pc());
			} else {
				// Exception while handling an exception
				debug!("Exception {:?} while already handling exception", e);
				self.reset();
			}
		}
		else {
			let old_sp = self.read_sp();
			let old_pc = self.read_pc();

			self.psr.set_exception_enabled(0);
			self.psr.set_exception_mode(1);
			self.write_msr(Msr::IsrOldSp, old_sp);
			self.write_msr(Msr::IsrOldPc, old_pc);
			self.write_sp(self.read_msr(Msr::IsrSp).unwrap());
			self.write_msr(Msr::IsrErr1, e.err1);
			self.write_msr(Msr::IsrErr1, e.err2);

			let index: u32 = e.kind.to_u32().unwrap();
			let handler = self.read_msr(Msr::IsrBase).unwrap() + 4 * index;
			self.write_reg(Register::pc(), handler);

			debug!("Handling exception {:?} old_sp: {:08x}, old_pc: {:08x} sp: {:08x}, pc: {:08x}", e, old_sp, old_pc, self.read_sp(), self.read_pc());
		}
	}

	fn execute_one(&mut self, instr: &Instruction) {
		debug!("Executing {:08x} {:?}", instr.encode(), instr);
		self.pc_changed = false;

		let res = match instr {
			Instruction::Rrr(i) => rrr::Rrr::execute(self, i),
			Instruction::Rri(i) => rri::Rri::execute(self, i),
			Instruction::Memory(i) => memory::Memory::execute(self, i),
			Instruction::Model(i) => model::Model::execute(self, i),
			_ => panic!("Unsupported instruction type")
		};

		if let Err(e) = res {
			self.handle_exception(&e);
			return;
		}

		// If pc wasn't updated by a jump, advance to next instruction
		if !self.pc_changed {
			self.write_pc(self.read_pc() + 4);
		}

		debug!("{}", self);
	}

	pub fn execute_instructions(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			self.execute_one(instr)
		}
	}

	pub fn execute(&mut self) {
		let res = self.mem().read(self.read_pc(), Width::Word);
		if let Err(e) = res {
			self.handle_exception(&e);
			return;
		}
		let value = res.unwrap();
		debug!("Fetching instruction at {:08x}", self.read_pc());

		let instruction = Instruction::decode(value);
		if instruction.is_none() {
			self.handle_exception(&Exception::opcode());
			return;
		}

		let instruction = instruction.unwrap();
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
