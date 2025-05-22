use super::*;
use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};
use sp_runtime::Weight;

/// The log target.
const TARGET: &'static str = "runtime::symmetric-key::migration";

pub mod v2 {
    use super::*;

    /// Migrate the symmetric-key pallet from V1 to V2.
    pub struct MigrateToV2<T>(core::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 1 {
                log::warn!(
                  target: TARGET,
                  "skipping v1 to v2 migration: executed on wrong storage version. Expected version 1, found {:?}",
                  version,
                );
                return T::DbWeight::get().reads(1);
            }

            Pallet::<T>::migrate_v1_to_v2() + T::DbWeight::get().reads(1)
        }
    }
}

impl<T: Config> Pallet<T> {
    fn migrate_v1_to_v2() -> Weight {
        let mut count = 0u64;
        <ProcessModel<T>>::translate_values(
            |old_value: ProcessOld<
                T::RoleKey,
                T::TokenMetadataKey,
                T::TokenMetadataValue,
                T::TokenMetadataValueDiscriminator,
                T::MaxProcessProgramLength,
            >| {
                count += 1;
                Some(old_value.into())
            },
        );

        StorageVersion::new(2).put::<Pallet<T>>();
        T::DbWeight::get().reads(count) + T::DbWeight::get().writes(count + 1)
    }
}

#[derive(Encode, Decode, Debug, Clone, MaxEncodedLen, TypeInfo, PartialEq)]
pub enum RestrictionOld<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> {
    None,
    Fail,
    SenderHasInputRole {
        index: u32,
        role_key: RoleKey,
    },
    SenderHasOutputRole {
        index: u32,
        role_key: RoleKey,
    },
    OutputHasRole {
        index: u32,
        role_key: RoleKey,
    },
    OutputHasMetadata {
        index: u32,
        metadata_key: TokenMetadataKey,
    },
    InputHasRole {
        index: u32,
        role_key: RoleKey,
    },
    InputHasMetadata {
        index: u32,
        metadata_key: TokenMetadataKey,
    },
    MatchInputOutputRole {
        input_index: u32,
        input_role_key: RoleKey,
        output_index: u32,
        output_role_key: RoleKey,
    },
    MatchInputOutputMetadataValue {
        input_index: u32,
        input_metadata_key: TokenMetadataKey,
        output_index: u32,
        output_metadata_key: TokenMetadataKey,
    },
    MatchInputIdOutputMetadataValue {
        input_index: u32,
        output_index: u32,
        output_metadata_key: TokenMetadataKey,
    },
    FixedNumberOfInputs {
        num_inputs: u32,
    },
    FixedNumberOfOutputs {
        num_outputs: u32,
    },
    FixedInputMetadataValue {
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value: TokenMetadataValue,
    },
    FixedOutputMetadataValue {
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value: TokenMetadataValue,
    },
    FixedOutputMetadataValueType {
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value_type: TokenMetadataValueDiscriminator,
    },
    FixedInputMetadataValueType {
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value_type: TokenMetadataValueDiscriminator,
    },
}

impl<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>
    Into<Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>>
    for RestrictionOld<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>
