use std::{
	cell::RefCell,
	fmt,
};

use bibe_instr::{
	Encode,
	Instruction,
	Register,
	Shift,
	ShiftKind,
	Width, Condition,
};
use bibe_instr::csr::regs::*;

use crate::{
	memory::Memory, 
	Interrupt, 
	InterruptKind,
	Result,
	target::Target,
};

use bitfield::bitfield;
use log::debug;

mod memory;
pub mod csr;
mod rrr;
mod rri;
mod jump;
mod util;

use self::csr::CsrCollection;

bitfield! {
	pub struct Psr(u32);
	impl Debug;
	pub v, set_v : 0, 0;
	pub c, set_c : 1, 1;
	pub z, set_z : 2, 2;
	pub n, set_n : 3, 3;
	pub msr_quiet, set_msr_quet : 2, 2;
	pub msr_err, set_msr_err : 3, 3;
	pub interrupt_mode, set_interrupt_mode : 4, 4;
	pub exception_enabled, set_exception_enabled : 5, 5;
}

impl Psr {
	pub fn set_zn(&mut self, val: u32) {
		self.set_z((val == 0) as u32);
		self.set_n(((val  as i32) < 0) as u32);
	}

	pub fn should_execute(&self, cond: Condition) -> bool {
		match cond {
			Condition::Always => true,
			Condition::Overflow => self.v() == 1,
			Condition::Carry => self.c() == 1,
			Condition::Zero => self.z() == 1,
			Condition::Negative => self.n() == 1,
			Condition::NotZero => self.z() == 0,
			Condition::NotNegative => self.n() == 0,
			Condition::GreaterThan => self.n() == 0 && self.z() == 0,
		}
	}
}

pub struct CoreState {
	regs: [u32; 31],
	pc_touched: bool,
}

impl CoreState {
	pub fn new() -> CoreState {
		CoreState {
			regs: [0; 31],
			pc_touched: false,
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
			self.pc_touched = true;
		}

		if r.as_u8() != 0 {
			self.regs[r.as_u8() as usize - 1] = value
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

	pub fn reset(&mut self) {
		for reg in &mut self.regs {
			*reg = 0;
		}
	}
}

pub struct State<T, M, C>
where
	T: Target,
	M: Memory,
	C: CsrCollection,
{
	pub core: RefCell<CoreState>,
	memory: Option<M>,
	target: T,

	csr_blocks: RefCell<C>,

	double_fault: bool,
}

const PC: usize = 31;

pub fn shift(s: &Shift, value: u32) -> u32 {
	let Shift {
		kind,
		shift: amount,
	} = s;

	match kind {
		ShiftKind::Shl => value << amount,
		ShiftKind::Shr => value >> amount,
		ShiftKind::Asl => ((value as i32) << amount) as u32,
		ShiftKind::Asr => ((value as i32) >> amount) as u32,
		ShiftKind::Rol => value.rotate_left(*amount as u32),
		ShiftKind::Ror => value.rotate_right(*amount as u32)
	}
}

impl<T, M, C> State<T, M, C>
where
	T: Target,
	M: Memory,
	C: CsrCollection,
{
	pub fn new(target: T, memory: Option<M>, csr_blocks: C) -> State<T, M, C> {
		State {
			core: RefCell::new(CoreState::new()),
			memory,
			target,
			csr_blocks: RefCell::new(csr_blocks),
			double_fault: false,
		}
	}

	pub fn attach_memory(&mut self, memory: Option<M>) {
		self.memory = memory
	}

	pub fn read_psr(&self) -> u32 {
		self.read_csr(PSR_PSR0_REG, Width::Word).unwrap()
	}

	pub fn write_psr(&mut self, value: u32) {
		self.write_csr(PSR_PSR0_REG, value, Width::Word).unwrap()
	}

	pub fn read_csr(&self, reg: u32, width: Width) -> Option<u32> {
		let mut index = 0;
		let mut found = false;
		let mut blocks = self.csr_blocks.borrow_mut();

		for i in 0..blocks.len() {
			let block = blocks.index(i);
			if reg >= block.base_reg() && reg < block.base_reg() + block.size() {
				found = true;
				break;
			}

			index += 1;
		}

		if found {
			return blocks.index_mut(index).read(&self.core.borrow(), reg, width);
		}

		None
	}

	pub fn write_csr(&mut self, reg: u32, value: u32, width: Width) -> Option<()> {
		let mut index = 0;
		let mut found = false;
		let mut blocks = self.csr_blocks.borrow_mut();

		debug!("CSR write: reg: {reg:#x}, value: {value:#x}, width: {width:?}");
		for i in 0..blocks.len() {
			let block = blocks.index(i);
			let base = block.base_reg();
			let max = block.base_reg() + block.size();

			debug!("CSR block: {index} {base:08x} {max:08x}");
			if reg >= base && reg < max {
				found = block.has_reg(reg);
				break;
			}

			index += 1;
		}

		if found {
			debug!("CSR block index {index}");
			return blocks.index_mut(index).write(&self.core.borrow_mut(), reg, width, value);
		}

		debug!("Invalid CSR write");
		None
	}

	pub fn target<'a>(&'a self) -> &'a T {
		&self.target
	}

