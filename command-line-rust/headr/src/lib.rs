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
