#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod aead;
mod dh;
mod ec;
mod kdf;
mod kem;

pub use aead::{AeadId, AeadType, AEAD_ID_EXPORT_ONLY, AES_TAG_LEN};
pub use dh::DhType;
pub use ec::Curve;
pub use kdf::{KdfId, KdfType};
pub use kem::{KemId, KemResult, KemType};

#[cfg(feature = "mock")]
pub mod mock;
