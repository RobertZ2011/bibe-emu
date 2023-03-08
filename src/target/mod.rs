use std::collections::HashSet;

use bibe_instr::BinOp;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Extension {
	IntegerMultplication,
}

pub struct Target {
	extensions: HashSet<Extension>,
}

impl Target {
	pub fn new() -> Self {
		Self {
			extensions: HashSet::new(),
		}
	}

	pub fn parse(s: &str) -> Option<Self> {
		let mut target = Self::new();

		// Need at least `bibe32`
		if s.len() < 6 || &s[0..5] != "bibe32" {
			return None;
		}

		for c in (&s[6..]).chars() {
			match c {
				'i' => target.add_extension(Extension::IntegerMultplication),
				_ => continue,
			}
		}

		Some(target)
	}

	pub fn supports_binop(&self, op: BinOp) -> bool {
		match op {
			BinOp::Div
			| BinOp::Mod
			| BinOp::Mul => self.has_extension(Extension::IntegerMultplication),
			_ => true,
		}
	}

	pub fn has_extension(&self, extension: Extension) -> bool {
		self.extensions.contains(&extension)
	}

	pub fn add_extension(&mut self, extension: Extension) {
		self.extensions.insert(extension);
	}

	pub fn is_64bit(&self) -> bool {
		false
	}
}