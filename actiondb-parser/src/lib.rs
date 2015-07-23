#[macro_use]
extern crate log;

#[macro_use]
extern crate syslog_ng_sys;
extern crate actiondb;

use std::borrow::Borrow;
use std::clone;

use actiondb::matcher::Matcher;
use actiondb::matcher::Factory;

use syslog_ng_sys::{RustParser,
                    LogMessage};

pub struct ActiondbParser {
    matcher: Option<Box<Matcher>>
}

impl ActiondbParser {
    pub fn new() -> ActiondbParser {
        debug!("ActiondbParser: new()");
        ActiondbParser{ matcher: None }
    }
}

impl RustParser for ActiondbParser {
    fn process(&self, msg: &mut LogMessage, input: &str) -> bool {
        debug!("ActiondbParser: process(input='{}')", input);
        let parse_result = self.matcher.as_ref().unwrap().parse(input);

        if let Some(kv_pairs) = parse_result {
            debug!("parser matched");
            for &(key, value) in kv_pairs.pairs() {
                msg.set_value(key, value);
            }
            true
        } else {
            false
        }
    }

    fn init(&mut self) -> bool {
        debug!("ActiondbParser: init()");
        if self.matcher.is_none() {
            error!("ActiondbParser: not all required parameters are set");
            false
        } else {
            true
        }
    }

    fn set_option(&mut self, key: String, value: String) {
        debug!("ActiondbParser: set_option(key={}, value={})", &key, &value);

        match key.borrow() {
            "pattern_file" => {
                let matcher = Factory::from_file(&value);

                if matcher.is_ok() {
                    self.matcher = matcher.ok();
                } else {
                    error!("ActiondbParser: failed to set 'pattern_file'");
                }
            },
            _ => {
                debug!("ActiondbParser not supported key: {:?}", key) ;
            }
        };
    }

    fn boxed_clone(&self) -> Box<RustParser> {
        Box::new(self.clone())
    }
}

impl clone::Clone for ActiondbParser {
    fn clone(&self) -> ActiondbParser {
        match self.matcher.as_ref() {
            Option::Some(matcher) => {
                ActiondbParser{
                    matcher: Some(matcher.boxed_clone())
                }
            },
            Option::None => {
                ActiondbParser{
                    matcher: None
                }
            }
        }
    }
}