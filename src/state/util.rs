use bibe_instr::BinOp;
use log::debug;
use crate::{
    Interrupt,
    Result,
};

use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::ToPrimitive;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub(crate) enum CmpResult {
	None,
	Lt,
	Gt,
	Eq,
}

pub struct BinOpOverflow {
	pub overflow: bool,
	pub carry: bool,
}

pub(crate) fn check_binop(op: BinOp, lhs: u32, rhs: u32) -> BinOpOverflow {
	let ilhs = lhs as i32;
	let irhs = rhs as i32;

	match op {
		BinOp::Addcc => BinOpOverflow {
			overflow: ilhs.overflowing_add(irhs).1,
			carry: lhs.overflowing_add(rhs).1
		},
		BinOp::Subcc => BinOpOverflow {
			overflow: ilhs.overflowing_sub(irhs).1,
			carry: lhs.overflowing_sub(rhs).1
		},
		_ => BinOpOverflow { overflow: false, carry: false },
	}
}

pub(crate) fn execute_binop(op: BinOp, lhs: u32, rhs: u32) -> Result<u32> {
	match op {
		BinOp::Add
		| BinOp::Addcc => Ok(lhs.wrapping_add(rhs)),
		BinOp::Sub 
		| BinOp::Subcc => Ok(lhs.wrapping_sub(rhs)),
		BinOp::Mul => Ok(lhs.wrapping_mul(rhs)),
		BinOp::Div => if rhs == 0 {
			Err(Interrupt::opcode())
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
	}
}