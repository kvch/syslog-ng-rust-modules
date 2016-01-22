// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use LogMessage;
use LogParser;

pub use proxies::parser::{OptionError, Parser, ParserBuilder};

#[repr(C)]
#[derive(Clone)]
pub struct ParserProxy<B>
    where B: ParserBuilder
{
    pub parser: Option<B::Parser>,
    pub builder: Option<B>,
}

impl<B> ParserProxy<B> where B: ParserBuilder
{
    pub fn new() -> ParserProxy<B> {
        ParserProxy {
            parser: None,
            builder: Some(B::new()),
        }
    }

    pub fn init(&mut self) -> bool {
        let builder = self.builder.take().expect("Called init when builder was not set");
        match builder.build() {
            Ok(parser) => {
                self.parser = Some(parser);
                true
            }
            Err(error) => {
                error!("Error: {:?}", error);
                false
            }
        }
    }

    pub fn set_option(&mut self, name: String, value: String) {
        if self.builder.is_none() {
            self.builder = Some(B::new());
        }

        let builder = self.builder.as_mut().expect("Failed to get builder on a ParserProxy");
        builder.option(name, value);
    }

    pub fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        self.parser
            .as_mut()
            .expect("Called process on a non-existing Rust parser")
            .parse(msg, input)
    }

    pub fn parent(&mut self, parent: LogParser) {
        let builder = self.builder
                          .as_mut()
                          .expect("Failed to get a builder on a new parser proxy instance");
        builder.parent(parent);
    }
}
