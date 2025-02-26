#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use pallet_subnet::{SubnetInfo, SubnetMetrics};

use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency, Get},
    weights::Weight,
    Blake2_128Concat,
};
use frame_system::pallet_prelude::*;
use sp_runtime::{traits::Hash, Vec};
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;

    #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, TypeInfo, MaxEncodedLen)]
    pub struct QueenInfo<T: Config> {
        pub staked_amount: BalanceOf<T>,
        pub voting_power: u32,
        pub monitored_subnets: BoundedVec<T::Hash, T::MaxMonitoredSubnets>,
        pub last_vote: BlockNumberFor<T>,
    }

    #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, TypeInfo, MaxEncodedLen)]
    pub struct VoteInfo<T: Config> {
        pub subnet_id: T::Hash,
        pub approve: bool,
        pub weight: u32,
        pub expiry: BlockNumberFor<T>,
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    pub trait WeightInfo {
        fn register_queen() -> Weight;
        fn cast_vote() -> Weight;
        // Add other function weights as needed
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_subnet::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        
        #[pallet::constant]
        type MinStake: Get<BalanceOf<Self>>;
        
        #[pallet::constant]
        type MaxMonitoredSubnets: Get<u32>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    pub type Queens<T: Config> = StorageMap
        <_,
        Blake2_128Concat,
        T::AccountId,
        QueenInfo<T>
    >;

    #[pallet::storage]
    pub type Votes<T: Config> = StorageDoubleMap
        <_,
        Blake2_128Concat, T::Hash,      // Subnet ID
        Blake2_128Concat, T::AccountId, // Queen
        VoteInfo<T>
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        QueenRegistered { queen: T::AccountId, stake: BalanceOf<T> },
        VoteCast { queen: T::AccountId, subnet_id: T::Hash, approve: bool },
        StakeIncreased { queen: T::AccountId, amount: BalanceOf<T> },
        SubnetMonitored { queen: T::AccountId, subnet_id: T::Hash },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyQueen,
        NotQueen,
        InsufficientStake,
        TooManySubnets,
        SubnetNotFound,
        AlreadyMonitoring,
        VotingPowerTooLow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::register_queen())]
        pub fn register_queen(
            origin: OriginFor<T>,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!Queens::<T>::contains_key(&who), Error::<T>::AlreadyQueen);
            ensure!(stake >= T::MinStake::get(), Error::<T>::InsufficientStake);

            T::Currency::reserve(&who, stake)?;

            let voting_power = Self::calculate_voting_power(stake);
            let queen_info = QueenInfo {
                staked_amount: stake,
                voting_power,
                monitored_subnets: BoundedVec::default(),
                last_vote: frame_system::Pallet::<T>::block_number(),
            };

            Queens::<T>::insert(&who, queen_info);
            Self::deposit_event(Event::QueenRegistered { queen: who, stake });
            
            Ok(())
        }

        #[pallet::weight(T::WeightInfo::cast_vote())]
        pub fn cast_vote(
            origin: OriginFor<T>,
            subnet_id: T::Hash,
            approve: bool,
        ) -> DispatchResult {
            let queen = ensure_signed(origin)?;
            let queen_info = Queens::<T>::get(&queen).ok_or(Error::<T>::NotQueen)?;
            
            ensure!(
                queen_info.monitored_subnets.contains(&subnet_id),
                Error::<T>::SubnetNotFound
            );

            let vote_info = VoteInfo {
                subnet_id,
                approve,
                weight: queen_info.voting_power,
                expiry: frame_system::Pallet::<T>::block_number() + 100u32.into(),
            };

            Votes::<T>::insert(subnet_id, &queen, vote_info);
            Self::deposit_event(Event::VoteCast { queen, subnet_id, approve });
            
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn calculate_voting_power(stake: BalanceOf<T>) -> u32 {
            // Implement voting power calculation based on stake
            stake.saturated_into::<u32>() / T::MinStake::get().saturated_into::<u32>()
        }
    }
}