where
    RoleKey: Parameter + Default + Ord,
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue>,
{
    fn into(self) -> Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> {
        match self {
            RestrictionOld::None => Restriction::None,
            RestrictionOld::Fail => Restriction::Fail,
            RestrictionOld::SenderHasInputRole { index, role_key } => Restriction::SenderHasArgRole {
                arg_type: ArgType::Input,
                index,
                role_key,
            },
            RestrictionOld::SenderHasOutputRole { index, role_key } => Restriction::SenderHasArgRole {
                arg_type: ArgType::Output,
                index,
                role_key,
            },
            RestrictionOld::OutputHasRole { index, role_key } => Restriction::ArgHasRole {
                arg_type: ArgType::Output,
                index,
                role_key,
            },
            RestrictionOld::OutputHasMetadata { index, metadata_key } => Restriction::ArgHasMetadata {
                arg_type: ArgType::Output,
                index,
                metadata_key,
            },
            RestrictionOld::InputHasRole { index, role_key } => Restriction::ArgHasRole {
                arg_type: ArgType::Input,
                index,
                role_key,
            },
            RestrictionOld::InputHasMetadata { index, metadata_key } => Restriction::ArgHasMetadata {
                arg_type: ArgType::Input,
                index,
                metadata_key,
            },
            RestrictionOld::MatchInputOutputRole {
                input_index,
                input_role_key,
                output_index,
                output_role_key,
            } => Restriction::MatchArgsRole {
                left_arg_type: ArgType::Input,
                left_index: input_index,
                left_role_key: input_role_key,
                right_arg_type: ArgType::Output,
                right_index: output_index,
                right_role_key: output_role_key,
            },
            RestrictionOld::MatchInputOutputMetadataValue {
                input_index,
                input_metadata_key,
                output_index,
                output_metadata_key,
            } => Restriction::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                left_index: input_index,
                left_metadata_key: input_metadata_key,
                right_arg_type: ArgType::Output,
                right_index: output_index,
                right_metadata_key: output_metadata_key,
            },
            RestrictionOld::MatchInputIdOutputMetadataValue {
                input_index,
                output_index,
                output_metadata_key,
            } => Restriction::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                left_index: input_index,
                right_arg_type: ArgType::Output,
                right_index: output_index,
                right_metadata_key: output_metadata_key,
            },
            RestrictionOld::FixedNumberOfInputs { num_inputs } => Restriction::FixedArgCount {
                arg_type: ArgType::Input,
                count: num_inputs,
            },
            RestrictionOld::FixedNumberOfOutputs { num_outputs } => Restriction::FixedArgCount {
                arg_type: ArgType::Output,
                count: num_outputs,
            },
            RestrictionOld::FixedInputMetadataValue {
                index,
                metadata_key,
                metadata_value,
            } => Restriction::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index,
                metadata_key,
                metadata_value,
            },
            RestrictionOld::FixedOutputMetadataValue {
                index,
                metadata_key,
                metadata_value,
            } => Restriction::FixedArgMetadataValue {
                arg_type: ArgType::Output,
                index,
                metadata_key,
                metadata_value,
            },
            RestrictionOld::FixedOutputMetadataValueType {
                index,
                metadata_key,
                metadata_value_type,
            } => Restriction::FixedArgMetadataValueType {
                arg_type: ArgType::Output,
                index,
                metadata_key,
                metadata_value_type,
            },
            RestrictionOld::FixedInputMetadataValueType {
                index,
                metadata_key,
                metadata_value_type,
            } => Restriction::FixedArgMetadataValueType {
                arg_type: ArgType::Input,
                index,
                metadata_key,
                metadata_value_type,
            },
        }
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxProcessProgramLength))]
pub struct ProcessOld<
    RoleKey,
    TokenMetadataKey,
    TokenMetadataValue,
    TokenMetadataValueDiscriminator,
    MaxProcessProgramLength,
> where
    RoleKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataValue: Parameter + Default + MaxEncodedLen,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue> + MaxEncodedLen,
    MaxProcessProgramLength: Get<u32>,
{
    status: ProcessStatus,
    program: BoundedVec<
        BooleanExpressionSymbolOld<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>,
        MaxProcessProgramLength,
    >,
}

impl<RK, TK, TV, TD, L> Into<Process<RK, TK, TV, TD, L>> for ProcessOld<RK, TK, TV, TD, L>
where
    RK: Parameter + Default + Ord + MaxEncodedLen,
    TK: Parameter + Default + Ord + MaxEncodedLen,
    TV: Parameter + Default + MaxEncodedLen,
    TD: Parameter + Default + From<TV> + MaxEncodedLen,
    L: Get<u32>,
{
    fn into(self) -> Process<RK, TK, TV, TD, L> {
        Process {
            status: self.status,
            program: BoundedVec::truncate_from(
                self.program
                    .into_iter()
                    .map(|e| match e {
                        BooleanExpressionSymbolOld::Op(boolean_operator) => {
                            BooleanExpressionSymbol::Op(boolean_operator)
                        }
                        BooleanExpressionSymbolOld::Restriction(restriction_old) => {
                            BooleanExpressionSymbol::Restriction(restriction_old.into())
                        }
                    })
                    .collect(),
            ),
        }
    }
}

