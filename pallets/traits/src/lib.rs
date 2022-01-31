#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::Parameter;
use codec::{Decode, Encode};

use frame_support::sp_runtime::traits::AtLeast32Bit;
use sp_std::prelude::*;
use sp_std::collections::btree_map::BTreeMap;


#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ProcessIO<AccountId, RoleKey, TokenMetadataKey, TokenMetadataValue> {
    pub roles: BTreeMap<RoleKey, AccountId>,
    pub metadata: BTreeMap<TokenMetadataKey, TokenMetadataValue>,
    pub parent_index: Option<u32>,
}

pub trait ProcessValidator<A, R, T, V> {
  type ProcessIdentifier: Parameter;
  type ProcessVersion: Parameter + AtLeast32Bit;

  fn validate_process(id: Self::ProcessIdentifier, version: Self::ProcessVersion, sender: A, inputs: &Vec<ProcessIO<A, R, T, V>>, outputs: &Vec<ProcessIO<A, R, T, V>>) -> bool;
}

impl<A, R, T, V> ProcessValidator<A, R, T, V> for () {
  type ProcessIdentifier = ();
  type ProcessVersion = u32;

  fn validate_process(_id: Self::ProcessIdentifier, _version: Self::ProcessVersion, _sender: A, _inputs: &Vec<ProcessIO<A, R, T, V>>, _outputs: &Vec<ProcessIO<A, R, T, V>>) -> bool {
      true
  }
}
