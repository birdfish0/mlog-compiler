use std::fmt::Display;
use std::io::ErrorKind;
use std::{ collections::HashMap };
use std::fs::read_to_string;
use crate::tokenize::{StringType, Token};
use crate::*;

#[derive(Debug)]
enum ValType {
    Nop,
    Ident,
    Const,
    FuncCall,
    MacroCall,
    CodeBlock,
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum VarType {
    Nop,
    Str,
    Char,
    Num,
}

#[derive(Debug)]
struct Val {
    pub t: ValType,
    pub vt: VarType,
    pub ident: Option<String>,
    pub left: Option<Box<Val>>,
    pub right: Option<Box<Val>>,
    pub args: Option<Vec<Val>>,
}

impl Default for Val {
    fn default() -> Self {
        Val { t: ValType::Nop, vt: VarType::Nop, ident: None, left: None, right: None, args: None, }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            ValType::Nop => write!(f, "<Nop>"),
            ValType::CodeBlock => {
                if let Some(args) = &self.args {
                    let mut outstr = "".to_string();
                    for arg in args {
                        outstr += format!("{}", arg).as_str();
                        outstr += ", ";
                    }
                    outstr = outstr.strip_suffix(", ").unwrap_or_default().to_string();
                    write!(f, "<CodeBlock [{}]>", outstr)
                }
                else {
                    write!(f, "<CodeBlock [INVALID]>")
                }
            }
            ValType::Ident => write!(f, "<Ident \"{}\">", self.ident.as_ref().unwrap_or(&"<UNKNOWN>".to_string())),
            ValType::Const => write!(f, "<Const \"{}\" ({:?})>", self.ident.as_ref().unwrap_or(&"<UNKNOWN>".to_string()), self.vt),
            ValType::MacroCall | ValType::FuncCall => {
                if let Some(args) = &self.args {
                    let mut outstr = "".to_string();
                    for arg in args {
                        outstr += format!("{}", arg).as_str();
                        outstr += ", ";
                    }
                    outstr = outstr.strip_suffix(", ").unwrap_or_default().to_string();
                    write!(f, "<{:?} {}([{}])>",
                            self.t,
                            self.ident.as_ref().unwrap_or(&"<UNKNOWN>".to_string()),
                            outstr)
                }
                else {
                    write!(f, "<{:?} {}([INVALID])>",
                            self.t,
                            self.ident.as_ref().unwrap_or(&"<UNKNOWN>".to_string()))
                }
            }
        }
    }
}

enum State {
    None,
    PrevIsIdentifier,
    PrevIsConst,
    ParseArgs,
}

fn is_num(s: &String) -> bool {
    let numerics = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'e'];
    if s.len() == 0 {
        return false;
    }
    if !s.is_ascii() {
        return false;
    }
    let mut ecount = 0;
    return s.chars().all(|x| {
        if x == 'e' {
            ecount += 1;
        }
        return ecount <= 1 && numerics.contains(&x);
    })
}

