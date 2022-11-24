use super::*;
use crate::binary_expression_tree::BooleanExpressionSymbol;
use crate::binary_expression_tree::BooleanOperator;
use crate::tests::ProcessIdentifier;
use crate::{Process, ProcessModel, ProcessStatus, Restriction, VersionModel};
use frame_support::bounded_vec;

// -- fixtures --
#[allow(dead_code)]
const PROCESS_ID1: ProcessIdentifier = ProcessIdentifier::A;
const PROCESS_ID2: ProcessIdentifier = ProcessIdentifier::B;

use crate::GenesisConfig;

#[test]
fn genesis_with_valid_processes() {
    new_test_ext_with_genesis(GenesisConfig::<Test> {
      processes: vec![(
        PROCESS_ID1,
        bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::Fail)]
      ), (
        PROCESS_ID2,
        bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::None)]
      )]
    }).execute_with(|| {
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 1u32);
        assert_eq!(<ProcessModel<Test>>::get(PROCESS_ID1, 1u32), Process {
          status: ProcessStatus::Enabled,
          program: bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::Fail)]
        });
        assert_eq!(<ProcessModel<Test>>::get(PROCESS_ID2, 1u32), Process {
          status: ProcessStatus::Enabled,
          program: bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::None)]
        });
    });
}

#[test]
#[should_panic]
fn genesis_with_invalid_process() {
    new_test_ext_with_genesis(GenesisConfig::<Test> {
      processes: vec![(
        PROCESS_ID1,
        bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::Fail)]
      ), (
        PROCESS_ID2,
        bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::None), BooleanExpressionSymbol::Op(BooleanOperator::And)]
      )]
    }).execute_with(|| {});
}
