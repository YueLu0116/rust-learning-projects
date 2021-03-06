use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()>{
    for filename in config.files{
        match open(&filename){
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num,line) in file.lines().enumerate(){
                    let disp = line?;
                    if config.number_lines{
                        println!("{:>6}\t{}", line_num+1, &disp);
                    }else if config.number_nonblank_lines {
                        if !disp.is_empty(){
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, &disp);
                        }else{
                            println!();
                        }
                    }else{
                        println!("{}", &disp);
                    }                    
                }
            },
        }
    }
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>>{
    match filename{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Yue Lu <dereklu19960116@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .allow_invalid_utf8(true)
                .value_name("FILE")
                .help("Input file(s)")
                .multiple_occurrences(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}