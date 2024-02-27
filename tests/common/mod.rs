#![cfg(feature = "std")]
extern crate std;

use std::collections::HashMap;

use bibe_asm::asm::emitter::link_instruction;
use bibe_asm::asm::Directive;
use bibe_emu::state::csr::*;
use bibe_emu::{memory::Mock, InterruptKind};
use bibe_emu::state::State;
use bibe_emu::target::StdTarget;
use bibe_instr::{Encode, Instruction, Register};
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

	for (i, instr) in instructions.iter().enumerate() {
		let encoded = instr.encode();
		let decoded = Instruction::decode(encoded);

		if decoded.is_none() {
			panic!("Instruction #{i} failed to decode {encoded:08X} {instr:?}");
		}

		if &decoded.unwrap() != instr {
			panic!("Instruction #{i} decoding/encoding is not idempotent");
		}
	}

	instructions
}

const EXECUTION_LIMIT: usize = 100_000;

pub fn run(program: &Vec<Instruction>, a0: u32) -> u32 {
	let mut state: State<_, Mock, Vec<Box<dyn CsrBlock>>> = State::new(StdTarget::new(), None, vec![
		Box::new(PsrBlock::new()),
		Box::new(IsrBlock::new()),
	]);
	let mut executed = 0;

	state.core.borrow_mut().write_reg(Register::a0(), a0);

	loop {
		let pc = state.core.borrow().read_pc();
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

	
	let val = state.core.borrow().read_reg(Register::o0());
	val
}