	pub fn reset(&mut self) {
		self.core.borrow_mut().reset();
		for i in 0..self.csr_blocks.borrow().len() {
			self.csr_blocks.borrow_mut().index_mut(i).reset();
		}

		self.double_fault = false;
		debug!("Reset");
	}

	fn swap_interrupt_banks(&mut self) {
		let mut core = self.core.borrow_mut();
		let mut csr_blocks = self.csr_blocks.borrow_mut();
		let isr = csr_blocks.get_isr_mut();
		let mut tmp = [0u32; 31];

		let isr_pc_idx = ((ISR_PC_REG - ISR_BASE) / 4) as usize;
		let isr_r1_idx = ((ISR_R1_REG - ISR_BASE) / 4) as usize;

		tmp.clone_from_slice(core.regs.as_slice());
		core.regs.clone_from_slice(&isr.0[isr_r1_idx..=isr_pc_idx]);
		isr.0[isr_r1_idx..=isr_pc_idx].clone_from_slice(&tmp);
		core.pc_touched = true;
	}

	pub fn handle_interrupt(&mut self, e: &Interrupt) {
		let mut psr = Psr(self.read_psr());

		if psr.interrupt_mode() == 1 {
			if e.kind == InterruptKind::IsrExit {
				self.swap_interrupt_banks();
				
				self.write_csr(ISR_ERR1_REG, 0, Width::Word);
				self.write_csr(ISR_ERR2_REG, 0, Width::Word);

				psr.set_exception_enabled(1);
				psr.set_interrupt_mode(0);
				self.write_psr(psr.0);

				debug!("ISR exit sp: {:08x}, pc: {:08x}", self.core.borrow().read_sp(), self.core.borrow().read_pc());
			} else {
				// Interrupt while handling an interrupt
				// NMIs are always processed, trigger a double fault if we haven't already
				let mut index = 0;
				
				if e.kind == InterruptKind::Nmi {
					index = InterruptKind::Nmi.to_index().unwrap();
				} else {
					if !self.double_fault {
						self.double_fault = true;
						index = InterruptKind::DoubleFault.to_index().unwrap();
					}
				}

				let handler = self.read_csr(ISR_BASE_REG, Width::Word).unwrap() + 4 * index;
				self.core.borrow_mut().write_reg(Register::pc(), handler);
				debug!("Interrupt {:?} while already handling interrupt", e);
				if index == 0 {
					self.reset();
				}
			}
		}
		else {
			let old_sp = self.core.borrow().read_sp();
			let old_pc = self.core.borrow().read_pc();

			self.swap_interrupt_banks();

			psr.set_exception_enabled(0);
			psr.set_interrupt_mode(1);
			self.write_psr(psr.0);

			self.write_csr(ISR_ERR1_REG, e.err1, Width::Word);
			self.write_csr(ISR_ERR2_REG, e.err2, Width::Word);

			let index: u32 = e.kind.to_index().unwrap();
			let handler = self.read_csr(ISR_BASE_REG, Width::Word).unwrap() + 4 * index;
			self.core.borrow_mut().write_reg(Register::pc(), handler);

			debug!("Interrupt {:?} old_sp: {:08x}, old_pc: {:08x} sp: {:08x}, pc: {:08x}", e, old_sp, old_pc, self.core.borrow().read_sp(), self.core.borrow().read_pc());
		}
	}

