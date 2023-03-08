use std::collections::HashSet;

use bibe_instr::BinOp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Extension {
	IntegerMultplication,
}

#[derive(Debug)]
pub struct Target {
	extensions: HashSet<Extension>,
}

impl Target {
	pub fn new() -> Self {
		Self {
			extensions: HashSet::new(),
		}
	}

	/// Target with all extensions present
	pub fn all() -> Self {
		let mut target = Self::new();
		target.add_extension(Extension::IntegerMultplication);
		target
	}

	pub fn parse(s: &str) -> Option<Self> {
		let mut target = Self::new();

		// Need at least `bibe32`
		if s.len() < 6 || &s[0..6] != "bibe32" {
			return None;
		}

		for c in (&s[6..]).chars() {
			match c {
				'A' => return Some(Self::all()),
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
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		// Try no extensions
		assert!(Target::parse("bibe32").is_some());

		// Try all the individual ones
		assert!(Target::parse("bibe32i").is_some());
		assert!(Target::parse("bibe32A").is_some());

		// Verify that the target string has to start with 'bibe32'
		assert!(Target::parse("i").is_none());
		assert!(Target::parse("A").is_none());
	}
}