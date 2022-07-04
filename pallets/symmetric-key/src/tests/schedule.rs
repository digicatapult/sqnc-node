use frame_support::bounded_vec;

use super::*;
use crate::tests::Event as TestEvent;
use crate::Event;

#[test]
fn schedule_before_first_call() {
    new_test_ext().execute_with(|| {
        SymmetricKey::on_initialize(System::block_number());

        let init_key: BoundedVec<u8, ConstU32<32>> = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

        for _bn in 1..2 {
            System::set_block_number(System::block_number() + 1);
            Scheduler::on_initialize(System::block_number());
        }

        assert_eq!(SymmetricKey::key(), init_key);
    });
}

#[test]
fn schedule_after_schedule_period() {
    new_test_ext().execute_with(|| {
        SymmetricKey::on_initialize(System::block_number());

        let init_key = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        let new_key = bounded_vec![
            83, 89, 77, 77, 69, 84, 82, 73, 67, 95, 75, 69, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
        ];
        SymmetricKey::update_key(Origin::root(), init_key).unwrap();

        for _bn in 1..5 {
            System::set_block_number(System::block_number() + 1);
            Scheduler::on_initialize(System::block_number());
        }

        assert_eq!(SymmetricKey::key(), new_key);
        assert_eq!(
            System::events().iter().rev().nth(1).unwrap().event,
            TestEvent::SymmetricKey(Event::UpdateKey(new_key)),
        )
    });
}
