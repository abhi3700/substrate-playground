//! # Lockable Currency pallet
//! ## Overview
//!
//! A simple pallet demonstrating the usage of `LockableCurrency` trait.
//!
//! NOTE: The runtime must include the `Balances` pallet to handle the
//! accounts and balances for your chain.
//!
//! ## Interface
//!
//! ### Dispatchables
//!
//! - `lock_capital`
//! - `extend_lock`
//! - `unlock_all`
//!
//! ## References
//! - https://docs.substrate.io/reference/how-to-guides/pallet-design/implement-lockable-currency/

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
	};
	use frame_system::pallet_prelude::*;

	const EXAMPLE_ID: LockIdentifier = *b"example ";

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	type BalanceOf<T> =
		<<T as Config>::StakeCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		// The lockable currency type
		type StakeCurrency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	// Here, the pallet's storage items can be defined by
	// having the person 🧍 -> locked_id -> locked_amount💰
	// #[pallet::storage]
	// #[pallet::getter(fn something)]
	// pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Locked { user: T::AccountId, amount: BalanceOf<T> },
		ExtendedLock { user: T::AccountId, amount: BalanceOf<T> },
		Unlocked { user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	// #[pallet::error]
	// pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Locks the specified amount of tokens from the caller.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn lock_capital(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// lock amount
			T::StakeCurrency::set_lock(EXAMPLE_ID, &user, amount, WithdrawReasons::all());

			// Emit an event.
			Self::deposit_event(Event::Locked { user, amount });

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Extends the lock period.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn extend_lock(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// extend lock amount
			T::StakeCurrency::extend_lock(EXAMPLE_ID, &user, amount, WithdrawReasons::all());

			// Emit an event.
			Self::deposit_event(Event::ExtendedLock { user, amount });

			Ok(())
		}

		/// Releases all locked tokens.
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn unlock_all(origin: OriginFor<T>) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// unlock amount
			T::StakeCurrency::remove_lock(EXAMPLE_ID, &user);

			// emit event
			Self::deposit_event(Event::Unlocked { user });

			Ok(())
		}
	}
}
