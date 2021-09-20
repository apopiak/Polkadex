//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
// use crate::pallet as PDEXMigration;
use super::pallet::Pallet;
use crate::mock::PDEXMigration;
use crate::mock::{Origin,PDEX};
// use sp_core::H256;
use sp_runtime::testing::H256;
use pallet::{Config,Call};
use super::*;
use sp_runtime::traits::Saturating;
use sp_runtime::traits::BlockNumberProvider;
use frame_support::{
	traits::Get,
};

pub use frame_benchmarking::{
	account, benchmarks, impl_benchmark_test_suite, whitelist_account,whitelisted_caller,
};
use frame_system::RawOrigin;
use frame_support::assert_ok;
#[allow(unused)]

benchmarks! {
    set_migration_operational_status {

    }: _(RawOrigin::Root, true)

    set_relayer_status {
        let relayer = account("relayer",0,0);
    }: _ (RawOrigin::Root, relayer, true)

    mint {
        let relayer1 = account("relayer1",0,0);
        let relayer2  = account("relayer2",0,0);
        let relayer3 = account("relayer3",0,0);
        let beneficiary = whitelisted_caller();
        assert_ok!(PDEXMigration::set_migration_operational_status(Origin::root(),true));
        // Register relayers
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer1,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer2,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer3,true));

        assert_ok!(PDEXMigration::mint(Origin::signed(relayer1), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer2), beneficiary,100*PDEX,H256::zero()));

        let beneficiary: T::AccountId = whitelisted_caller();
        let eth_hash: T::Hash = T::Hash::default();
        let relayer3: T::AccountId = account("relayer3",0,0);
        let amount: T::Balance = <T as pallet_balances::Config>::ExistentialDeposit::get().saturating_mul(100u32.into());
    }: _(RawOrigin::Signed(relayer3),beneficiary,amount,eth_hash)


    unlock {
        let relayer1 = account("relayer1",0,0);
        let relayer2  = account("relayer2",0,0);
        let relayer3 = account("relayer3",0,0);
        let beneficiary = whitelisted_caller();
        assert_ok!(PDEXMigration::set_migration_operational_status(Origin::root(),true));
        // Register relayers
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer1,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer2,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer3,true));

        assert_ok!(PDEXMigration::mint(Origin::signed(relayer1), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer2), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer3), beneficiary,100*PDEX,H256::zero()));

        frame_system::Pallet::<T>::set_block_number(frame_system::Pallet::<T>::current_block_number()+T::LockPeriod::get());

        let beneficiary: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(beneficiary))

    remove_minted_tokens {
        let relayer1 = account("relayer1",0,0);
        let relayer2  = account("relayer2",0,0);
        let relayer3 = account("relayer3",0,0);
        let beneficiary = whitelisted_caller();
        assert_ok!(PDEXMigration::set_migration_operational_status(Origin::root(),true));
        // Register relayers
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer1,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer2,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer3,true));

        assert_ok!(PDEXMigration::mint(Origin::signed(relayer1), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer2), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer3), beneficiary,100*PDEX,H256::zero()));

        let beneficiary: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Root,beneficiary)
}

// impl_benchmark_test_suite!(
// 	PDEXMigration,
// 	crate::mock::new_test_ext(),
// 	crate::mock::Test,
// );
