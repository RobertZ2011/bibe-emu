use std::collections::HashMap;

use bibe_asm::asm::emitter::link_instruction;
use bibe_asm::asm::Directive;
use bibe_emu::InterruptKind;
use bibe_emu::state::State;
use bibe_emu::target::Target;
use bibe_instr::{Instruction, Register};
use bibe_asm::parser::{ tokenize, parse };

pub fn assemble(program: &str) -> Vec<Instruction> {
	let tokens = tokenize(program);
	if tokens.is_err() {
		panic!("Failed to tokenize program: {tokens:?}");
	}

	let (remaining, tokens) = tokens.unwrap();
	if remaining.len() != 0 {
		panic!("Failed to completely tokenize program, remaining {remaining}");
	}

	let statements = parse(&tokens);
	if statements.is_err() {
		panic!("Failed parse program: {statements:?}");
	}

	let (remaining, statements) = statements.unwrap();
	if remaining.len() != 0 {
		panic!("Failed to completely parse program, remaining {remaining:?}");
	}

	let mut instructions = Vec::new();
	let mut addr = 0;
	let mut symbols = HashMap::new();

	// Create symbol table
	for (i, statement) in statements.iter().enumerate() {
		if let Some(directive) = statement.directive() {
			match directive {
				Directive::Label(id) => { symbols.insert(*id, addr); },
				_ => panic!("Unsupported asm directive {i}"),
			}
		} else {
			addr += 4;
		}
	}

	addr = 0;
	for (i, statement) in statements.iter().enumerate() {
		if let Some(instr) = statement.instruction() {
			let linked = link_instruction(&symbols, addr, &instr);
			if let Err(e) = linked {
				panic!("Failed to link instruction #{i} {instr:?} {e:?}");
			}

			instructions.push(linked.unwrap());
			addr += 4;
		}
	}

	instructions
}

const EXECUTION_LIMIT: usize = 100_000;

pub fn run(program: &Vec<Instruction>, a0: u32) -> u32 {
	let mut state = State::new(None, Target::new());
	let mut executed = 0;

	state.write_reg(Register::a0(), a0);

	loop {
		let pc = state.read_pc();
		let i = (pc / 4) as usize;
		let res = state.execute(&program[i]);

		if let Err(interrupt) = res {
			match interrupt.kind {
				InterruptKind::Swi => break,
				_ => panic!("Interrupt: {interrupt:?}")
			}
		}

		if executed > EXECUTION_LIMIT {
			panic!("Execution limit exceeded");
		}

		executed += 1;
	}

	state.read_reg(Register::o0())
}