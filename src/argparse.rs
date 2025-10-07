use std::collections::HashMap;
use crate::ExitReason;

pub fn flag_set(opts: &HashMap<String, String>, id: &str) -> bool {
    opts.contains_key(id)
}

pub fn parse_args(
    args: Vec<String>,
    out_opts: &mut HashMap<String, String>,
    out_args: &mut Vec<String>
) -> Result<(), (String, ExitReason)> {
    let mut argsi = args.into_iter();
    while let Some(arg) = argsi.next() {
        if arg.starts_with("-") {
            let mut argnames: Vec<&str>;
            if arg.chars().nth(1) == Some('-') {
                let long_argname = arg.split_at(2).1;
                argnames = Vec::<&str>::new();
                argnames.push(long_argname);
            } else {
                argnames = arg
                    .split_at(1)
                    .1.split("")
                    .filter(|x| x != &"")
                    .collect();
            }
            for argname in argnames {
                let mut value: String = "".to_string();
                let argname_dsp = format!(
                    "-{}{}",
                    if arg.chars().nth(1) == Some('-') {
                        "-"
                    } else {
                        ""
                    },
                    argname
                );
                let expected_input_arg = Err((
                    format!("Option \"{}\" expected an input argument but got none.\nRun without arguments for the help page.", argname_dsp),
                    ExitReason::OptionExpectedInputArgument,
                ));

                let longhand;
                macro_rules! opt_with_arg {
                    ($longhand:expr) => {
                        {
                            match argsi.next() {
                                Some(val) => {
                                    value = val;
                                    longhand = $longhand;
                                }
                                None => {
                                    return expected_input_arg;
                                }
                            }
                        }
                    };
                }
                macro_rules! opt_no_arg {
                    ($longhand:expr) => {
                        {
                            longhand = $longhand;
                        }
                    };
                }
                match argname {
                    "out-file" | "O" => opt_with_arg!("out-file"),
                    "no-warn" | "W" => opt_no_arg!("no-warn"),
                    "version" | "V" => opt_no_arg!("version"),
                    "silent" | "s" => opt_no_arg!("silent"),
                    "soft-silent" | "S" => opt_no_arg!("soft-silent"),
                    "verbose" | "v" => opt_no_arg!("verbose"),
                    _ => {
                        return Err((
                            format!("Unknown option \"{}\".\nRun without arguments for the help page.", argname_dsp),
                            ExitReason::UnknownOption,
                        ));
                    }
                }
                out_opts.insert(longhand.to_string(), value.to_string());
            }
        } else if arg.starts_with("\\") {
            out_args.push(arg.split_at(1).1.to_string());
        } else {
            out_args.push(arg);
        }
    }
    return Ok(());
}
