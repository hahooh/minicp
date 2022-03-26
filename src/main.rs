use colored::Colorize;
use std::env;
use std::path;

#[derive(Debug)]
struct Config {
    to: String,
    from: Vec<String>,
}

impl Config {
    fn new(args: env::Args) -> Result<Config, String> {
        let mut from = vec![];
        let mut to = String::new();
        let num_arg = args.len();
        for (index, arg) in args.into_iter().enumerate() {
            let path = path::Path::new(&arg);
            if !path.exists() {
                return Result::Err(format!("path does not exists {}", arg));
            }
            if index + 1 < num_arg && index != 0 {
                from.push(arg);
            } else {
                to = arg;
            }
        }
        return Result::Ok(Config { to, from });
    }
}

fn main() {
    let args = env::args();
    let num_args = args.len();
    if num_args < 2 {
        println!("num args is < 2")
    }

    let config = Config::new(args);
    let config = match config {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{} {}", "error:".red(), err.red());
            std::process::exit(1);
        }
    };

    println!("{:?}", config);
}
