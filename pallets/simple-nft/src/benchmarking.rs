//! Benchmarking setup for pallet-template

use super::*;

use core::convert::TryInto;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::{BoundedBTreeMap, BoundedVec};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

use crate::output::Output;
#[allow(unused)]
use crate::Pallet as SimpleNFT;

const SEED: u32 = 0;

fn add_nfts<T: Config>(r: u32) -> Result<(), &'static str> {
    let account_id: T::AccountId = account("owner", 0, SEED);
    let mut roles = BoundedBTreeMap::<_, _, _>::new();
    let mut metadata = BoundedBTreeMap::<_, _, _>::new();
    roles.try_insert(T::RoleKey::default(), account_id.clone()).unwrap();
    metadata
        .try_insert(T::TokenMetadataKey::default(), T::TokenMetadataValue::default())
        .unwrap();

    let outputs: BoundedVec<_, T::MaxOutputCount> = (0..r)
        .map(|_| Output {
            roles: roles.clone(),
            metadata: metadata.clone(),
            parent_index: None
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    SimpleNFT::<T>::run_process(
        RawOrigin::Signed(account_id.clone()).into(),
        None,
        BoundedVec::<_, _>::with_max_capacity(),
        outputs
    )?;

    let expected_last_token = nth_token_id::<T>(r)?;

    assert_eq!(LastToken::<T>::get(), expected_last_token);
    assert_eq!(TokensById::<T>::iter_values().collect::<Vec<_>>().len(), r as usize);
    Ok(())
}

fn mk_inputs<T: Config>(i: u32) -> Result<BoundedVec<T::TokenId, T::MaxInputCount>, &'static str> {
    let inputs = (0..i).fold(Vec::<T::TokenId>::new(), |mut acc, _| {
        acc.push(*acc.last().unwrap_or(&T::TokenId::default()) + One::one());
        acc
    });

    Ok(inputs.try_into().unwrap())
}

fn mk_outputs<T: Config>(
    o: u32,
    inputs_len: u32
) -> Result<
    BoundedVec<
        Output<
            T::MaxRoleCount,
            T::AccountId,
            T::RoleKey,
            T::MaxMetadataCount,
            T::TokenMetadataKey,
            T::TokenMetadataValue
        >,
        T::MaxOutputCount
    >,
    &'static str
> {
    let account_id: T::AccountId = account("owner", 0, SEED);
    let mut roles = BoundedBTreeMap::<_, _, _>::new();
    let mut metadata = BoundedBTreeMap::<_, _, _>::new();
    roles.try_insert(T::RoleKey::default(), account_id.clone()).unwrap();
    metadata
        .try_insert(T::TokenMetadataKey::default(), T::TokenMetadataValue::default())
        .unwrap();
    let outputs = (0..o)
        .map(|output_index| Output {
            roles: roles.clone(),
            metadata: metadata.clone(),
            parent_index: valid_parent_index(inputs_len, output_index)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

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
  }: _(RawOrigin::Signed(caller), None, inputs, outputs)
  verify {
    assert_eq!(LastToken::<T>::get(), nth_token_id::<T>(i + o)?);
  }
}

impl_benchmark_test_suite!(SimpleNFT, crate::mock::new_test_ext(), crate::mock::Test,);