#[derive(Encode, Decode, Debug, Clone, MaxEncodedLen, TypeInfo, PartialEq)]
pub enum BooleanExpressionSymbolOld<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> {
    Op(BooleanOperator),
    Restriction(RestrictionOld<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>),
}

#[cfg(test)]
mod test {
    use super::{v2::MigrateToV2, *};
    use crate::tests::*;
    use frame_support::migration::put_storage_value;
    use frame_support::{pallet_prelude::*, StorageHasher};
    use sp_runtime::bounded_vec;

    type RestrictionOldTest = RestrictionOld<
        <Test as Config>::RoleKey,
        <Test as Config>::TokenMetadataKey,
        <Test as Config>::TokenMetadataValue,
        <Test as Config>::TokenMetadataValueDiscriminator,
    >;

    type RestrictionTest = Restriction<
        <Test as Config>::RoleKey,
        <Test as Config>::TokenMetadataKey,
        <Test as Config>::TokenMetadataValue,
        <Test as Config>::TokenMetadataValueDiscriminator,
    >;

    type ProcessOldTest = ProcessOld<
        <Test as Config>::RoleKey,
        <Test as Config>::TokenMetadataKey,
        <Test as Config>::TokenMetadataValue,
        <Test as Config>::TokenMetadataValueDiscriminator,
        <Test as Config>::MaxProcessProgramLength,
    >;

    type ProcessTest = Process<
        <Test as Config>::RoleKey,
        <Test as Config>::TokenMetadataKey,
        <Test as Config>::TokenMetadataValue,
        <Test as Config>::TokenMetadataValueDiscriminator,
        <Test as Config>::MaxProcessProgramLength,
    >;

    fn get_expected_weight<T: Config>(count: u64) -> Weight {
        let read_count = count + 1;
        let write_count = match count {
            0 => 0,
            count => count + 1,
        };
        T::DbWeight::get().reads(read_count) + T::DbWeight::get().writes(write_count)
    }

    fn get_process_model_key_hash(id: ProcessIdentifier, version: u32) -> Vec<u8> {
        let key1_hashed = id.using_encoded(Blake2_128Concat::hash);
        let key2_hashed = version.using_encoded(Blake2_128Concat::hash);

        let mut final_key = Vec::with_capacity(key1_hashed.len() + key2_hashed.len());

        final_key.extend_from_slice(key1_hashed.as_ref());
        final_key.extend_from_slice(key2_hashed.as_ref());

        final_key
    }

    #[test]
    fn migration_v1_to_v2_works_with_no_entries() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<ProcessValidation>();

            // do the runtime upgrade
            let weight = MigrateToV2::<Test>::on_runtime_upgrade();

