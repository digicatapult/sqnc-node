use super::*;

#[test]
fn schedule_before_first_call() {
    new_test_ext().execute_with(|| {
        SymmetricKey::on_initialize(System::block_number());

        let init_key = (0..32).collect::<Vec<u8>>();
        SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

        for _bn in 1..2 {
            System::set_block_number(System::block_number() + 1);
            Scheduler::on_initialize(System::block_number());
        }

        assert_eq!(SymmetricKey::key(), init_key.clone());
    });
}

#[test]
fn schedule_after_schedule_period() {
    new_test_ext().execute_with(|| {
        SymmetricKey::on_initialize(System::block_number());

        let init_key = (0..32).collect::<Vec<u8>>();
        let new_key = vec![
            83, 89, 77, 77, 69, 84, 82, 73, 67, 95, 75, 69, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

        for _bn in 1..5 {
            System::set_block_number(System::block_number() + 1);
            Scheduler::on_initialize(System::block_number());
        }

        assert_eq!(SymmetricKey::key(), new_key);
        assert_eq!(
            System::events().iter().rev().nth(1).unwrap().event,
            Event::pallet_symmetric_key(pallet_symmetric_key::Event::UpdateKey(new_key)),
        )
    });
}
