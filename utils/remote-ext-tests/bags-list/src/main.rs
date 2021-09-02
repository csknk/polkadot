// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Remote tests.

mod voter_bags;

#[tokio::main]
async fn main() {
	if cfg!(feature = "polkadot") {
		use polkadot_runtime::{constants::currency::UNITS, Block, Runtime};
		voter_bags::test_voter_bags_migration::<Runtime, Block>(UNITS as u64).await;
	} else {
		use kusama_runtime::{constants::currency::UNITS, Block, Runtime};
		voter_bags::test_voter_bags_migration::<Runtime, Block>(UNITS as u64).await;
	}
}
