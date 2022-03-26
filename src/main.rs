use colored::Colorize;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path;

#[derive(Debug)]
struct Config {
    to: String,
    from: Vec<String>,
}

impl Config {
    fn from_args(args: env::Args) -> Result<Config, String> {
        let mut from = vec![];
        let mut to = String::new();
        let num_arg = args.len();
        for (index, arg) in args.into_iter().enumerate() {
            if index + 1 < num_arg && index != 0 {
                let path = path::Path::new(&arg);
                if !path.exists() {
                    return Result::Err(format!("path does not exists {}", arg));
                }
                if !path.is_file() {
                    return Result::Err(format!("from path is not a file {}", arg));
                }
                from.push(arg);
            } else if index != 0 {
                to = arg;
            }
        }
        if num_arg > 3 {
            let to_path = path::Path::new(&to);
            if !to_path.is_dir() {
                return Result::Err(format!("destination is not a dir"));
            }
        }
        Result::Ok(Config { to, from })
    }
}

fn dump(from: &mut fs::File, to: &mut fs::File) -> Result<usize, String> {
    let mut buf = vec![];
    match from.read_to_end(&mut buf) {
        Ok(n) => n,
        Err(err) => return Result::Err(format!("cannot read. {}", err)),
    };

    match to.write(&buf) {
        Err(err) => Result::Err(format!("write file error {}", err)),
        Ok(n) => Result::Ok(n),
    }
}

fn get_to_file(from: &str, to: &str) -> Result<fs::File, String> {
    let to_path = path::Path::new(&to);
    if to_path.is_dir() {
        let from_path = path::Path::new(&from);
        let from_file_name = match from_path.file_name() {
            Some(n) => match n.to_str() {
                Some(name) => name,
                None => return Result::Err("invalid file name".to_string()),
            },
            None => return Result::Err("file name not found".to_string()),
        };

        return match fs::File::create([&to[..], &from_file_name[..]].concat()) {
            Ok(f) => Result::Ok(f),
            Err(err) => Result::Err(format!("cannot create file {}", err)),
        };
    }

    match fs::File::create(to) {
        Ok(f) => Result::Ok(f),
        Err(err) => Result::Err(format!("cannot create file {}, err {}", to, err)),
    }
}

fn mv(config: Config) -> Result<u8, String> {
    for from in config.from {
        let mut to_file = match get_to_file(&from, &config.to) {
            Ok(f) => f,
            Err(err) => return Result::Err(err),
        };

        let from_file = fs::File::open(&from);
        let mut from_file = match from_file {
            Ok(f) => f,
            Err(err) => return Result::Err(format!("cannot open file {}", err)),
        };

        match dump(&mut from_file, &mut to_file) {
            Ok(_) => println!("{} {}", from, "moved".green()),
            Err(e) => return Result::Err(e),
        };

        match fs::remove_file(&from) {
            Err(err) => println!("cannot remove file {}", err),
            Ok(_) => (),
        }
    }

    Result::Ok(1)
}

fn main() {
    let args = env::args();
    let num_args = args.len();
    if num_args < 3 {
        println!("{}", "min num args is < 2".magenta());
        std::process::exit(1);
    }

    let config = Config::from_args(args);
    let config = match config {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{} {}", "error:".red(), err.red());
            std::process::exit(1);
        }
    };

    match mv(config) {
        Ok(_) => println!("{}", "files moved!".green()),
        Err(err) => println!("{}", err.red()),
    };
}
