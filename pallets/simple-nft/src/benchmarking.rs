//! Benchmarking setup for pallet-template

use super::*;

use core::convert::TryInto;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, vec, vec::Vec};

#[allow(unused)]
use crate::Module as SimpleNFT;
use crate::Output;

const SEED: u32 = 0;

fn add_nfts<T: Config>(r: u32) -> Result<(), &'static str> {
    let account_id: T::AccountId = account("owner", 0, SEED);
    let mut roles = BTreeMap::new();
    let mut metadata = BTreeMap::new();
    roles.insert(T::RoleKey::default(), account_id.clone());
    metadata.insert(T::TokenMetadataKey::default(), T::TokenMetadataValue::default());
    // let _ = T::Currency::make_free_balance_be(&owner, BalanceOf::<T>::max_value());

    let outputs: Vec<_> = (0..r)
        .map(|_| Output {
            roles: roles.clone(),
            metadata: metadata.clone(),
            parent_index: None,
        })
        .collect();
    SimpleNFT::<T>::run_process(RawOrigin::Signed(account_id.clone()).into(), Vec::new(), outputs)?;

    let expected_last_token = nth_token_id::<T>(r)?;

    assert_eq!(LastToken::<T>::get(), expected_last_token);
    assert_eq!(TokensById::<T>::iter_values().collect::<Vec<_>>().len(), r as usize);
    Ok(())
}

fn mk_inputs<T: Config>(i: u32) -> Result<Vec<T::TokenId>, &'static str> {
    let inputs = (0..i).fold(Vec::<T::TokenId>::new(), |mut acc, _| {
        acc.push(*acc.last().unwrap_or(&T::TokenId::default()) + One::one());
        acc
    });

    Ok(inputs)
}

fn mk_outputs<T: Config>(
    o: u32,
    inputs_len: u32,
) -> Result<Vec<Output<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>, &'static str> {
    let account_id: T::AccountId = account("owner", 0, SEED);
    let mut roles = BTreeMap::new();
    let mut metadata = BTreeMap::new();
    roles.insert(T::RoleKey::default(), account_id.clone());
    metadata.insert(T::TokenMetadataKey::default(), T::TokenMetadataValue::default());
    let outputs = (0..o)
        .map(|output_index| Output {
            roles: roles.clone(),
            metadata: metadata.clone(),
            parent_index: valid_parent_index(inputs_len, output_index),
        })
        .collect::<Vec<_>>();

    Ok(outputs)
}

fn valid_parent_index(input_len: u32, output_count: u32) -> Option<u32> {
    if input_len > 0 && output_count < input_len {
        Some(output_count)
    } else {
        None
    }
}

fn nth_token_id<T: Config>(iteration: u32) -> Result<T::TokenId, &'static str> {
    let token_id = (0..iteration).fold(T::TokenId::default(), |acc, _| acc + One::one());
    Ok(token_id)
}

benchmarks! {
  run_process {
    let i in 1..10;
    let o in 1..10;

    add_nfts::<T>(i)?;
    let inputs = mk_inputs::<T>(i)?;
    let outputs = mk_outputs::<T>(o, inputs.len().try_into().unwrap())?;
    let caller: T::AccountId = account("owner", 0, SEED);
  }: _(RawOrigin::Signed(caller), inputs, outputs)
  verify {
    assert_eq!(LastToken::<T>::get(), nth_token_id::<T>(i + o)?);
  }
}

impl_benchmark_test_suite!(SimpleNFT, crate::mock::new_test_ext(), crate::mock::Test,);
