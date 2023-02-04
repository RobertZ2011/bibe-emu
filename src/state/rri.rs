use bibe_instr::{
	BinOp,
	rri::{
		Instruction,
		Condition
	},
};

use crate::{
	state::{
        execute_binop,
        State,
    },
};

pub fn execute(s: &mut State, instr: &Instruction) {
    if instr.cond != Condition::Al {
        panic!("Conditions not yet supported");
    }

    let src = s.read_reg(instr.src);
    let imm = (instr.imm as i32) as u32;
    s.write_reg(instr.dest, execute_binop(instr.op, src, imm));
}