#![no_std]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "oid")]
use digest::const_oid::{AssociatedOid, ObjectIdentifier};

mod gost94_core;
/// GOST94 parameters.
pub mod params;

pub use digest::{self, Digest};

pub use gost94_core::Gost94Core;

digest::newtype_fixed_hash!(
    /// GOST94 hash function with CryptoPro parameters.
    pub struct Gost94CryptoPro(Gost94Core<params::CryptoProParam>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Gost94CryptoPro {
    /// Per RFC 4490
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.643.2.2.9");
}

digest::newtype_fixed_hash!(
    /// GOST94 hash function with S-box defined in GOST R 34.12-2015.
    pub struct Gost94s2015(Gost94Core<params::S2015Param>);
);

digest::newtype_fixed_hash!(
    /// GOST94 hash function with test parameters.
    pub struct Gost94Test(Gost94Core<params::TestParam>);
);

digest::newtype_fixed_hash!(
    /// GOST94 hash function with UAPKI GOST 34.311-95 parameters
    /// (1.2.804.2.1.1.1.1.2.1 OID).
    pub struct Gost94UA(Gost94Core<params::GOST28147UAParam>);
);

#[cfg(feature = "oid")]
impl AssociatedOid for Gost94UA {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.804.2.1.1.1.1.2.1");
}
