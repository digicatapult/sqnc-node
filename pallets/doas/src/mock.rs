use super::*;
use crate as doas;
use frame_support::{derive_impl, ord_parameter_types};
use frame_system::EnsureSignedBy;
use sp_io;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

// Logger module to track execution.
#[frame_support::pallet]
pub mod logger {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(*weight)]
        pub fn privileged_i32_log(origin: OriginFor<T>, i: i32, weight: Weight) -> DispatchResultWithPostInfo {
            // Ensure that the `origin` is `Root`.
            ensure_root(origin)?;
            <I32Log<T>>::append(i);
            Self::deposit_event(Event::AppendI32 { value: i, weight });
            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(*weight)]
        pub fn non_privileged_log(origin: OriginFor<T>, i: i32, weight: Weight) -> DispatchResultWithPostInfo {
            // Ensure that the `origin` is some signed account.
            let sender = ensure_signed(origin)?;
            <I32Log<T>>::append(i);
            <AccountLog<T>>::append(sender.clone());
            Self::deposit_event(Event::AppendI32AndAccount {
                sender,
                value: i,
                weight,
            });
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AppendI32 {
            value: i32,
            weight: Weight,
        },
        AppendI32AndAccount {
            sender: T::AccountId,
            value: i32,
            weight: Weight,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn account_log)]
    pub(super) type AccountLog<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn i32_log)]
    pub(super) type I32Log<T> = StorageValue<_, Vec<i32>, ValueQuery>;
}

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Doas: doas::{Pallet, Call, Event<T>},
        Logger: logger::{Pallet, Call, Storage, Event<T>},
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

// Implement the logger module's `Config` on the Test runtime.
impl logger::Config for Test {
    type RuntimeEvent = RuntimeEvent;
}

ord_parameter_types! {
    pub const One: u64 = 1;
    pub const Two: u64 = 2;
}

// Implement the doas module's `Config` on the Test runtime.
impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Call = RuntimeCall;
    type DoasOrigin = EnsureSignedBy<One, u64>;
}

// New types for dispatchable functions.
pub type DoasCall = doas::Call<Test>;
pub type LoggerCall = logger::Call<Test>;

// Build test environment by setting the root `key` for the Genesis.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
