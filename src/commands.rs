use std::{ collections::HashMap, ffi::OsStr, fs::exists, path::Path };
use crate::{ ExitReason, APP_VER };

pub fn run_command(
    args: &Vec<String>,
    opts: &HashMap<String, String>
) -> Result<(), (String, ExitReason)> {
    match args[1].as_str() {
        "version" => {
            println!("{}", APP_VER);
            return Ok(());
        }
        _ => {
            if exists(&args[1]).unwrap_or_else(|_| { false }) {
                let filename = Path::new(&args[0])
                    .file_name()
                    .unwrap_or(&OsStr::new("HOW-DID-YOU-EXECUTE-A-DIRECTORY"))
                    .to_str()
                    .unwrap_or("INVALID-FILE-NAME");
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
