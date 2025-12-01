use std::path::PathBuf;
use std::{env, path::Path};
use std::fs;

use crate::externals;

type Builtin = fn(&Vec<String>) -> i32 ;

pub fn lookup(command: & String) -> Option<Builtin> {
    match command.as_str() {
        "echo" => Some(echo),
        "type" => Some(type_f),
        "pwd" => Some(pwd),
        "cd" => Some(cd),
        "exit" => Some(exit),
        _ => None
    }
}

pub fn exit(_args: &Vec<String>) -> i32 {
    std::process::exit(0)
}

pub fn echo(args: &Vec<String>) -> i32 {
    for arg in &args[1..]{
        print!("{} ", arg);
    }
    print!("\n");
    0
}

pub fn type_f(args: &Vec<String>) -> i32 {
    if !check_argument_len(2, args.len()){
        return 64; 
    };

    match lookup(&args[1]){
        // Internal lookup
        Some(_t) => {
            println!("{} is a shell builtin", args[1]);
            return 0
        },
        None => {
            // External lookup
                match externals::lookup(& args[1]){
                    Some(path) => {
                        println!("{} is {}", args[1], path);
                        return 0
                    },
                    None => {}
                }
            println!("{}: not found", args[1]);
            0
        }
    }
}

pub fn pwd(_args: &Vec<String>) -> i32 {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            return 0
        }
        Err(_) => {
            println!("Unkown path");
            return 1;
        }
    };
}

pub fn cd(args: &Vec<String>) -> i32 {
    if !check_argument_len(2, args.len()){
        return 64; 
    };

    let path: PathBuf = 
        if args[1] == "~"{
            match env::home_dir() {
                Some(dir) => dir,
                None => {
                    println!("cd: Error getting home directory");
                    return 1
                }
            }
        } else {
            PathBuf::from(&args[1])
        };

    if !fs::exists(&path).unwrap_or(false) {
        println!("cd: {}: No such file or directory", path.display());
        return 1;
    };

    if !is_dir(&path) {
        println!("{} is not a directory", path.to_str().unwrap_or(""));
        return 1;
    };

    match env::set_current_dir(path){
        Ok(_) => 0,
        Err(e) => {
            println!("cd: failed to change working directory: {}", e.to_string()); 
            1
        }
    }
}

fn is_dir(path: & Path) -> bool {
    match fs::read_dir(path){
        Ok(_) => true,
        Err(_) => false
    }
}

// fn has_multiple_args(args: & [&str]) -> bool {
//     return args.len() > 1
// }

fn check_argument_len(desired_len: usize, actaul_len: usize) -> bool {
    if desired_len != actaul_len {
        println!("Expected {} argument, but got {}", desired_len - 1, actaul_len - 1);
        return false;
    }
    true
}