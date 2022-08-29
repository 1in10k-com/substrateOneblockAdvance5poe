//a8,代表不使用std标签，就一定使用no_std
#![cfg_attr(not(feature = "std"), no_std)]

//a9,导出poe模块定义的内容，注意，并不是用其它模块的内容，而是让外部用此模块的内容，mmmi。
pub use pallet::*;

//a10，l2，定义poe模块，放到模块空间pallet里，需要用到宏
#[frame_support::pallet]
pub mod pallet {
	// a11,引入预定义的一些依赖，应该就是之前cargotoml里的frame_support内容。
	use frame_support::pallet_prelude::*;
	// a12，此依赖包含了ensuresand，ensurenone？？？？这样方便签名验证的方法
	use frame_system::pallet_prelude::*;
	// a13，包含vector集合类型
	use sp_std::prelude::*;

	// a14,l2，引入依赖后定义模块配置接口，此模块名叫Config，继承自frame_system::Config，这样前者就拥有了后者里定义的一些数据类型，如blocknumber，hash，accountid
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// a16，此内容可以在前端显示mmma
		/// The maximum length of claim that can be added.
		// a15，添加关联类型，此为存证最大长度限制。通常链上只存储原始内容hash值，其长度固定。这里是get接口mmma定义的u32整数类型，iiic，因为是常量，所以需要使用如下宏来声明其为链上的常量
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		// a17，在runtime进行配置接口实现时会把runtime定义的event设置在此类型里。iiic
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// a18，定义完配置接口后定义模块所需的结构体Pallet，使用#[pallet::pallet]宏，因为模块会定义自己所需的存储项，所以需要另一个generate-store宏来帮助生成包含所有存储项的traits-store这样一个接口mmmu。
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// a19,接着定义存储项proofs，是StorageMap类型，在susbtrate里用StorageMap表示一个键值对，这里键是BoundedVec<u8>，也就是u8的一个长度受限的集合。在新的版本里，runtime不能直接使用vector而是使用BoundedVec这样一个更安全的，长度受限的集合类型，iiic，注意T::*这里只能使用get<u32>这样的类型mmmu。
	// value是包含两个元素的tuple，iiic
	// a28，1545，忘记添加宏里，这里添加，此时成功编译了，之后还需添加一些可调用函数等。
	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	// a20,0909s,
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
	}

	// a21,0951s，定义错误信息。定义了这些，可在后面的可调用函数里使用
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	// a22，1025s，定义一些保留函数（钩子函数？）在区块的不同时期执行，因为存证模块并没有使用保留函数，所以这里留空。
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// a23，1047s，l4，#[pallet::call]宏定义可调用函数，claim是存证内容
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			// a24，1133s，校验交易发送方，交易为签名交易
			let sender = ensure_signed(origin)?;
			// a25，1143s，l5，校验存证内容hash值，是否超过最大长度。并验证claim还未被存储过
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);

			// a25，1235s，i3，插入键值对，之后触发事件，接着返回ok
			Proofs::<T>::insert(
				&bounded_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}
	}
}
