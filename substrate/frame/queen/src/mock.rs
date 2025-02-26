#![cfg(test)]

use crate as pallet_queen;
use frame_support::{
    traits::{ConstU16, ConstU32, ConstU64, Currency},
    weights::Weight,
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
use frame_support::parameter_types;

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u64;

// Configure mock runtime
frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        King: pallet_king,
        Subnet: pallet_subnet,
        Queen: pallet_queen,
        Balances: pallet_balances,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
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
    type AccountData = pallet_balances::AccountData<Balance>;
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

parameter_types! {
    pub const ExistentialDeposit: Balance = 1;
    pub const MinStake: Balance = 100;
    pub const MaxMonitoredSubnets: u32 = 10;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type MaxHolds = ();
}

impl pallet_queen::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinStake = MinStake;
    type MaxMonitoredSubnets = MaxMonitoredSubnets;
    type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 1000), (2, 2000), (3, 3000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}