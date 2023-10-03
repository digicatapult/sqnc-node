use frame_support::bounded_vec;

use super::*;
use crate::tests::RuntimeEvent as TestEvent;
use crate::Event;

#[test]
fn schedule_before_first_call() {
    new_test_ext().execute_with(|| {
        SymmetricKey::on_initialize(System::block_number());
        run_to_block(1);

        let init_key: BoundedVec<u8, ConstU32<32>> = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        SymmetricKey::update_key(RuntimeOrigin::root(), init_key.clone()).unwrap();

        run_to_block(3);

        assert_eq!(SymmetricKey::key(), init_key);
    });
}

#[test]
fn schedule_after_first_schedule() {
    new_test_ext().execute_with(|| {
        SymmetricKey::on_initialize(System::block_number());
        run_to_block(1);

        let init_key = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        SymmetricKey::update_key(RuntimeOrigin::root(), init_key).unwrap();

        let new_key = bounded_vec![
            48, 49, 50, 51, 52, 53, 54, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];

        run_to_block(6);

        assert_eq!(SymmetricKey::key(), new_key);
        assert_eq!(
            System::events().iter().rev().nth(2).unwrap().event,
            TestEvent::SymmetricKey(Event::UpdateKey(new_key)),
        )
    });
}

#[test]
fn schedule_after_schedule_period() {
    new_test_ext().execute_with(|| {
        SymmetricKey::on_initialize(System::block_number());
        run_to_block(1);

        let init_key = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        SymmetricKey::update_key(RuntimeOrigin::root(), init_key).unwrap();

        let new_key = bounded_vec![
            88, 89, 90, 91, 92, 93, 94, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        run_to_block(11);

        assert_eq!(SymmetricKey::key(), new_key);

        let events = System::events();
        let last_key_update = &events
            .iter()
            .rev()
            .find(|&x| match x.event {
                TestEvent::SymmetricKey(Event::UpdateKey(_)) => true,
                _ => false,
            })
            .unwrap()
            .event;
        assert_eq!(last_key_update, &TestEvent::SymmetricKey(Event::UpdateKey(new_key)),)
    });
}
