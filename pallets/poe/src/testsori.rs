// aa41，2720，此时仍有很多错误，就不一一写过成了，直接按视频修改代码后，又新增一个测试create_claim_failed_when_claim_already_exist，最后cargo test -p pallet-poe，顺利通过测试，视频1-1完结。

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// aa39，2500，测试创建存证，happy pass？？？？此时使用cargo test -p pallet-poe无效，因为lib.rs里还没有引入子模块tests所需要的模块

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		// bb1, bb开头为220831，2537,因为之前system模块配置里，accountid类型是u64，所以这里用数字类型1代表交易发送方。
		// assert_ok! 是substrate frame_support包里的，不是rust官方包，所以rust官方文档搜不到介绍。应该是代码不panic就没问题。
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		// bb2, 2602，用try_from方法把普通集合类型转换为，BoundedVec集合类型
		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		// bb3，对存储项断言，断言前者对应的值是后者（some，两个元素的tuple，第一个是accountid 1，第二个是区块数）
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		// bb4，2837，创建存证，因为返回值是result类型，所以必须用let * = 在左边接住？？？
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		// bb5，2855，assert_noop也是substrate的宏，功能看插件解释，且里面的调用不会对链上存储进行修改。
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}
