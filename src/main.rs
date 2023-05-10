#![allow(unused)]

mod command;
mod parser;

use parser::parse_arg;
use serde::{Deserialize, Serialize};
use std::env;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

fn main() {
    for argument in env::args() {
        parse_arg(argument);
    }
}