	pub fn execute(&mut self, instr: &Instruction) -> Result<()>{
		debug!("Executing {:08x} {:?}", instr.encode(), instr);
		self.core.borrow_mut().pc_touched = false;

		let res = match instr {
			Instruction::Rrr(i) => rrr::execute(self, i),
			Instruction::Rri(i) => rri::execute(self, i),
			Instruction::Memory(i) => memory::execute(self, i),
			Instruction::Csr(i) => csr::execute(self, i),
			Instruction::Jump(i) => jump::execute(self, i),
			_ => panic!("Unsupported instruction type")
		};

		if res.is_err() {
			return res;
		}

		// If pc wasn't updated by a jump, advance to next instruction
		if !self.core.borrow().pc_touched {
			let new_pc = self.core.borrow().read_pc() + 4;
			self.core.borrow_mut().write_pc(new_pc);
		}

		debug!("{}", self);
		Ok(())
	}

	pub fn execute_instructions(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			if let Err(interrupt) = self.execute(instr) {
				self.handle_interrupt(&interrupt);
			}
		}
	}

	pub fn fetch(&self) -> Result<u32> {
		let core = self.core.borrow();
		debug!("Fetching instruction at {:08x}", core.read_pc());
		let res = self.memory.as_ref().unwrap().read(core.read_pc(), Width::Word);
		if res.is_err() {
			debug!("Failed to fetch instruction");
		}
		res
	}

	pub fn decode(&self, instr: u32) -> Result<Instruction> {
		let instruction = Instruction::decode(instr);
		if instruction.is_none() {
			debug!("Failed to decode instruction {instr:08x}");
			return Err(Interrupt::opcode());
		}
		Ok(instruction.unwrap())
	}

	pub fn execute_one(&mut self) {
		if self.memory.is_none() {
			return;
		}

		let instr = self.fetch();
		if let Err(int) = instr {
			self.handle_interrupt(&int);
			return;
		}

		let instr = self.decode(instr.unwrap());
		if let Err(int) = instr {
			self.handle_interrupt(&int);
			return;
		}

		let res = self.execute(&instr.unwrap());
		if let Err(int) = res {
			self.handle_interrupt(&int);
		}
	}
 }

impl<T, M, C> Memory for State<T, M, C>
where
 	T: Target,
	M: Memory,
	C: CsrCollection,
{
	fn size(&self) -> u32 {
		if self.memory.is_none() {
			return 0;
		}

		self.memory.as_ref().unwrap().size()
	}

	fn read(&self, addr: u32, width: Width) -> Result<u32> {
		if self.memory.is_none() {
			return Err(Interrupt::mem_fault(addr));
		}

		self.memory.as_ref().unwrap().read(addr, width)
	}

	fn write(&mut self, addr: u32, width: Width, val: u32) -> Result<()> {
		if self.memory.is_none() {
			return Err(Interrupt::mem_fault(addr));
		}

		self.memory.as_mut().unwrap().write(addr, width, val)
	}
}

impl<T,M, C> fmt::Display for State<T, M, C>
where
	T: Target,
	M: Memory,
	C: CsrCollection,
{
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let core = self.core.borrow();
		formatter.write_str("Core State\n")?;
		for i in 0..32 {
			write!(formatter, "\tr{}: 0x{:08x}\n", i, core.read_reg(Register::new(i).unwrap())).unwrap();
		}
		write!(formatter, "\tpsr: 0x{:08x}\n", self.read_psr()).unwrap();
		Ok(())
	}
}
