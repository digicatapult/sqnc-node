/*!
Benchmarking setup for pallet-template
*/
use super::*;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::{traits::ConstU32, BoundedBTreeMap, BoundedVec};
use frame_system::{Pallet as System, RawOrigin};
use sp_runtime::traits::Bounded;
use sp_std::convert::TryFrom;
use sp_std::vec::Vec;

use dscp_pallet_traits::{ProcessFullyQualifiedId, ProcessValidator};

use crate::output::Output;
#[allow(unused)]
use crate::Pallet as UtxoNFT;

const SEED: u32 = 0;

type ProcessIdentifier<T> = <<T as Config>::ProcessValidator as ProcessValidator<
    <T as Config>::TokenId,
    <T as frame_system::Config>::AccountId,
    <T as Config>::RoleKey,
    <T as Config>::TokenMetadataKey,
    <T as Config>::TokenMetadataValue,
>>::ProcessIdentifier;

type ProcessVersion<T> = <<T as Config>::ProcessValidator as ProcessValidator<
    <T as Config>::TokenId,
    <T as frame_system::Config>::AccountId,
    <T as Config>::RoleKey,
    <T as Config>::TokenMetadataKey,
    <T as Config>::TokenMetadataValue,
>>::ProcessVersion;

fn add_nfts<T: Config>(r: u32) -> Result<(), &'static str>
where
    ProcessIdentifier<T>: From<BoundedVec<u8, ConstU32<32>>>,
    ProcessVersion<T>: From<u32>,
{
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
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let default_process = BoundedVec::<u8, ConstU32<32>>::try_from("default".as_bytes().to_vec()).unwrap();
    UtxoNFT::<T>::run_process(
        RawOrigin::Signed(account_id.clone()).into(),
        ProcessFullyQualifiedId {
            id: default_process.into(),
            version: 1u32.into(),
        },
        BoundedVec::<_, _>::with_max_capacity(),
        outputs,
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
) -> Result<
    BoundedVec<
        Output<
            T::MaxRoleCount,
            T::AccountId,
            T::RoleKey,
            T::MaxMetadataCount,
            T::TokenMetadataKey,
            T::TokenMetadataValue,
        >,
        T::MaxOutputCount,
    >,
    &'static str,
> {
    let account_id: T::AccountId = account("owner", 0, SEED);
    let mut roles = BoundedBTreeMap::<_, _, _>::new();
    let mut metadata = BoundedBTreeMap::<_, _, _>::new();
    roles.try_insert(T::RoleKey::default(), account_id.clone()).unwrap();
    metadata
        .try_insert(T::TokenMetadataKey::default(), T::TokenMetadataValue::default())
        .unwrap();
    let outputs = (0..o)
        .map(|_| Output {
            roles: roles.clone(),
            metadata: metadata.clone(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Ok(outputs)
}

fn nth_token_id<T: Config>(iteration: u32) -> Result<T::TokenId, &'static str> {
    let token_id = (0..iteration).fold(T::TokenId::default(), |acc, _| acc + One::one());
    Ok(token_id)
}

benchmarks! {
    where_clause { where
        ProcessIdentifier::<T>: From<BoundedVec<u8, ConstU32<32>>>,
        ProcessVersion::<T>: From<u32>
    }

    run_process {
        let i in 1..10;
        let o in 1..10;

        let default_process = BoundedVec::<u8, ConstU32<32>>::try_from("default".as_bytes().to_vec()).unwrap();
        let process = ProcessFullyQualifiedId {
            id: default_process.into(),
            version: 1u32.into()
        };

        add_nfts::<T>(i)?;
        let inputs = mk_inputs::<T>(i)?;
        let outputs = mk_outputs::<T>(o)?;
        let caller: T::AccountId = account("owner", 0, SEED);
    }: _(RawOrigin::Signed(caller), process, inputs, outputs)
    verify {
        assert_eq!(LastToken::<T>::get(), nth_token_id::<T>(i + o)?);
    }

    delete_token {
        let token_id: T::TokenId = 1u32.into();
        let caller: T::AccountId = account("owner", 0, SEED);

        add_nfts::<T>(1)?;
        let inputs = mk_inputs::<T>(1)?;
        let outputs = mk_outputs::<T>(0)?;
        let default_process = BoundedVec::<u8, ConstU32<32>>::try_from("default".as_bytes().to_vec()).unwrap();
        let process = ProcessFullyQualifiedId {
            id: default_process.into(),
            version: 1u32.into()
        };
        UtxoNFT::<T>::run_process(
            RawOrigin::Signed(caller.clone()).into(),
            process,
            inputs,
            outputs
        )?;

        System::<T>::set_block_number(T::BlockNumber::max_value());
        assert_eq!(TokensById::<T>::get(token_id).is_none(), false);
    }: _(RawOrigin::Signed(caller), token_id)
    verify {
        assert_eq!(TokensById::<T>::get(token_id).is_none(), true);
    }
}

impl_benchmark_test_suite!(UtxoNFT, crate::tests::mock::new_test_ext(), crate::tests::mock::Test,);
