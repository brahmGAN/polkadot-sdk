#![cfg(test)]

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Hash;

#[test]
fn queen_registration_works() {
    new_test_ext().execute_with(|| {
        // Account with enough balance to stake
        let queen_account = 1;
        let stake_amount = 500;

        assert_ok!(Queen::register_queen(
            RuntimeOrigin::signed(queen_account),
            stake_amount
        ));

        // Check event was emitted
        System::assert_last_event(Event::QueenRegistered { 
            queen: queen_account,
            stake: stake_amount 
        }.into());

        // Verify queen exists
        assert!(crate::Queens::<Test>::contains_key(queen_account));
    });
}

#[test]
fn registration_fails_insufficient_stake() {
    new_test_ext().execute_with(|| {
        let queen_account = 1;
        let low_stake = MinStake::get() - 1;

        assert_noop!(
            Queen::register_queen(
                RuntimeOrigin::signed(queen_account),
                low_stake
            ),
            Error::<Test>::InsufficientStake
        );
    });
}

#[test]
fn duplicate_registration_fails() {
    new_test_ext().execute_with(|| {
        let queen_account = 1;
        let stake_amount = 500;

        // First registration succeeds
        assert_ok!(Queen::register_queen(
            RuntimeOrigin::signed(queen_account),
            stake_amount
        ));

        // Second registration fails
        assert_noop!(
            Queen::register_queen(
                RuntimeOrigin::signed(queen_account),
                stake_amount
            ),
            Error::<Test>::AlreadyQueen
        );
    });
}