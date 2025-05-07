#![no_std]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "oid")]
use digest::const_oid::{AssociatedOid, ObjectIdentifier};
use digest::{
    consts::{U32, U64},
    core_api::CtVariableCoreWrapper,
};

mod consts;
mod core_api;

pub use core_api::StreebogVarCore;
pub use digest::{self, Digest};

digest::newtype_fixed_hash!(
    /// Streebog256 hasher.
    pub struct Streebog256(CtVariableCoreWrapper<StreebogVarCore, U32>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Streebog256 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.643.7.1.1.2.2");
}

digest::newtype_fixed_hash!(
    /// Streebog512 hasher.
    pub struct Streebog512(CtVariableCoreWrapper<StreebogVarCore, U64>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Streebog512 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.643.7.1.1.2.3");
}
