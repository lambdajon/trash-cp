#![allow(unused)]

use crate::parser;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]

pub enum OptionType {
    R,
    I,
}

impl FromStr for OptionType {
    type Err = ();

    fn from_str(input: &str) -> Result<OptionType, Self::Err> {
        match input {
            "R" => Ok(OptionType::R),
            "I" => Ok(OptionType::I),
            str => Err(()),
        }
    }
}

fn is_option(str: &String) -> bool {
    if (str.starts_with("-") && str.len() > 1) {
        let mut is_match = false;

        str.split("").for_each(|ch| {
            if (ch.ne("-") && !ch.is_empty()) {
                let up_char = ch.to_uppercase();

                match OptionType::from_str(&up_char) {
                    Ok(res) => is_match = true,
                    Err(err) => {
                        println!("{:?}", err);
                        is_match = false
                    }
                };
            }
        });
        return is_match;
    }
    return false;
}

pub fn parse_arg(str: String) {
    let res = is_option(&str);
    // TODO: need parse command

    println!("Argument {}", str);
}

#[cfg(test)]
mod parser_tests {
    use super::is_option;

    #[test]
    fn check_is_option() {
        let correct_options: Vec<&str> = vec!["-i", "-R", "-Ri", "-iR"];
        for item in correct_options {
            assert_eq!(is_option(&String::from(item)), true);
        }
    }
}