            assert_eq!(weight, get_expected_weight::<Test>(0));
        })
    }

    #[test]
    fn migration_v1_to_v2_works_with_single_entry() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<ProcessValidation>();

            put_storage_value(
                b"ProcessValidation",
                b"ProcessModel",
                get_process_model_key_hash(ProcessIdentifier::A, 1).as_slice(),
                ProcessOldTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![
                        BooleanExpressionSymbolOld::Restriction(RestrictionOld::Fail),
                        BooleanExpressionSymbolOld::Restriction(RestrictionOld::None),
                        BooleanExpressionSymbolOld::Op(BooleanOperator::Or)
                    ],
                },
            );

            // do the runtime upgrade
            let weight = MigrateToV2::<Test>::on_runtime_upgrade();
            assert_eq!(weight, get_expected_weight::<Test>(1));

            let value = <ProcessModel<Test>>::get(ProcessIdentifier::A, 1);
            assert_eq!(
                value,
                ProcessTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![
                        BooleanExpressionSymbol::Restriction(Restriction::Fail),
                        BooleanExpressionSymbol::Restriction(Restriction::None),
                        BooleanExpressionSymbol::Op(BooleanOperator::Or)
                    ]
                }
            );

            let storage_version = StorageVersion::get::<Pallet<Test>>();
            assert_eq!(storage_version, StorageVersion::new(2));
        })
    }

    #[test]
    fn migration_v1_to_v2_works_with_multiple_entries() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<ProcessValidation>();

            put_storage_value(
                b"ProcessValidation",
                b"ProcessModel",
                get_process_model_key_hash(ProcessIdentifier::A, 1).as_slice(),
                ProcessOldTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![
                        BooleanExpressionSymbolOld::Restriction(RestrictionOld::Fail),
                        BooleanExpressionSymbolOld::Restriction(RestrictionOld::None),
                        BooleanExpressionSymbolOld::Op(BooleanOperator::Or)
                    ],
                },
            );
            put_storage_value(
                b"ProcessValidation",
                b"ProcessModel",
                get_process_model_key_hash(ProcessIdentifier::B, 1).as_slice(),
                ProcessOldTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![
                        BooleanExpressionSymbolOld::Restriction(RestrictionOld::None),
                        BooleanExpressionSymbolOld::Restriction(RestrictionOld::Fail),
                        BooleanExpressionSymbolOld::Op(BooleanOperator::Xor)
                    ],
                },
            );

            // do the runtime upgrade
            let weight = MigrateToV2::<Test>::on_runtime_upgrade();
            assert_eq!(weight, get_expected_weight::<Test>(2));

            let value = <ProcessModel<Test>>::get(ProcessIdentifier::A, 1);
            assert_eq!(
                value,
                ProcessTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![
                        BooleanExpressionSymbol::Restriction(Restriction::Fail),
                        BooleanExpressionSymbol::Restriction(Restriction::None),
                        BooleanExpressionSymbol::Op(BooleanOperator::Or)
                    ]
                }
            );
            let value = <ProcessModel<Test>>::get(ProcessIdentifier::B, 1);
            assert_eq!(
                value,
                ProcessTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![
                        BooleanExpressionSymbol::Restriction(Restriction::None),
                        BooleanExpressionSymbol::Restriction(Restriction::Fail),
                        BooleanExpressionSymbol::Op(BooleanOperator::Xor)
                    ]
                }
            );

            let storage_version = StorageVersion::get::<Pallet<Test>>();
            assert_eq!(storage_version, StorageVersion::new(2));
        })
    }

    fn migration_v1_to_v2_by_restriction(old: RestrictionOldTest, new: RestrictionTest) {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<ProcessValidation>();

            put_storage_value(
                b"ProcessValidation",
                b"ProcessModel",
                get_process_model_key_hash(ProcessIdentifier::A, 1).as_slice(),
                ProcessOldTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![BooleanExpressionSymbolOld::Restriction(old)],
                },
            );

            // do the runtime upgrade
            let weight = MigrateToV2::<Test>::on_runtime_upgrade();
            assert_eq!(weight, get_expected_weight::<Test>(1));

            let value = <ProcessModel<Test>>::get(ProcessIdentifier::A, 1);
            assert_eq!(
                value,
                ProcessTest {
                    status: ProcessStatus::Enabled,
                    program: bounded_vec![BooleanExpressionSymbol::Restriction(new)]
                }
            );

            let storage_version = StorageVersion::get::<Pallet<Test>>();
            assert_eq!(storage_version, StorageVersion::new(2));
        })
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_fail() {
        migration_v1_to_v2_by_restriction(RestrictionOld::Fail, Restriction::Fail);
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_none() {
        migration_v1_to_v2_by_restriction(RestrictionOld::None, Restriction::None);
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_fixed_input_metadata_value() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::FixedInputMetadataValue {
                index: 1,
                metadata_key: 2,
                metadata_value: 3,
            },
            Restriction::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 2,
                metadata_value: 3,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_fixed_input_metadata_value_type() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::FixedInputMetadataValueType {
                index: 1,
                metadata_key: 2,
                metadata_value_type: TokenMetadataValueDiscriminator { value: 3 },
            },
            Restriction::FixedArgMetadataValueType {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 2,
                metadata_value_type: TokenMetadataValueDiscriminator { value: 3 },
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_fixed_number_of_inputs() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::FixedNumberOfInputs { num_inputs: 4 },
            Restriction::FixedArgCount {
                arg_type: ArgType::Input,
                count: 4,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_fixed_number_of_outputs() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::FixedNumberOfOutputs { num_outputs: 4 },
            Restriction::FixedArgCount {
                arg_type: ArgType::Output,
                count: 4,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_fixed_output_metadata_value() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::FixedOutputMetadataValue {
                index: 1,
                metadata_key: 2,
                metadata_value: 3,
            },
            Restriction::FixedArgMetadataValue {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 2,
                metadata_value: 3,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_fixed_output_metadata_value_type() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::FixedOutputMetadataValueType {
                index: 1,
                metadata_key: 2,
                metadata_value_type: TokenMetadataValueDiscriminator { value: 3 },
            },
            Restriction::FixedArgMetadataValueType {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 2,
                metadata_value_type: TokenMetadataValueDiscriminator { value: 3 },
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_input_has_metadata() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::InputHasMetadata {
                index: 1,
                metadata_key: 2,
            },
            Restriction::ArgHasMetadata {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 2,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_input_has_role() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::InputHasRole { index: 1, role_key: 2 },
            Restriction::ArgHasRole {
                arg_type: ArgType::Input,
                index: 1,
                role_key: 2,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_output_has_metadata() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::OutputHasMetadata {
                index: 1,
                metadata_key: 2,
            },
            Restriction::ArgHasMetadata {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 2,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_output_has_role() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::OutputHasRole { index: 1, role_key: 2 },
            Restriction::ArgHasRole {
                arg_type: ArgType::Output,
                index: 1,
                role_key: 2,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_match_arg_id_to_metadata_value() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::MatchInputIdOutputMetadataValue {
                input_index: 1,
                output_index: 2,
                output_metadata_key: 3,
            },
            Restriction::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                left_index: 1,
                right_arg_type: ArgType::Output,
                right_index: 2,
                right_metadata_key: 3,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_match_input_output_metadata_value() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::MatchInputOutputMetadataValue {
                input_index: 1,
                input_metadata_key: 2,
                output_index: 3,
                output_metadata_key: 4,
            },
            Restriction::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                left_index: 1,
                left_metadata_key: 2,
                right_arg_type: ArgType::Output,
                right_index: 3,
                right_metadata_key: 4,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_match_input_output_roles() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::MatchInputOutputRole {
                input_index: 1,
                input_role_key: 2,
                output_index: 3,
                output_role_key: 4,
            },
            Restriction::MatchArgsRole {
                left_arg_type: ArgType::Input,
                left_index: 1,
                left_role_key: 2,
                right_arg_type: ArgType::Output,
                right_index: 3,
                right_role_key: 4,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_sender_has_input_role() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::SenderHasInputRole { index: 1, role_key: 2 },
            Restriction::SenderHasArgRole {
                arg_type: ArgType::Input,
                index: 1,
                role_key: 2,
            },
        );
    }

    #[test]
    fn migration_v1_to_v2_works_with_restriction_sender_has_output_role() {
        migration_v1_to_v2_by_restriction(
            RestrictionOld::SenderHasOutputRole { index: 1, role_key: 2 },
            Restriction::SenderHasArgRole {
                arg_type: ArgType::Output,
                index: 1,
                role_key: 2,
            },
        );
    }
}
