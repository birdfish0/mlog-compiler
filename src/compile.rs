use std::io::ErrorKind;
use std::{ collections::HashMap };
use std::fs::read_to_string;
use crate::tokenize::{StringType, Token};
use crate::*;

#[derive(Debug)]
enum ValType {
    Nop,
    Ident,
    FuncCall,
    MacroCall,
    CodeBlock,
}

struct Val {
    pub t: ValType,
    pub ident: Option<String>,
    pub left: Option<Box<Val>>,
    pub right: Option<Box<Val>>,
    pub args: Option<Vec<Val>>,
}

impl Default for Val {
    fn default() -> Self {
        Val { t: ValType::Nop, ident: None, left: None, right: None, args: None, }
    }
}

enum State {
    None,
    PrevIsIdentifier,
    ParseArgs,
}

fn parse_tokens(tokens:&Vec<&Token>, opts:&HashMap<String, String>) -> Result<Val, (String, ExitReason)> {
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

    let nonestr = "None".to_string();

    let mut cblock = Val { t: ValType::CodeBlock, args: Some(Vec::<Val>::new()), ..Default::default() };
    let mut state = State::None;
    let mut val_wip = Val::default();
    let mut itokens = tokens.into_iter().peekable();
    let mut parenthesis_depth = 0;
    let mut buffer = Vec::<&Token>::new();
    while let Some(token) = itokens.next() {
        debug!("Processing {}, ", token);
        match state {
            State::None => {
                match token.content.as_str() {
                    _ => {
                        state = State::PrevIsIdentifier;
                        val_wip.left = Some(Box::new(Val { t: ValType::Ident, ident: Some(token.content.clone()), ..Default::default() }));
                    }
                }
            }
            State::PrevIsIdentifier => {
                match token.content.as_str() {
                    "(" => {
                        val_wip.t = ValType::FuncCall;
                        val_wip.args = Some(Vec::<Val>::new());
                        state = State::ParseArgs;
                        parenthesis_depth = 1;
                    }
                    _ => {
                        return Err((
                            format!(
                                "Unpexpected token {}'{}' after '{}', expected one of ['(', '!', '::', '=', '--', '++', '+=' , '-=', '*=', '/=', '//=', '^=', '=', '%=', '%%=', '<<=', '>>=', '>>>=', '&=', '|=', '&&=', '||='].{}",
                                match token.strtype {
                                    StringType::Not => "",
                                    _ => "string ",
                                },
                                token.content,
                                match val_wip.left { Some(x) => x.ident.unwrap_or(nonestr), None => nonestr},
                                pos!(token)
                            ),
                            ExitReason::CompileBadTokenAfterIdentifier,
                        ));
                    }
                }
            }
            State::ParseArgs => {
                match token.content.as_str() {
                    "(" => {
                        parenthesis_depth += 1;
                    }
                    ")" => {
                        parenthesis_depth -= 1;
                    }
                    "," => {
                        if parenthesis_depth == 1 {
                            match val_wip.args.as_mut(){
                                Some(v) => v,
                                None => {
                                    return Err(("Unwrapping val_wip.args.as_mut() failed.".to_string(), ExitReason::CompileWipArgsUnwrapFailed));
                                }
                            }.push(match parse_tokens(&buffer, opts){
                                Ok(val) => {
                                    match val.t {
                                        ValType::CodeBlock => {
                                            return Err((format!("Function argument should be a value, not executable code.{}", pos!(token)), ExitReason::CompileFuncArgNotValue));
                                        },
                                        _ => val
                                    }
                                },
                                e => {
                                    return e;
                                }
                            });
                        }
                        else {
                            buffer.push(token);
                        }
                    }
                    _ => {
                    }
                }
                if parenthesis_depth == 0 {
                    state = State::None;
                }
            }
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
    return Ok(cblock);
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
    if args.len() < 3 {
        return Err((
            "Command \"compile\" expected 1 argument. 0 were provided.".to_string(),
            ExitReason::CommandExpectedInputArgument,
        ));
    }
    let file = match read_to_string(&args[2]) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err((format!("File \"{}\" not found.", &args[2]), ExitReason::CompileFileNotFound));
            }
            else {
                return Err((format!("Reading file \"{}\" failed. Error: {}", &args[2], e.kind()), ExitReason::CompileFileNotFound));
            }
        }
    };
    let tokens = tokenize::tokenize(file);
    debug!("------ All tokens:");
    for token in &tokens {
        debug!("{}", token);
    }
    debug!("------");
    let tokens = match parse_tokens(&tokens.iter().collect(), opts) {
        Ok(t) => t,
        Err(e) => { return Err(e); }
    };
    return Ok(());
}
