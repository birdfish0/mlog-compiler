use std::{ collections::HashMap };
use std::fs::read_to_string;
use crate::tokenize::StringType;
use crate::*;

enum InstType {
    Stop,
    Nop,
    FuncCall,
    MacroCall,
}

struct Inst {
    pub t: InstType,
    pub ident: Option<String>,
}

impl Default for Inst {
    fn default() -> Self {
        Inst { t: InstType::Nop, ident: None }
    }
}

enum State {
    None,
    PrevIsIdentifier,
}

struct CodeBlock {
    c: Vec<CodeBlock>,
}

pub fn compile(
    args: &Vec<String>,
    opts: &HashMap<String, String>
) -> Result<(), (String, ExitReason)> {
    macro_rules! opts {
        () => {
            &opts
        };
    }

    macro_rules! pos {
        ($tok:expr) => {
            format!(" (line {}, col {})", $tok.line, $tok.col)
        };
    }

    if args.len() < 3 {
        return Err((
            "Command \"compile\" expected 1 argument. 0 were provided.".to_string(),
            ExitReason::CommandExpectedInputArgument,
        ));
    }
    if let Ok(file) = read_to_string(&args[2]) {
        let tokens = tokenize::tokenize(file);
        let mut insts = CodeBlock { c: Vec::<CodeBlock>::new() };
        let mut state = State::None;
        let mut instWIP = Inst::default();
        let nonestr = "None".to_string();
        let mut itokens = tokens.into_iter().peekable();
        while let Some(token) = itokens.next() {
            info!("{}, ", token);
            match state {
                State::None => {
                    match token.content.as_str() {
                        _ => {
                            state = State::PrevIsIdentifier;
                            instWIP.ident = Some(token.content.clone());
                        }
                    }
                }
                State::PrevIsIdentifier => {
                    match token.content.as_str() {
                        "(" => {
                            instWIP.t = InstType::FuncCall;
                        }
                        _ => {
                            return Err((
                                format!(
                                    "Unpexpected token after '{}', expected one of ['(', '!', '::', '=', '--', '++', '+=' , '-=', '*=', '/=', '//=', '^=', '=', '%=', '%%=', '<<=', '>>=', '>>>=', '&=', '|=', '&&=', '||='], but got {}'{}'.{}",
                                    instWIP.ident.unwrap_or(nonestr),
                                    match token.strtype {
                                        StringType::Not => "",
                                        _ => "string ",
                                    },
                                    token.content,
                                    pos!(token)
                                ),
                                ExitReason::CompileBadTokenAfterIdentifier,
                            ));
                        }
                    }
                }
                _ => {}
            }
            if
                token.strtype == StringType::Char &&
                !(
                    token.content.clone().char_indices().count() == 1 ||
                    token.content.starts_with("\\u")
                )
            {
                return Err((
                    format!(
                        "Char '{}' should be 1 character long, but is {}.{}",
                        token.content,
                        token.content.len(),
                        pos!(token)
                    ),
                    ExitReason::CompileCharTooLong,
                ));
            }
        }
        return Ok(());
    }
    return Err((format!("File \"{}\" not found.", &args[2]), ExitReason::CompileFileNotFound));
}
