use crate::Config;
use frame_system::pallet_prelude::*;
use frame_support::{
	sp_runtime::traits::Hash,
	traits::{ Randomness, Currency, tokens::ExistenceRequirement, Time },
	transactional
};
use sp_io::hashing::blake2_128;
use scale_info::TypeInfo;
use core::fmt::Debug;
use frame_support::inherent::Vec;
use codec::{Encode, Decode};
use frame_support::RuntimeDebug;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize, Serialize};

use log::{info};

use crate::AccountOf;
use crate::BalanceOf;
use crate::TimeOf;

// Struct for holding Kitty information.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Kitty<T: Config> {
	pub dna: [u8; 16],   // Using 16 bytes to represent a kitty DNA
	pub price: Option<BalanceOf<T>>,
	pub gender: Gender,
	pub owner: AccountOf<T>,
	pub created: TimeOf<T>,
}

#[cfg(feature = "std")]
mod serde_kitty {
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn serialize<S: Serializer, T: std::fmt::Display>(
		t: &T,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&t.to_string())
	}

	pub fn deserialize<'de, D: Deserializer<'de>, T: std::str::FromStr>(
		deserializer: D,
	) -> Result<T, D::Error> {
		let s = String::deserialize(deserializer)?;
		s.parse::<T>().map_err(|_| serde::de::Error::custom("Parse from string failed"))
	}
}

// Enum declaration for Gender.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Gender {
	Male,
	Female,
}

impl <T> sp_std::fmt::Display for Kitty<T> where T: Config {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		write!(f, "(dna: {:?}, price: {:?}, gender: {:?}, owner: {:?})", self.dna, self.price, self.gender, self.owner)
	}
}
