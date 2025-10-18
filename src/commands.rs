use std::{ collections::HashMap, ffi::OsStr, fs::exists, path::Path };
use crate::*;

pub fn run_command(
    args: &Vec<String>,
    opts: &HashMap<String, String>
) -> Result<(), (String, ExitReason)> {
    macro_rules! opts {
        () => {
            &opts
        };
    }

    let filename = Path::new(&args[0])
        .file_name()
        .unwrap_or(&OsStr::new("HOW-DID-YOU-EXECUTE-A-DIRECTORY"))
        .to_str()
        .unwrap_or("INVALID-FILE-NAME");
    info!("Executing command [{}]", args[1].as_str());
    match args[1].as_str() {
        "version" => {
            println!("{}", APP_VER);
            Ok(())
        }
        "compile" => { compile::compile(args, opts) }
        _ => {
            if exists(&args[1]).unwrap_or(false) {
                return Err((
                    format!(
                        "Unknown command \"{}\". Did you mean \"{} compile {}\"?",
                        args[1],
                        filename,
                        args[1]
                    ),
                    ExitReason::UnknownCommand,
                ));
            }
            return Err((format!("Unknown command \"{}\".", args[1]), ExitReason::UnknownCommand));
        }
    }
}
