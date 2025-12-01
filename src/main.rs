use std::io::{self, Write};
use std::process;

mod builtins;
mod externals;
mod string_utils;

use string_utils as su;

fn take_input() -> String {
    let mut command = String::new();

    io::stdin().read_line(&mut command).unwrap();

    return command;
}

fn eval(command: & String) -> i32 {
    let args = su::split_args(&command);

    let mut _result = 1;
    match builtins::lookup(& args[0]){
        Some(cmd) => return cmd(& args),
        None => _result = -1
    }

    match externals::lookup(& args[0]){
        Some(_path) => {
            let output = process::Command::new(&args[0])
                    .args(args[1..].iter())
                    .output()
                    .expect("Failed to execute command");
            print!("{}", String::from_utf8_lossy(&output.stdout));
            _result = output.status.code().unwrap_or(1);
        },
        None => {
            _result = -1;
        }
    }
    _result
}

// Some return codes: 
// -1 -> Command not found
// 0 -> OK
// 1 -> Generic
// 64 -> Command usage error

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = take_input();

        let result = eval(&command);

        // su::split_args(&command);

        if result == -1 {
            println!("{}: command not found", command.trim());
        }
    }
}
