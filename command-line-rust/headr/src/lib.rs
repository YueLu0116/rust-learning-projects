use clap::{Command, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config{
    file: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_config() -> MyResult<Config>{


    unimplemented!()
}

fn string2usize(input : &str) ->MyResult<usize>{
    match input.parse::<usize>(){
        Ok(num) if num > 0 => Ok(num),  // usize > 0
        _ => Err(input.into())
    }
}

#[test]
fn test_string2usize(){
    let input = "3";
    assert!(string2usize(input).is_ok());
    assert_eq!(string2usize(input).unwrap(), 3);

    let invalid_input = "foo";
    assert!(string2usize(invalid_input).is_err());

    let zero_input = "0";
    assert!(string2usize(zero_input).is_err());
}

