/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
pub mod memory;
pub mod state;
pub mod target;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InterruptKind {
	Reset,
	Nmi,
	Breakpoint,
	AlignFault,
	MemoryFault,
	OpcodeFault,
	DoubleFault,
	Swi,
	Reserved(u8),
	Irq(u8),

	// Implementation detail, doesn't exist in the ISA
	IsrExit,
}

impl InterruptKind {
	pub fn to_index(self) -> Option<u32> {
		if self == InterruptKind::IsrExit {
			return None;
		}

		Some(match self {
			InterruptKind::Reset => 0,
			InterruptKind::Nmi => 1,
			InterruptKind::Breakpoint => 2,
			InterruptKind::AlignFault => 3,
			InterruptKind::MemoryFault => 4,
			InterruptKind::OpcodeFault => 5,
			InterruptKind::DoubleFault => 6,
			InterruptKind::Swi => 7,
			InterruptKind::Reserved(i) => 8 + i as u32,
			InterruptKind::Irq(i) => 16 + i as u32,
			_ => unreachable!(),
		})
	}
}

#[derive(Debug)]
pub struct Interrupt {
	pub kind: InterruptKind,
	pub err1: u32,
	pub err2: u32,
}

impl Interrupt {
	pub fn opcode() -> Interrupt {
		Interrupt {
			kind: InterruptKind::OpcodeFault,
			err1: 0,
			err2: 0,
		}
	}

	pub fn mem_fault(addr: u32) -> Interrupt {
		Interrupt {
			kind: InterruptKind::MemoryFault,
			err1: addr, err2: 0 
		}
	}
}

pub type Result<T> = std::result::Result<T, Interrupt>;
