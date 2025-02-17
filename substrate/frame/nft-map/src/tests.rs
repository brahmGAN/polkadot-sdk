#![cfg(test)]

use crate::{mock::*, pallet::*,Error, Event, weights::WeightInfo};
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_nft_works() {
    new_test_ext().execute_with(|| {
        let account = 2;
        let value = 200;

        // Add a new NFT
        assert_ok!(NFTMap::add_nft(
            RuntimeOrigin::root(),
            account,
            value
        ));

        // Check storage
        assert_eq!(NFTMap::nfts(account), Some(value));

        // Verify event
        System::assert_last_event(Event::NFTAdded { 
            who: account, 
            val: value,
            when: 1 
        }.into());
    });
}

#[test]
fn add_nft_validation_works() {
    new_test_ext().execute_with(|| {
        let account = 2;

        // Try to add NFT with value below minimum
        assert_noop!(
            NFTMap::add_nft(
                RuntimeOrigin::root(),
                account,
                0
            ),
            Error::<Test>::ValueTooLow
        );

        // Try to add NFT with value above maximum
        assert_noop!(
            NFTMap::add_nft(
                RuntimeOrigin::root(),
                account,
                2_000_000
            ),
            Error::<Test>::ValueTooHigh
        );
    });
}

#[test]
fn add_existing_nft_fails() {
    new_test_ext().execute_with(|| {
        let existing_account = 1;  // Account with NFT from genesis

        assert_noop!(
            NFTMap::add_nft(
                RuntimeOrigin::root(),
                existing_account,
                200
            ),
            Error::<Test>::NFTAlreadyExists
        );
    });
}

#[test]
fn update_nft_works() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let old_value = 100;  // From genesis
        let new_value = 150;

        // Update existing NFT
        assert_ok!(NFTMap::update_nft(
            RuntimeOrigin::root(),
            account,
            new_value
        ));

        // Check storage
        assert_eq!(NFTMap::nfts(account), Some(new_value));

        // Verify event
        System::assert_last_event(Event::NFTUpdated { 
            who: account, 
            old_val: old_value,
            new_val: new_value 
        }.into());
    });
}

#[test]
fn update_nft_validation_works() {
    new_test_ext().execute_with(|| {
        let account = 1;

        // Try to update NFT with value below minimum
        assert_noop!(
            NFTMap::update_nft(
                RuntimeOrigin::root(),
                account,
                0
            ),
            Error::<Test>::ValueTooLow
        );

        // Try to update NFT with value above maximum
        assert_noop!(
            NFTMap::update_nft(
                RuntimeOrigin::root(),
                account,
                2_000_000
            ),
            Error::<Test>::ValueTooHigh
        );
    });
}

#[test]
fn update_non_existent_nft_fails() {
    new_test_ext().execute_with(|| {
        let non_existent_account = 999;

        assert_noop!(
            NFTMap::update_nft(
                RuntimeOrigin::root(),
                non_existent_account,
                150
            ),
            Error::<Test>::NFTNotFound
        );
    });
}

#[test]
fn delete_nft_works() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let initial_value = 100;  // From genesis

        // Delete existing NFT
        assert_ok!(NFTMap::delete_nft(
            RuntimeOrigin::root(),
            account
        ));

        // Check storage
        assert_eq!(NFTMap::nfts(account), None);

        // Verify event
        System::assert_last_event(Event::NFTRemoved { 
            who: account,
            val: initial_value
        }.into());
    });
}

#[test]
fn delete_non_existent_nft_fails() {
    new_test_ext().execute_with(|| {
        let non_existent_account = 999;

        assert_noop!(
            NFTMap::delete_nft(
                RuntimeOrigin::root(),
                non_existent_account
            ),
            Error::<Test>::NFTNotFound
        );
    });
}

#[test]
fn authorization_checks_work() {
    new_test_ext().execute_with(|| {
        let account = 2;
        let non_root_account = 1;

        // Non-root account cannot add NFT
        assert_noop!(
            NFTMap::add_nft(
                RuntimeOrigin::signed(non_root_account),
                account,
                200
            ),
            sp_runtime::DispatchError::BadOrigin
        );

        // Add NFT as root
        assert_ok!(NFTMap::add_nft(
            RuntimeOrigin::root(),
            account,
            200
        ));

        // Non-root account cannot update NFT
        assert_noop!(
            NFTMap::update_nft(
                RuntimeOrigin::signed(non_root_account),
                account,
                300
            ),
            sp_runtime::DispatchError::BadOrigin
        );

        // Non-root account cannot delete NFT
        assert_noop!(
            NFTMap::delete_nft(
                RuntimeOrigin::signed(non_root_account),
                account
            ),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}