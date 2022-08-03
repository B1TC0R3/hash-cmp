#![deny(clippy::pedantic)]

use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};
use std::error::Error;
use std::{env, fs, io, process};

enum ExitCode {
    HashEqual = 0,
    HashNotEqual = 255,
}

enum CmpResult {
    Equal(CmpData),
    NotEqual(CmpData),
}

struct CmpData {
    msg: String,
    file_hash: String,
    expected_hash: String,
}

fn print_help() {
    println!("Usage: hash-cmp [optional: -q] <input-file> <expected hash>");
    println!("Parameters:");
    println!("  -q: quiet mode");
}

fn print_verbose(cmp_data: &CmpData) {
    println!("{}", cmp_data.msg);
    println!("Found   :: {}", cmp_data.file_hash);
    println!("Expected:: {}", cmp_data.expected_hash);
}

fn print_quiet(cmp_data: &CmpData) {
    println!("{}", cmp_data.file_hash);
    println!("{}", cmp_data.expected_hash);
}

fn parse_args(mut args: Vec<String>) -> Result<(String, String, bool), Box<dyn Error>> {
    let mut quiet = false;

    if args.len() == 1 {
        print_help();
        process::exit(1);
    }

    if args[1] == "-h" {
        print_help();
        process::exit(0);
    }

    for arg in &args {
        if arg == "-q" {
            quiet = true;
        }
    }

    if args.len() >= 3 {
        return Ok((args.pop().unwrap(), args.pop().unwrap(), quiet));
    }
    Err("Invalid Arguments!".into())
}

fn get_file_hash256(path: String) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(path)?;
    io::copy(&mut file, &mut hasher)?;
    let hash256 = hasher.finalize();

    Ok(format!("{:x}", hash256))
}

fn hash_cmp(a: String, b: String) -> CmpResult {
    if a.len() != b.len() {
        return CmpResult::NotEqual(CmpData {
            msg: "Hash lengths do not match!".to_string(),
            file_hash: a,
            expected_hash: b,
        });
    }

    let mut is_equal: bool = true;
    let mut cmp_marker: String = String::new();

    for (a, b) in a.bytes().zip(b.bytes()) {
        if a == b {
            cmp_marker = format!("{}{}", cmp_marker, b as char);
        } else {
            cmp_marker = format!("{}\x1b[31m{}\x1b[0m", cmp_marker, b as char);
            is_equal = false;
        }
    }

    if is_equal {
        CmpResult::Equal(CmpData {
            msg: "Hashes are equal!".to_string(),
            file_hash: a,
            expected_hash: cmp_marker,
        })
    } else {
        CmpResult::NotEqual(CmpData {
            msg: "Hashes are not equal!".to_string(),
            file_hash: a,
            expected_hash: cmp_marker,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_hash: String;
    let expected_hash: String;
    let quiet: bool;

    match parse_args(args) {
        Err(e) => return Err(e),
        Ok((hash, file_path, mode)) => {
            file_hash = get_file_hash256(file_path)?;
            expected_hash = hash;
            quiet = mode;
        }
    }

    match hash_cmp(file_hash, expected_hash) {
        CmpResult::Equal(data) => {
            if quiet {
                print_quiet(&data);
            } else {
                print_verbose(&data);
            }
            process::exit(ExitCode::HashEqual as i32);
        }
        CmpResult::NotEqual(data) => {
            if quiet {
                print_quiet(&data);
            } else {
                print_verbose(&data);
            }
            process::exit(ExitCode::HashNotEqual as i32);
        }
    }
}
