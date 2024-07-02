//! Proxmox schema module.
//!
//! This provides utilities to define APIs in a declarative way using
//! Schemas. Primary use case it to define REST/HTTP APIs. Another use case
//! is to define command line tools using Schemas. Finally, it is
//! possible to use schema definitions to derive configuration file
//! parsers.

#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "api-macro")]
pub use proxmox_api_macro::api;

mod api_type_macros;

#[macro_use]
mod const_regex;
pub use const_regex::ConstRegexPattern;

pub mod de;
pub mod format;
pub mod ser;

pub mod property_string;

mod schema;
pub use schema::*;

pub mod upid;

// const_regex uses lazy_static, but we otherwise don't need it, and don't want to force users to
// have to write it out in their Cargo.toml as dependency, so we add a hidden re-export here which
// is semver-exempt!
#[doc(hidden)]
pub mod semver_exempt {
    pub use lazy_static::lazy_static;
}

#[cfg(feature = "api-types")]
pub mod api_types;
