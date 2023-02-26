use num_derive::{ FromPrimitive, ToPrimitive };

/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
pub mod memory;
pub mod state;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ExceptionKind {
	Reset,
	DivideByZero,
	Opcode,
	MemFault,
	Swi,
	Irq,
	Nmi,
	IsrExit,
}

#[derive(Debug)]
pub struct Exception {
	pub kind: ExceptionKind,
	pub err1: u32,
	pub err2: u32,
}

impl Exception {
	pub fn div_zero() -> Exception {
		Exception {
			kind: ExceptionKind::DivideByZero,
			err1: 0,
			err2: 0,
		}
	}

	pub fn mem_fault(addr: u32, unaligned: bool) -> Exception {
		Exception {
			kind: ExceptionKind::MemFault,
			err1: addr,
			err2: unaligned as u32
		}
	}

	pub fn opcode() -> Exception {
		Exception {
			kind: ExceptionKind::Opcode,
			err1: 0,
			err2: 0,
		}
	}
}

pub type Result<T> = std::result::Result<T, Exception>;
