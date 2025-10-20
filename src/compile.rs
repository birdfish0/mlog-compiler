use std::io::ErrorKind;
use std::{ collections::HashMap };
use std::fs::read_to_string;
use crate::tokenize::{StringType, Token};
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
    isi: bool, // Is Instruction
    c: Option<Vec<CodeBlock>>, // Code collection or smth idk
    i: Option<Inst>, // Instruction
}

impl CodeBlock {
    pub fn new_block() -> Self {
        Self { isi: false, c: Some(Vec::<CodeBlock>::new()), i: None }
    }
    pub fn new_inst() -> Self {
        Self {isi:true, c:None, i: Some(Inst::default())}
    }
    pub fn new_inst_from(inst:Inst) -> Self {
        Self {isi:true, c:None, i: Some(inst)}
    }
}

fn parse_tokens(tokens:Vec<Token>, opts:&HashMap<String, String>) -> Result<CodeBlock, (String, ExitReason)> {
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

    let mut insts = CodeBlock::new_block();
    let mut state = State::None;
    let mut inst_wip = Inst::default();
    let nonestr = "None".to_string();
    // let mut itokens = tokens.into_iter().peekable();
    for token in &tokens {
        info!("{}, ", token);
        match state {
            State::None => {
                match token.content.as_str() {
                    _ => {
                        state = State::PrevIsIdentifier;
                        inst_wip.ident = Some(token.content.clone());
                    }
                }
            }
            State::PrevIsIdentifier => {
                match token.content.as_str() {
                    "(" => {
                        inst_wip.t = InstType::FuncCall;
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
                                inst_wip.ident.unwrap_or(nonestr),
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
    return Ok(insts);
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
    for token in &tokens {
        debug!("{}", token);
    }
    info!("------");
    let tokens = match parse_tokens(tokens, opts) {
        Ok(t) => t,
        Err(e) => { return Err(e); }
    };
    return Ok(());
}
