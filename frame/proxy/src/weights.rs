// This file is part of Axlib.

// Copyright (C) 2022 Axia Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/axlib
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/proxy/src/weights.rs
// --template=.maintain/frame-weight-template.hbs
// --header=HEADER-APACHE2
// --raw

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_proxy.
pub trait WeightInfo {
	fn proxy(p: u32, ) -> Weight;
	fn proxy_announced(a: u32, p: u32, ) -> Weight;
	fn remove_announcement(a: u32, p: u32, ) -> Weight;
	fn reject_announcement(a: u32, p: u32, ) -> Weight;
	fn announce(a: u32, p: u32, ) -> Weight;
	fn add_proxy(p: u32, ) -> Weight;
	fn remove_proxy(p: u32, ) -> Weight;
	fn remove_proxies(p: u32, ) -> Weight;
	fn anonymous(p: u32, ) -> Weight;
	fn kill_anonymous(p: u32, ) -> Weight;
}

/// Weights for pallet_proxy using the Axlib node and recommended hardware.
pub struct AxlibWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AxlibWeight<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy(p: u32, ) -> Weight {
		(13_323_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((114_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(28_825_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((254_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((120_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(20_594_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((259_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((15_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(20_316_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((265_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((18_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce(a: u32, p: u32, ) -> Weight {
		(27_696_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((241_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((111_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy(p: u32, ) -> Weight {
		(22_849_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((140_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy(p: u32, ) -> Weight {
		(19_350_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((147_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies(p: u32, ) -> Weight {
		(18_878_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((112_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous(p: u32, ) -> Weight {
		(25_743_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((25_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous(p: u32, ) -> Weight {
		(20_484_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((105_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy(p: u32, ) -> Weight {
		(13_323_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((114_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(28_825_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((254_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((120_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(20_594_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((259_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((15_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(20_316_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((265_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((18_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce(a: u32, p: u32, ) -> Weight {
		(27_696_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((241_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((111_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy(p: u32, ) -> Weight {
		(22_849_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((140_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy(p: u32, ) -> Weight {
		(19_350_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((147_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies(p: u32, ) -> Weight {
		(18_878_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((112_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous(p: u32, ) -> Weight {
		(25_743_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((25_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous(p: u32, ) -> Weight {
		(20_484_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((105_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
