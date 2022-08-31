// tests.rs的参考代码，来自：https://github.com/kaichaosun/play-substrate/blob/master/pallets/poe/src/tests.rs

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// 创建存证成功
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

// 当同样的存证已经存在时创建失败
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

// 转移存证成功测试
#[test]
fn transfer_claim_success() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];
		let _ = PoeModule::create_claim(Origin::signed(111), claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(111), claim.clone(), 222));

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((222, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

// 当存证不存在时，转移存证失败
#[test]
fn transfer_claim_failed_because_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

// 当转移存证不是存证拥有者时，转移存证失败。
#[test]
fn transfer_claim_failed_because_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];
		let _ = PoeModule::create_claim(Origin::signed(111), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(222), claim.clone(), 333),
			Error::<Test>::NotClaimOwner
		);
	})
}

// 撤销存证成功
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];
		let _ = PoeModule::create_claim(Origin::signed(111), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(111), claim.clone()));
	})
}

// 当目标存证并不存在时，撤销目标存证失败
#[test]
fn revoke_claim_failed_because_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(12345), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

// 当撤销存证的人不是目标存证拥有者时撤销失败。
#[test]
fn revoke_claim_failed_because_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2345), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}
