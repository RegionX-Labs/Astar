// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet-assets chain-extension.
pub trait WeightInfo {
    fn owner() -> Weight;
    fn collection_owner() -> Weight;
    fn attribute() -> Weight;
    fn collection_attribute() -> Weight;
    fn can_transfer() -> Weight;
    fn collections(n: u32) -> Weight;
    fn items(n: u32) -> Weight;
    fn owned(n: u32) -> Weight;
    fn owned_in_collection(n: u32) -> Weight;
}

/// Weights for pallet-uniques chain-extension
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn owner() -> Weight {
        T::DbWeight::get().reads(1 as u64)
    }

    fn collection_owner() -> Weight {
        T::DbWeight::get().reads(1 as u64)
    }

    fn attribute() -> Weight {
        T::DbWeight::get().reads(1 as u64)
    }

    fn collection_attribute() -> Weight {
        T::DbWeight::get().reads(1 as u64)
    }

    fn can_transfer() -> Weight {
        T::DbWeight::get().reads(1 as u64)
    }

    fn collections(n: u32) -> Weight {
        T::DbWeight::get().reads(1 as u64).saturating_mul(n.into())
    }

    fn items(n: u32) -> Weight {
        T::DbWeight::get().reads(1 as u64).saturating_mul(n.into())
    }

    fn owned(n: u32) -> Weight {
        T::DbWeight::get().reads(1 as u64).saturating_mul(n.into())
    }

    fn owned_in_collection(n: u32) -> Weight {
        T::DbWeight::get().reads(1 as u64).saturating_mul(n.into())
    }
}
