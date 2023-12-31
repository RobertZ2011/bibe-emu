use bibe_instr::Width;

use super::Memory;
use crate::Result;

struct MappedRegion {
	start: u32,
	memory: Box<dyn Memory>,
}

pub struct Mapped {
	regions: Vec<MappedRegion>,
}

impl MappedRegion {
	fn overlaps(&self, other: &MappedRegion) -> bool {
		(self.start >= other.start && self.start < other.end())
		|| (other.start >= self.start && other.start < self.end())
	}

	pub fn end(&self) -> u32 {
		self.start + self.size()
	}
}

impl Memory for MappedRegion {
	fn size(&self) -> u32 {
		self.memory.size()
	}

	fn read_validated(&self, addr: u32, width: Width) -> Result<u32> {
		self.memory.read_validated(addr, width)
	}

	fn write_validated(&mut self, addr: u32, width: Width, value: u32) -> Result<()> {
		self.memory.write_validated(addr, width, value)
	}
}

impl Mapped {
	pub fn new() -> Self {
		Self {
			regions: Vec::new()
		}
	}

	fn find_region(&self, addr: u32) -> Option<&MappedRegion> {
		//TODO: make this work in constant time
		for region in &self.regions {
			if addr >= region.start && addr < region.end() {
				return Some(region);
			}
		}

		None
	}

	fn find_region_mut(&mut self, addr: u32) -> Option<&mut MappedRegion> {
		//TODO: make this work in constant time
		for region in &mut self.regions {
			if addr >= region.start && addr < region.end() {
				return Some(region);
			}
		}

		None
	}

	pub fn is_mapped(&self, addr: u32) -> bool {
		self.find_region(addr).is_some()
	}

	/// Attempt to map `memory` at the given start address
	pub fn map(&mut self, start: u32, memory: Box<dyn Memory>) -> Option<()> {
		let new = MappedRegion {
			start,
			memory
		};

		// Find the index where this should be inserted
		let mut index = None;
		for (i, peripheral) in self.regions.iter().enumerate() {
			if new.overlaps(peripheral) {
				return None;
			}

			if start > peripheral.end() {
				if i + 1 < self.regions.len() {
					if new.overlaps(&self.regions[i + 1]) {
						// Requested address range overlap with the next peripheral
						return None;
					}
				}

				index = Some(i);
				break;
			}
		}

		if let Some(index) = index {
			self.regions.insert(index, new);
		}
		else {
			// Didn't find an index, must have an empty vec
			self.regions.push(new);
		}

		Some(())
	}
}

impl Memory for Mapped {
	fn contains(&self, addr: u32) -> bool {
		self.find_region(addr).is_some()
	}

	fn validate_access(&self, addr: u32, width: Width) -> bool {
		let region = self.find_region(addr);
		if region.is_none() {
			return false;
		}

		region.unwrap().validate_access(addr, width)
	}

	fn size(&self) -> u32 {
		if self.regions.len() == 0 {
			0
		} else {
			let last = self.regions.last().unwrap();
			last.start + last.size()
		}
	}

	fn read_validated(&self, addr: u32, width: Width) -> Result<u32> {
		self.find_region(addr).unwrap().memory.read_validated(addr, width)
	}

	fn write_validated(&mut self, addr: u32, width: Width, value: u32) -> Result<()> {
		self.find_region_mut(addr).unwrap().memory.write_validated(addr, width, value)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::memory::Mock;

	fn mock_memory(size: u32) -> Box<dyn Memory> {
		Box::new(Mock::new(size))
	}

	fn mock_region(start: u32, size: u32) -> MappedRegion {
		MappedRegion {
			start,
			memory: mock_memory(size),
		}
	}

	#[test]
	fn test_region() {
		let r =  mock_region(0, 32);

		// `overlaps` tests
		let full_overlap = mock_region(0, 32);
		assert!(r.overlaps(&full_overlap));
		assert!(full_overlap.overlaps(&r));

		let start_overlap = mock_region(0, 16);
		assert!(r.overlaps(&start_overlap));
		assert!(start_overlap.overlaps(&r));

		let end_overlap = mock_region(16, 32);
		assert!(r.overlaps(&end_overlap));
		assert!(end_overlap.overlaps(&r));

		let subregion = mock_region(0, 16);
		assert!(r.overlaps(&subregion));
		assert!(subregion.overlaps(&r));

		let no_overlap = mock_region(32, 48);
		assert!(!r.overlaps(&no_overlap));
		assert!(!no_overlap.overlaps(&r));

		let no_overlap = mock_region(128, 256);
		assert!(!r.overlaps(&no_overlap));
		assert!(!no_overlap.overlaps(&r));

		// `validate_access` tests
		// Test at beginning
		assert!(r.validate_access(0, Width::Byte));
		assert!(r.validate_access(0, Width::Short));
		assert!(r.validate_access(0, Width::Word));

		// Test at end
		assert!(r.validate_access(31, Width::Byte));
		assert!(r.validate_access(30, Width::Short));
		assert!(r.validate_access(28, Width::Word));

		// Test exact size
		let r = mock_region(0, 1);
		assert!(r.validate_access(0, Width::Byte));

		let r = mock_region(0, 2);
		assert!(r.validate_access(0, Width::Short));

		let r = mock_region(0, 4);
		assert!(r.validate_access(0, Width::Word));
	}

	#[test]
	fn test_mapped() {
		let mut mapped = Mapped::new();

		// Initial map
		assert!(mapped.map(0, mock_memory(32)).is_some());

		// Overlap
		// Exact
		assert!(mapped.map(0, mock_memory(32)).is_none());

		// Start
		assert!(mapped.map(0, mock_memory(16)).is_none());

		// End
		assert!(mapped.map(16, mock_memory(16)).is_none());

		// Continguous
		assert!(mapped.map(32, mock_memory(32)).is_some());

		// Disparate
		assert!(mapped.map(128, mock_memory(128)).is_some());

		// Sub region
		assert!(mapped.map(0, mock_memory(16)).is_none());
	}
}