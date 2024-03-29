use bibe_instr::BinOp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Extension {
	IntegerMultplication,
}

pub trait Target {
	fn supports_binop(&self, op: BinOp) -> bool;
	fn has_extension(&self, extension: Extension) -> bool;
}

#[cfg(feature = "std")]
mod std {
	extern crate std;
	use std::collections::HashSet;
	use super::*;

	#[derive(Clone, Debug)]
	pub struct StdTarget {
		extensions: HashSet<Extension>,
	}

	impl StdTarget {
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
	
		pub fn add_extension(&mut self, extension: Extension) {
			self.extensions.insert(extension);
		}
	}

	impl Target for StdTarget {
		fn supports_binop(&self, op: BinOp) -> bool {
			match op {
				BinOp::Div
				| BinOp::Mod
				| BinOp::Mul => self.has_extension(Extension::IntegerMultplication),
				_ => true,
			}
		}
	
		fn has_extension(&self, extension: Extension) -> bool {
			self.extensions.contains(&extension)
		}
	}
}

#[cfg(feature = "std")]
pub use self::std::StdTarget;

#[cfg(test)]
#[cfg(feature = "std")]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		// Try no extensions
		assert!(StdTarget::parse("bibe32").is_some());

		// Try all the individual ones
		assert!(StdTarget::parse("bibe32i").is_some());
		assert!(StdTarget::parse("bibe32A").is_some());

		// Verify that the target string has to start with 'bibe32'
		assert!(StdTarget::parse("i").is_none());
		assert!(StdTarget::parse("A").is_none());
	}
}