fn parse_tokens(tokens:&Vec<&Token>, opts:&HashMap<String, String>, depth: u64) -> Result<Val, (String, ExitReason)> {
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

    debug!("Begin depth {}", depth);

    let nonestr = "None".to_string();

    let mut cblock = Val { t: ValType::CodeBlock, args: Some(Vec::<Val>::new()), ..Default::default() };
    let mut state = State::None;
    let mut val_wip = Val::default();
    let mut val_isnew = true;
    let mut parenthesis_depth = 0;
    let mut buffer = Vec::<&Token>::new();
    let mut should_return_codeblock = false;

    macro_rules! flush {
        () => {
            cblock.args.as_mut().unwrap(/* safe unwrap */).push(val_wip);
            val_wip = Val::default();
            val_isnew = true;
        };
    }

    let mut first = true;
    let mut i:usize = 0;
    loop {
        if !first {
            i += 1;
        }
        else {
            first = false;
        }
        if i >= tokens.len() {
            break;
        }
        let token = tokens[i];
        debug!("[Depth {}] Processing {}.", depth, token);
        match state {
            State::None => {
                match token.content.as_str() {
                    ";" => {
                        flush!();
                        should_return_codeblock = true;
                    }
                    _ => {
                        val_isnew = false;
                        val_wip = Val { ident: Some(token.content.clone()), ..Default::default() };
                        if token.strtype != StringType::Not {
                            val_wip.t = ValType::Const;
                            val_wip.vt = match token.strtype {
                                StringType::Char => VarType::Char,
                                _ => VarType::Str
                            };
                            state = State::PrevIsConst;
                        }
                        else if is_num(&token.content) {
                            val_wip.t = ValType::Const;
                            val_wip.vt = VarType::Num;
                            let def = &&Default::default();
                            let maybe_dot = tokens.get(i+1).unwrap_or(def);
                            if maybe_dot.content == "." {
                                if !token.content.contains("e") {
                                    debug!("[Depth {}] Processing {} as decimal point.", depth, maybe_dot);
                                    let def = &&Default::default();
                                    let remainder = tokens.get(i+2).unwrap_or(def);
                                    let nextcont = &remainder.content;
                                    if is_num(&nextcont) {
                                        debug!("[Depth {}] Processing {} as number remainder.", depth, remainder);
                                        let mut _discard = "".to_string();
                                        *val_wip.ident.as_mut().unwrap_or(&mut _discard) += ".";
                                        *val_wip.ident.as_mut().unwrap_or(&mut _discard) += nextcont.as_str();
                                        i += 2;
                                    }
                                    else {
                                        debug!("[Depth {}] Skipped {} as number remainder.", depth, remainder);
                                    }
                                }
                                else {
                                    debug!("[Depth {}] Skipped {} as decimal point.", depth, maybe_dot);
                                }
                            }
                            state = State::PrevIsConst;
                        }
                        else {
                            val_wip.t = ValType::Ident;
                            state = State::PrevIsIdentifier;
                        }
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
                macro_rules! flush_funcarg {
                    () => {
                        match val_wip.args.as_mut(){
                            Some(v) => v,
                            None => {
                                return Err(("Unwrapping val_wip.args.as_mut() failed.".to_string(), ExitReason::CompileWipArgsUnwrapFailed));
                            }
                        }.push(match parse_tokens(&buffer, opts, depth+1){
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
                    };
                }
                match token.content.as_str() {
                    "(" => {
                        parenthesis_depth += 1;
                    }
                    ")" => {
                        parenthesis_depth -= 1;
                    }
                    "," => {
                        if parenthesis_depth == 1 {
                            flush_funcarg!();
                            buffer.clear();
                        }
                        else {
                            buffer.push(token);
                        }
                    }
                    _ => {
                        buffer.push(token);
                    }
                }
                if parenthesis_depth == 0 {
                    if buffer.len() > 0 {
                        flush_funcarg!();
                    }
                    state = State::None;
                    buffer.clear();
                }
            }
            State::PrevIsConst => {
                err!("not implemented");
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
    if !val_isnew {
        cblock.args.as_mut().unwrap().push(val_wip);
    }
    if (!should_return_codeblock) && cblock.args.as_ref().unwrap().len() == 1 {
        cblock = cblock.args.as_mut().unwrap().remove(0);
        debug!("[Depth {}] Returning Val of type {:?} instead of CodeBlock.", depth, cblock.t);
    }

    debug!("End depth {}", depth);
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
    info!("Reading file");
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
    info!("File read success");
    info!("Begin tokenize");
    let tokens = tokenize::tokenize(file);
    info!("Tokenize success");
    debug!("------ All tokens:");
    for token in &tokens {
        debug!("{}", token);
    }
    debug!("------");
    info!("Parse tokens (first pass)");
    let root = match parse_tokens(&tokens.iter().collect(), opts, 0) {
        Ok(t) => t,
        Err(e) => { return Err(e); }
    };
    debug!("{}", root);
    return Ok(());
}
