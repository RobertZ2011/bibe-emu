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

pub(crate) fn execute_binop(op: BinOp, lhs: u32, rhs: u32) -> Result<u32> {
	match op {
		BinOp::Add => Ok(lhs.wrapping_add(rhs)),
		BinOp::Sub => Ok(lhs.wrapping_sub(rhs)),
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
		BinOp::Cmp => Ok(if lhs == rhs {
			CmpResult::Eq
		} else if lhs < rhs {
			CmpResult::Lt
		} else {
			CmpResult::Gt
		}.to_u32().unwrap()),
	}
}