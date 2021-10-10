use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config)->Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive{
        search_case_sensitive(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }
    Ok(())
}

// best practices: minimize the amount of mutable state to make code clearer
pub fn search_case_sensitive<'a>(query: &str, contents:&'a str)->Vec<&'a str>{
    contents.lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str)->Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}

impl Config{
    // 'static means entire lifetime of the running program
    // About why should add lifetime here, one can refer to this answer
    // https://stackoverflow.com/a/49509702/11100389
    pub fn new(mut args: env::Args)->Result<Config, & 'static str>{
        args.next();

        let query = match args.next(){
            Some(arg)=>arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive,})
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn cast_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn cast_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}