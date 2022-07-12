// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for the module.

use super::*;
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};
use mock::{new_test_ext, Call, Doas, DoasCall, Event as TestEvent, Logger, LoggerCall, Origin, System};

#[test]
fn test_setup_works() {
    // Environment setup, logger storage, and sudo `key` retrieval should work as expected.
    new_test_ext().execute_with(|| {
        assert!(Logger::i32_log().is_empty());
        assert!(Logger::account_log().is_empty());
    });
}

#[test]
fn doas_root_basics() {
    // Configure a default test environment and set the root `key` to 1.
    new_test_ext().execute_with(|| {
        // A privileged function should work when the correct Origin SignedBy(One) is used
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1_000 }));
        assert_ok!(Doas::doas_root(Origin::signed(1), call));
        assert_eq!(Logger::i32_log(), vec![42i32]);

        // A privileged function should not work when the incorrect Origin is used
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1_000 }));
        assert_noop!(Doas::doas_root(Origin::signed(2), call), DispatchError::BadOrigin);
    });
}

#[test]
fn doas_root_emits_events_correctly() {
    new_test_ext().execute_with(|| {
        // Set block number to 1 because events are not emitted on block 0.
        System::set_block_number(1);

        // Should emit event to indicate success when called with the root `key` and `call` is `Ok`.
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1 }));
        assert_ok!(Doas::doas_root(Origin::signed(1), call));
        let expected_event = TestEvent::Doas(Event::DidAsRoot(Ok(())));
        assert!(System::events().iter().any(|a| a.event == expected_event));
    })
}

#[test]
fn doas_root_unchecked_weight_basics() {
    new_test_ext().execute_with(|| {
        // A privileged function should work when `sudo` is passed the root `key` as origin.
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1_000 }));
        assert_ok!(Doas::doas_root_unchecked_weight(Origin::signed(1), call, 1_000));
        assert_eq!(Logger::i32_log(), vec![42i32]);

        // A privileged function should not work when called with a non-root `key`.
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1_000 }));
        assert_noop!(
            Doas::doas_root_unchecked_weight(Origin::signed(2), call, 1_000),
            DispatchError::BadOrigin,
        );
        // `I32Log` is unchanged after unsuccessful call.
        assert_eq!(Logger::i32_log(), vec![42i32]);

        // Controls the dispatched weight.
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1 }));
        let doas_root_unchecked_weight_call = DoasCall::doas_root_unchecked_weight { call, weight: 1_000 };
        let info = doas_root_unchecked_weight_call.get_dispatch_info();
        assert_eq!(info.weight, 1_000);
    });
}

#[test]
fn doas_root_unchecked_weight_emits_events_correctly() {
    new_test_ext().execute_with(|| {
        // Set block number to 1 because events are not emitted on block 0.
        System::set_block_number(1);

        // Should emit event to indicate success when called with the root `key` and `call` is `Ok`.
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1 }));
        assert_ok!(Doas::doas_root_unchecked_weight(Origin::signed(1), call, 1_000));
        let expected_event = TestEvent::Doas(Event::DidAsRoot(Ok(())));
        assert!(System::events().iter().any(|a| a.event == expected_event));
    })
}

#[test]
fn doas_basics() {
    new_test_ext().execute_with(|| {
        // A privileged function will not work when passed to `sudo_as`.
        let call = Box::new(Call::Logger(LoggerCall::privileged_i32_log { i: 42, weight: 1_000 }));
        assert_ok!(Doas::doas(Origin::signed(1), 2, call));
        assert!(Logger::i32_log().is_empty());
        assert!(Logger::account_log().is_empty());

        // A non-privileged function should not work when called with a non-root `key`.
        let call = Box::new(Call::Logger(LoggerCall::non_privileged_log { i: 42, weight: 1 }));
        assert_noop!(Doas::doas(Origin::signed(3), 2, call), DispatchError::BadOrigin);

        // A non-privileged function will work when passed to `sudo_as` with the root `key`.
        let call = Box::new(Call::Logger(LoggerCall::non_privileged_log { i: 42, weight: 1 }));
        assert_ok!(Doas::doas(Origin::signed(1), 2, call));
        assert_eq!(Logger::i32_log(), vec![42i32]);
        // The correct user makes the call within `sudo_as`.
        assert_eq!(Logger::account_log(), vec![2]);
    });
}

#[test]
fn doas_emits_events_correctly() {
    new_test_ext().execute_with(|| {
        // Set block number to 1 because events are not emitted on block 0.
        System::set_block_number(1);

        // A non-privileged function will work when passed to `sudo_as` with the root `key`.
        let call = Box::new(Call::Logger(LoggerCall::non_privileged_log { i: 42, weight: 1 }));
        assert_ok!(Doas::doas(Origin::signed(1), 2, call));
        let expected_event = TestEvent::Doas(Event::DidAs(Ok(())));
        assert!(System::events().iter().any(|a| a.event == expected_event));
    });
}
