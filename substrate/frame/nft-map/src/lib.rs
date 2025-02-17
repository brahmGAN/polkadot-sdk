//! # NFT Map Pallet
//! 
//! A simple pallet for managing NFT ownership and values.
//!
//! ## Overview
//!
//! This pallet provides functionality to:
//! - Add new NFTs for accounts
//! - Update existing NFT values
//! - Remove NFTs from accounts
//!
//! The pallet uses a simple storage map to track NFT ownership and values.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::Get,
        storage::types::StorageMap,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use codec::{MaxEncodedLen};
    use scale_info::TypeInfo;
    use serde::{Serialize, Deserialize};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuration trait for NFT Map pallet
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// The type used to store NFT values
        type NFTValue: Member + Parameter + From<u32> + Into<u32> + MaxEncodedLen + Copy 
            + Serialize + for<'a> Deserialize<'a> + TypeInfo;
        
        /// Minimum value for an NFT
        #[pallet::constant]
        type MinValue: Get<u32>;
        
        /// Maximum value for an NFT
        #[pallet::constant]
        type MaxValue: Get<u32>;
    }

    /// Events emitted by the NFT Map pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// NFT was added. [account, value, timestamp]
        NFTAdded { 
            who: T::AccountId, 
            val: T::NFTValue, 
            when: BlockNumberFor<T>
        },
        /// NFT was updated. [account, old_value, new_value]
        NFTUpdated { 
            who: T::AccountId, 
            old_val: T::NFTValue, 
            new_val: T::NFTValue 
        },
        /// NFT was removed. [account, last_value]
        NFTRemoved { 
            who: T::AccountId, 
            val: T::NFTValue 
        },
    }

    /// Errors that can occur in the NFT Map pallet
    #[pallet::error]
    pub enum Error<T> {
        /// NFT already exists for this account
        NFTAlreadyExists,
        /// NFT does not exist for this account
        NFTNotFound,
        /// Invalid NFT value provided
        InvalidNFTValue,
        /// NFT value below minimum
        ValueTooLow,
        /// NFT value above maximum
        ValueTooHigh,
    }

    /// Storage map from account ID to NFT value
    #[pallet::storage]
    #[pallet::getter(fn nfts)]
    pub type NFTs<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::NFTValue>;

    /// Genesis configuration for the NFT Map pallet
    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub nft_mappers: Vec<(T::AccountId, T::NFTValue)>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (account, value) in &self.nft_mappers {
                NFTs::<T>::insert(account, *value);
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new NFT for an account
        #[pallet::weight({
            let s = T::DbWeight::get();
            s.reads_writes(1, 1)
        })]
        #[pallet::call_index(0)]
        pub fn add_nft(
            origin: OriginFor<T>, 
            account: T::AccountId, 
            val: T::NFTValue
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer
            let _sender = ensure_root(origin)?;
            
            // Value validation
            let value_u32: u32 = val.into();
            ensure!(value_u32 >= T::MinValue::get(), Error::<T>::ValueTooLow);
            ensure!(value_u32 <= T::MaxValue::get(), Error::<T>::ValueTooHigh);
            
            // Verify that the account doesn't already have an NFT
            ensure!(!NFTs::<T>::contains_key(&account), Error::<T>::NFTAlreadyExists);
            
            // Store the NFT
            NFTs::<T>::insert(&account, val);
            
            // Emit event
            Self::deposit_event(Event::NFTAdded { 
                who: account, 
                val, 
                when: frame_system::Pallet::<T>::block_number() 
            });
            
            Ok(())
        }

        /// Update an existing NFT's value
        #[pallet::weight({
            let s = T::DbWeight::get();
            s.reads_writes(1, 1)
        })]
        #[pallet::call_index(1)]
        pub fn update_nft(
            origin: OriginFor<T>, 
            account: T::AccountId, 
            val: T::NFTValue
        ) -> DispatchResult {
            let _sender = ensure_root(origin)?;
            
            // Value validation
            let value_u32: u32 = val.into();
            ensure!(value_u32 >= T::MinValue::get(), Error::<T>::ValueTooLow);
            ensure!(value_u32 <= T::MaxValue::get(), Error::<T>::ValueTooHigh);
            
            // Get old value and ensure NFT exists
            let old_val = NFTs::<T>::get(&account).ok_or(Error::<T>::NFTNotFound)?;
            
            // Update storage
            NFTs::<T>::insert(&account, val);
            
            // Emit event
            Self::deposit_event(Event::NFTUpdated { 
                who: account, 
                old_val, 
                new_val: val 
            });
            
            Ok(())
        }

        /// Remove an NFT
        #[pallet::weight({
            let s = T::DbWeight::get();
            s.reads_writes(1, 1)
        })]
        #[pallet::call_index(2)]
        pub fn delete_nft(
            origin: OriginFor<T>, 
            account: T::AccountId
        ) -> DispatchResult {
            let _sender = ensure_root(origin)?;
            
            // Get value and ensure NFT exists
            let val = NFTs::<T>::get(&account).ok_or(Error::<T>::NFTNotFound)?;
            
            // Remove from storage
            NFTs::<T>::remove(&account);
            
            // Emit event
            Self::deposit_event(Event::NFTRemoved { 
                who: account,
                val
            });
            
            Ok(())
        }
    }
}

pub mod weights {
    use frame_support::{weights::Weight, traits::Get};
    use sp_std::marker::PhantomData;

    /// Weight functions needed for pallet_nft_map.
    pub trait WeightInfo {
        fn add_nft() -> Weight;
        fn update_nft() -> Weight;
        fn delete_nft() -> Weight;
    }

    /// Weights for pallet_nft_map using the Substrate node and recommended hardware.
    pub struct SubstrateWeight<T>(PhantomData<T>);
    impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
        fn add_nft() -> Weight {
            Weight::from_parts(10_000, 0)
                .saturating_add(T::DbWeight::get().reads(1))
                .saturating_add(T::DbWeight::get().writes(1))
        }

        fn update_nft() -> Weight {
            Weight::from_parts(10_000, 0)
                .saturating_add(T::DbWeight::get().reads(1))
                .saturating_add(T::DbWeight::get().writes(1))
        }

        fn delete_nft() -> Weight {
            Weight::from_parts(10_000, 0)
                .saturating_add(T::DbWeight::get().reads(1))
                .saturating_add(T::DbWeight::get().writes(1))
        }
    }
}