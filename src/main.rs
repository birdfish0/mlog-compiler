use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::process::exit;
use std::path::Path;

mod argparse;
mod help;
mod commands;
mod tokenize;
mod compile;
mod logging;

use argparse::{ parse_args, flag_set };
use help::default_help_msg;
use commands::run_command;

enum ExitReason {
    #[allow(dead_code)]
    OK,
    UnknownOption,
    OptionExpectedInputArgument,
    UnknownCommand,
    CompileFileNotFound,
    CommandExpectedInputArgument,
}

#[macro_export]
macro_rules! unwrap {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(tuple) => {
                let er = tuple.0;
                let exit_code = tuple.1 as i32;
                err!("{}", er);
                err!("Exit code: {}", exit_code);
                exit(exit_code)
            },
        }
    };
}

const APP_VER: &str = "0.0.1";

fn main() -> Result<(), String> {
    let argv = env::args().collect::<Vec<String>>();
    let mut opts = HashMap::<String, String>::new();

    macro_rules! opts {
        () => {
            &opts
        };
    }

    let mut args = Vec::<String>::new();
    let filename = Path::new(&argv[0])
        .file_name()
        .unwrap_or(&OsStr::new("HOW-DID-YOU-EXECUTE-A-DIRECTORY"))
        .to_str()
        .unwrap_or("INVALID-FILE-NAME");
    unwrap!(parse_args(argv.clone(), &mut opts, &mut args));

    if flag_set(&opts, "version") {
        println!("{}", APP_VER);
        return Ok(());
    }

    if args.len() == 1 {
        default_help_msg(filename);
        return Ok(());
    }

    unwrap!(run_command(&args, &opts));

    return Ok(());
}
