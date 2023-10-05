use codec::MaxEncodedLen;
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo, Clone, PartialEq)]
pub struct GraveyardState {
    pub(crate) start_index: u64,
    pub(crate) end_index: u64,
}
