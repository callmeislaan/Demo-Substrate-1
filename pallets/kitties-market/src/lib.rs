#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;
	use frame_support::inherent::Vec;
	use frame_system::ensure_signed;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_kitties::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		CountOfKitties(u32),
		Kitties(Vec<(T::Hash, pallet_kitties::pallet::Kitty<T>)>),
		KittiesByPrice(Vec<(T::Hash, pallet_kitties::pallet::Kitty<T>)>),
		KittiesByOwner((T::AccountId, Vec<T::Hash>)),

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		pub fn get_all_kitties(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let kitties_value = pallet_kitties::Pallet::<T>::get_all_kitties();
			Self::deposit_event(Event::Kitties(kitties_value));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn get_all_kitties_by_owner(origin: OriginFor<T>, owner: T::AccountId) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let kitties_owned = pallet_kitties::Pallet::<T>::get_kitties_by_owner(&owner);
			Self::deposit_event(Event::KittiesByOwner((owner, kitties_owned)));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn get_all_my_kitties(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitties_owned = pallet_kitties::Pallet::<T>::get_kitties_by_owner(&sender);
			Self::deposit_event(Event::KittiesByOwner((sender, kitties_owned)));
			Ok(())
		}
		
	}
}
