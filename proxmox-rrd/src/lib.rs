//! # Round Robin Database files
//!
//! ## Features
//!
//! * One file stores a single data source
//! * Stores data for different time resolution
//! * Simple cache implementation with journal support

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "rrd_v1")]
mod rrd_v1;

pub mod rrd;
#[doc(inline)]
pub use rrd::Entry;

mod cache;
pub use cache::*;

#[cfg(feature = "api-types")]
pub mod api_types;
