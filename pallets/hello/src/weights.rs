
//! Autogenerated weights for pallet_hello
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Alexs-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../../target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_hello
// --extrinsic
// *
// --steps=50
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --output
// pallets/hello/src/weights.rs
// --hello
// ../../.maintain/frame-weight-hello.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_hello.
pub trait WeightInfo {
	fn say_hello() -> Weight;
	fn say_any() -> Weight;
}

/// Weights for pallet_hello using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: HelloModule Something (r:0 w:1)
	/// Proof: HelloModule Something (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn say_hello() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: HelloModule Something (r:1 w:1)
	/// Proof: HelloModule Something (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn say_any() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: HelloModule Something (r:0 w:1)
	/// Proof: HelloModule Something (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn say_hello() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: HelloModule Something (r:1 w:1)
	/// Proof: HelloModule Something (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn say_any() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
