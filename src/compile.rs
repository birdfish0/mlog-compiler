use std::{ collections::HashMap };
use std::fs::read_to_string;
use crate::tokenize::tokenize;
use crate::{ tokenize, ExitReason };

pub fn compile(
    args: &Vec<String>,
    opts: &HashMap<String, String>
) -> Result<(), (String, ExitReason)> {
    if args.len() < 3 {
        return Err((
            "Command \"compile\" expected 1 argument. 0 were provided.".to_string(),
            ExitReason::CommandExpectedInputArgument,
        ));
    }
    if let Ok(file) = read_to_string(&args[2]) {
        let tokens = tokenize(file);
        print!("[");
        for token in tokens {
            print!("{}, ", token);
        }
        print!("]");
        println!("");
        return Ok(());
    }
    return Err((format!("File \"{}\" not found.", &args[2]), ExitReason::CompileFileNotFound));
}
