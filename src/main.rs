#![allow(unused)]

mod parser;

use std::env;
use parser::parse_arg;
use serde::{Serialize, Deserialize};

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

pub fn is_path(str: String) -> bool {

    return false;
}

fn main() {
    
    for argument in env::args() {
        parse_arg(argument);
    }
}
