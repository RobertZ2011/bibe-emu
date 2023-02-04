use bibe_instr::Register;

pub struct State {
	regs: [u32; 31]
}

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
}