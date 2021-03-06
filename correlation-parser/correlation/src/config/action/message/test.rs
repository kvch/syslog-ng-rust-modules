// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use config::action::message::MessageActionBuilder;
use message::MessageBuilder;

use conditions::ConditionsBuilder;
use state::State;
use action::Action;

use env_logger;
use std::time::Duration;
use std::collections::VecDeque;
use uuid::Uuid;
use Event;
use Message;
use test_utils::{MockTemplate, BaseContextBuilder};

#[test]
fn test_given_message_action_when_it_is_executed_then_the_additional_values_are_inserted_into_the_generated_message
    () {
    let mut responder = VecDeque::default();
    let _ = env_logger::init();
    let base_context = {
        let conditions = ConditionsBuilder::new(Duration::from_millis(100)).build();
        let uuid = Uuid::new_v4();
        BaseContextBuilder::<Message, MockTemplate>::new(uuid, conditions).name(Some("name".to_owned())).build()
    };
    let state = {
        let messages = vec![MessageBuilder::new("uuid1", "message1").build(),
                            MessageBuilder::new("uuid2", "message2").build()];
        State::with_messages(messages)
    };
    let message_action = MessageActionBuilder::<MockTemplate>::new("uuid", MockTemplate::literal(b"message"))
                                              .pair("key1", MockTemplate::literal(b"value1"))
                                              .pair("key2", MockTemplate::literal(b"value2"))
                                              .build();

    message_action.on_closed(&state, &base_context, &mut responder);
    assert_eq!(1, responder.len());
    let response = responder.get(0).unwrap();
    let message = &response.message;
    assert_eq!(b"value1",
               message.get(b"key1").expect("Failed to get an additional key-value pair from a generated message"));
    assert_eq!(b"value2",
               message.get(b"key2").expect("Failed to get an additional key-value pair from a generated message"));
}

#[test]
fn test_executed_message_action_uses_the_templates() {
    let mut responder = VecDeque::default();
    let uuid_as_str = "2f34112c-6fc8-406b-a6f0-78158ca724b6";
    let uuid = Uuid::parse_str(uuid_as_str).unwrap();
    let base_context = {
        let conditions = ConditionsBuilder::new(Duration::from_millis(100)).build();
        BaseContextBuilder::<Message, MockTemplate>::new(uuid.to_owned(), conditions).name(Some("name".to_owned())).build()
    };
    let state = {
        let messages = vec![MessageBuilder::new("uuid1", "message1").build(),
                            MessageBuilder::new("uuid2", "message2").build()];
        State::with_messages(messages)
    };
    let message_action = MessageActionBuilder::<MockTemplate>::new("uuid", MockTemplate::literal(uuid_as_str.as_bytes()))
                                              .pair("key1", MockTemplate::literal(b"value1"))
                                              .pair("key2", MockTemplate::literal(b"value2"))
                                              .pair("context_id", MockTemplate::context_id())
                                              .pair("context_len", MockTemplate::context_len())
                                              .build();

    message_action.on_closed(&state, &base_context, &mut responder);
    assert_eq!(1, responder.len());
    let response = responder.get(0).unwrap();
    let message = &response.message;
    assert_eq!(uuid_as_str.as_bytes(), message.message());
    assert_eq!(b"value1",
               message.get(b"key1").expect("Failed to get an additional key-value pair from a generated message"));
    assert_eq!(b"value2",
               message.get(b"key2").expect("Failed to get an additional key-value pair from a generated message"));
    assert_eq!(uuid_as_str.as_bytes(), message.get(b"context_id").unwrap());
    assert_eq!(b"2", message.get(b"context_len").unwrap());
}
