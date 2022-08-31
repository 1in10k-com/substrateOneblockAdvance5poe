//aa8,代表不使用std标签，就一定使用no_std
#![cfg_attr(not(feature = "std"), no_std)]

//aa9,导出poe模块定义的内容，注意，并不是用其它模块的内容，而是让外部用此模块的内容，mmmi。
// add220830, 以上可能理解错误，“当外部的模块项 A 被引入到当前模块中时，它的可见性自动被设置为私有的，如果你希望允许其它外部代码引用我们的模块项 A，那么可以对它进行再导出” https://course.rs/basic/crate-module/use.html
pub use pallet::*;

// aa40,2656,引入测试需要的对应模块，用test标签表示只有是test时才会引入这些模块。mmmi
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//aa10，l2，定义poe模块，放到模块空间pallet里，需要用到宏
#[frame_support::pallet]
pub mod pallet {
	// aa11,引入预定义的一些依赖，应该就是之前cargotoml里的frame_support内容。
	use frame_support::pallet_prelude::*;
	// aa12，此依赖包含了ensuresand，ensurenone？？？？这样方便签名验证的方法
	use frame_system::pallet_prelude::*;
	// aa13，包含vector集合类型
	use sp_std::prelude::*;

	// aa14,l2，引入依赖后定义模块配置接口，此模块名叫Config，继承自frame_system::Config，这样前者就拥有了后者里定义的一些数据类型，如blocknumber，hash，accountid
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// aa16，此内容可以在前端显示
		// add220830，https://course.rs/basic/comment.html，这是文档注释
		/// The maximum length of claim that can be added.
		// aa15，添加关联类型，此为存证最大长度限制。通常链上只存储原始内容hash值，其长度固定。这里是get接口mmma定义的u32整数类型，iiic，因为是常量，所以需要使用如下宏来声明其为链上的常量
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		// aa17，在runtime进行配置接口实现时会把runtime定义的event设置在此类型里。iiic，可以先不理解具体含义，应该是固定写法，lllf
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// aa18，0726，定义完配置接口后定义模块所需的结构体Pallet，使用#[pallet::pallet]宏，因为模块会定义自己所需的存储项，
	// 所以需要另一个generate-store宏来帮助生成包含所有存储项的traits-store这样一个接口mmmu。
	#[pallet::pallet]
	//add220830,这一段忘记写了，现在添上
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// aa19,0745，接着定义存储项proofs，是StorageMap类型，在susbtrate里用StorageMap表示一个键值对，这里键是BoundedVec<u8>，也就是u8的一个长度受限的集合。
	// 在新的版本里，runtime不能直接使用vector而是使用BoundedVec这样一个更安全的，长度受限的集合类型，iiic，注意MaxClaimLength这里只能使用get<u32>这样的类型mmmn。
	// value是包含两个元素的tuple，iiic
	// aa28，1545，忘记添加宏里，这里添加，此时成功编译了，之后还需添加一些可调用函数等。
	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	// aa20,0909s,
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
	}

	// aa21,0951s，定义错误信息。定义了这些，可在后面的可调用函数里使用
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	// aa22，1025s，定义一些保留函数（钩子函数？）在区块的不同时期执行，因为存证模块并没有使用保留函数，所以这里留空。
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// aa23，1047s，l4，#[pallet::call]宏定义可调用函数，claim是存证内容
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&bounded_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}
		// aa29,1618,删除存证的可调用函数
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}
		// aa30，1758，转移存证
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _block_number) =
				Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::insert(&bounded_claim, (dest, frame_system::Pallet::<T>::block_number()));

			Ok(().into())
		}
	}
}
