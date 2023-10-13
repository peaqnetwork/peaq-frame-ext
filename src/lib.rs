//! It stands for Peaq's frame extension, whereas the term frame belongs to the substrate-framework.
//! We seperate definitions and implementations, that are very generic for substrate-pallets out into
//! this crate. In case someone in the open-source-world wants to use some of our pallets, that depend
//! on this crate's trait definitions, you only have to import this crate and not other parts from the
//! peaq-network-node. The main goal of this crate is independence by using loose coupling of pallets
//! and very generic reusable implementations.
//!
//! ### Features
//!
//! Currently, there is one use case for the averaging-capability in the peaq-network-node, that is
//! Currency-based. For this use case we designed a pallet-mockup, to be able to write independent
//! unit-tests for pallets, that uses the averaging-traits. If you want to use this mockup, the feature
//! "mockup-avg-currency" needs to be enabled. Feel free to add other datatype-based mockups, or a
//! datatype-independent mockup.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod averaging;

#[cfg(feature = "mockup-avg-currency")]
pub mod mockups;
