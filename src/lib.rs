pub mod memory;
pub mod state;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Exception {
	DivideByZero,
    InvalidOpcode,
    UnalignedAccess,
    MemFault,
}

pub type Result<T> = std::result::Result<T, Exception>;
