use bibe_instr::{
	Instruction,
	Register,
};

mod rrr;

#[derive(Clone, Debug)]
pub struct State {
	regs: [u32; 31]
}

const PC: usize = 31;

impl State {
	pub fn new() -> State {
		State {
			regs: [0u32; 31]
		}
	}

	pub fn read_reg(&self, r: Register) -> u32 {
		self.regs[r.as_u8() as usize]
	}

	pub fn write_reg(&mut self, r: Register, value: u32) {
		self.regs[r.as_u8() as usize] = value
	}

	pub fn pc(&self) -> u32 {
		self.regs[PC]
	}

	pub fn pc_mut(&mut self) -> &mut u32 {
		&mut self.regs[PC]
	}

	fn execute_one(&mut self, instr: &Instruction) {
		match instr {
			Instruction::Rrr(i) => rrr::execute(self, i),
			_ => panic!("Unsupported instruction type")
		}

		*self.pc_mut() += 4;
	}

	fn execute(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			self.execute_one(instr)
		}
	}
 }