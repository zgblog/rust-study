use std::{env, error::Error, fs, process, result};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) =  run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 使用IO读取文件内容
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 使用迭代器
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

/**
 * 大小写不敏感
 */
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // 读取环境变量
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        println!("{}", case_sensitive);

        Ok(Config {query, filename, case_sensitive})
    }
}