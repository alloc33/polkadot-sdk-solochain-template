//! Username Registry Pallet
//!
//! Provides secure on-chain storage and retrieval of usernames associated with Ethereum addresses.
//! All username updates require signed transactions for security and authenticity.

// Ensure no_std compatibility for WebAssembly compilation.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
use sp_core::H160;
pub use weights::*;

sp_api::decl_runtime_apis! {
    pub trait UsernameRegistryApi<AccountId: codec::Codec> {
        fn get_username(ethereum_address: H160) -> Option<scale_info::prelude::vec::Vec<u8>>;
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    /// Storage mapping Ethereum addresses to their registered usernames.
    /// Usernames are limited to 64 bytes to prevent storage bloat.
    #[pallet::storage]
    pub type UserNames<T: Config> =
        StorageMap<_, Blake2_128Concat, H160, BoundedVec<u8, ConstU32<64>>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UsernameSet {
            ethereum_address: H160,
            username: Vec<u8>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        UsernameTooLong,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register or update a username for the specified Ethereum address.
        ///
        /// # Parameters
        /// - `ethereum_address`: The Ethereum address to associate with the username
        /// - `username`: The username to register (max 64 bytes)
        ///
        /// # Errors
        /// - `UsernameTooLong`: Username exceeds 64 byte limit
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::set_username())]
        pub fn set_username(
            origin: OriginFor<T>,
            ethereum_address: H160,
            username: Vec<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            let bounded_username: BoundedVec<u8, ConstU32<64>> = username
                .try_into()
                .map_err(|_| Error::<T>::UsernameTooLong)?;

            UserNames::<T>::insert(ethereum_address, &bounded_username);

            Self::deposit_event(Event::UsernameSet {
                ethereum_address,
                username: bounded_username.into(),
            });

            Ok(())
        }
    }
}
