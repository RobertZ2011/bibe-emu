pub mod state;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Exception {
	DivideByZero,
    InvalidOpcode,
    UnalignedAccess,
}

pub type Result<T> = std::result::Result<Exception, T>;
