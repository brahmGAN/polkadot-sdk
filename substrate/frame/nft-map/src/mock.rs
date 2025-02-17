#![cfg(test)]

use crate::{self as pallet_nft_map, weights::WeightInfo};
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64, Everything},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        NFTMap: pallet_nft_map,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeTask = RuntimeTask;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type Nonce = u64;
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

// Mock Weight implementation
pub struct MockWeightInfo;
impl WeightInfo for MockWeightInfo {
    fn add_nft() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
    fn update_nft() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
    fn delete_nft() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
}

parameter_types! {
    pub const MinNFTValue: u32 = 1;
    pub const MaxNFTValue: u32 = 1_000_000;
}

impl pallet_nft_map::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type NFTValue = u32;
    type MinValue = MinNFTValue;
    type MaxValue = MaxNFTValue;
}

// Build genesis storage according to the mock runtime
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into();

    pallet_nft_map::GenesisConfig::<Test> {
        nft_mappers: vec![(1, 100)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}