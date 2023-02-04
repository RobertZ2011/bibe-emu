use bibe_instr::{
	BinOp,
	Instruction,
	Register,
};

mod rrr;
mod rri;

#[derive(Clone, Debug)]
pub struct State {
	regs: [u32; 31]
}

// 30 because we don't reserved space for r0
const PC: usize = 30;

pub(crate) fn execute_binop(op: BinOp, lhs: u32, rhs: u32) -> u32 {
		match op {
		BinOp::Add => lhs + rhs,
		BinOp::Sub => lhs - rhs,
		BinOp::Mul => lhs * rhs,
		BinOp::Div => lhs / rhs,

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
	}
}

impl State {
	pub fn new() -> State {
		State {
			regs: [0u32; 31]
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

	pub fn pc(&self) -> u32 {
		self.regs[PC]
	}

	pub fn pc_mut(&mut self) -> &mut u32 {
		&mut self.regs[PC]
	}

	fn execute_one(&mut self, instr: &Instruction) {
		match instr {
			Instruction::Rrr(i) => rrr::execute(self, i),
			Instruction::Rri(i) => rri::execute(self, i),
			_ => panic!("Unsupported instruction type")
		}

		*self.pc_mut() += 4;
	}

	pub fn execute(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			self.execute_one(instr)
		}
	}
 }