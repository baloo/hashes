#![no_std]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![cfg_attr(
    any(sha2_backend = "riscv-zknh", sha2_backend = "riscv-zknh-compact"),
    feature(riscv_ext_intrinsics)
)]
#![allow(clippy::needless_range_loop)]

#[cfg(all(
    any(sha2_backend = "riscv-zknh", sha2_backend = "riscv-zknh-compact"),
    not(any(any(target_arch = "riscv32", target_arch = "riscv64")))
))]
compile_error!("The Zknh backends can be enabled only for RISC-V targets");

pub use digest::{self, Digest};

#[cfg(feature = "oid")]
use digest::const_oid::{AssociatedOid, ObjectIdentifier};
use digest::{
    consts::{U28, U32, U48, U64},
    core_api::CtVariableCoreWrapper,
};

#[rustfmt::skip]
mod consts;
mod core_api;
mod sha256;
mod sha512;

pub use sha256::compress256;
pub use sha512::compress512;

pub use core_api::{Sha256VarCore, Sha512VarCore};

digest::newtype_fixed_hash!(
    /// SHA-224 hasher.
    pub struct Sha224(CtVariableCoreWrapper<Sha256VarCore, U28>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Sha224 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.16.840.1.101.3.4.2.4");
}

digest::newtype_fixed_hash!(
    /// SHA-256 hasher.
    pub struct Sha256(CtVariableCoreWrapper<Sha256VarCore, U32>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Sha256 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.16.840.1.101.3.4.2.1");
}

digest::newtype_fixed_hash!(
    /// SHA-512/224 hasher.
    pub struct Sha512_224(CtVariableCoreWrapper<Sha512VarCore, U28>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Sha512_224 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.16.840.1.101.3.4.2.5");
}

digest::newtype_fixed_hash!(
    /// SHA-512/256 hasher.
    pub struct Sha512_256(CtVariableCoreWrapper<Sha512VarCore, U32>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Sha512_256 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.16.840.1.101.3.4.2.6");
}

digest::newtype_fixed_hash!(
    /// SHA-384 hasher.
    pub struct Sha384(CtVariableCoreWrapper<Sha512VarCore, U48>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Sha384 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.16.840.1.101.3.4.2.2");
}

digest::newtype_fixed_hash!(
    /// SHA-512 hasher.
    pub struct Sha512(CtVariableCoreWrapper<Sha512VarCore, U64>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Sha512 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.16.840.1.101.3.4.2.2");
}
