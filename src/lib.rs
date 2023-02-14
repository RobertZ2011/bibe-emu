/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
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
