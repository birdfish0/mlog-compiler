use std::fmt::Display;

#[derive(PartialEq)]
pub enum StringType {
    Not,
    String,
    Char,
    Backtick,
}

pub struct Token {
    pub content: String,
    pub line: u64,
    pub col: u64,
    pub strtype: StringType,
}

impl Default for Token {
    fn default() -> Self {
        Token { content: "".to_string(), line: 0, col: 0, strtype: StringType::Not }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Token {}\"{}\" {}:{}>",
            match self.strtype {
                StringType::Not => { "" }
                StringType::Backtick => { "b" }
                StringType::String => { "s" }
                StringType::Char => { "c" }
            },
            self.content,
            self.line,
            self.col
        )
    }
}

pub fn tokenize(file: String) -> Vec<Token> {
    let punctuation = [
        ';',
        ':',
        '.',
        ',',
        '{',
        '}',
        '[',
        ']',
        '(',
        ')',
        '!',
        '@',
        '#',
        '$',
        '%',
        '/',
        '?',
        '^',
        '&',
        '*',
        '-',
        '+',
        '=',
        '|',
        '<',
        '>',
        '~',
    ];
    let multi_puncs = [
        "==",
        "--",
        "++",
        "===",
        "||",
        "&&",
        "//",
        ">>",
        ">>>",
        "<<",
        ">=",
        "<=",
        "!=",
        "!==",
        "%%",
        "*=",
        "/=",
        "//=",
        "+=",
        "-=",
        "^=",
        "~=",
        "<<=",
        ">>=",
        ">>>=",
        "|=",
        "&=",
        "||=",
        "&&=",
        "%%=",
        "..",
    ];
    let string_specifiers = ['\"', '\'', '`'];
    let whitespace = [
        ' ',
        '\t',
        '\n',
        '\r',
        '\u{000c}',
        '\u{000b}',
        '\u{0085}',
        '\u{200e}',
        '\u{200f}',
        '\u{2028}',
        '\u{2029}',
    ];
    let mut tokens = Vec::<Token>::new();
    let mut left = 0;
    let mut right = 0;
    let file = file + " ";
    let mut chars = file.chars().peekable();
    let charidx = file.char_indices().collect::<Vec<_>>();
    let mut instr = StringType::Not;
    let mut strescape = false;
    let mut intok = false;
    let mut line = 1;
    let mut col:i128 = -1;
    macro_rules! flush_token {
        () => {
            if left != right - 1 {
                let macro_str = file
                                    .split_at(i!(left))
                                    .1.split_at(i!(right - 1) - i!(left))
                                    .0.to_string();
                tokens.push(Token {
                    content: macro_str,
                        line,
                        col: match (col < 0, col > u64::MAX as i128) {
                                (true, _) => 0,
                                (_, true) => u64::MAX,
                                _ => col as u64
                            },
                        ..Default::default()
                    });
            }
            left = right;
        };
    }
    let nopunc_tok = Token {
        content: "ERR".to_string(),
        ..Default::default()
    };
    macro_rules! i {
        ($idx:expr) => {
            charidx[$idx].0
        };
    }
    let ch: char = '\0';
    macro_rules! cont {
        () => {
            if ch == '\n' {
                col = -1;
                line += 1;
            }
            continue;
        };
    }
    while let Some(ch) = chars.next() {
        right += 1;
        col += 1;

        if instr != StringType::Not && strescape {
            strescape = false;
            cont!();
        }
        if string_specifiers.contains(&ch) {
            instr = match (&instr, ch) {
                (StringType::Not, '\'') => StringType::Char,
                (StringType::Not, '\"') => StringType::String,
                (StringType::Not, '`') => StringType::Backtick,
                (StringType::Char, '\'') => StringType::Not,
                (StringType::String, '\"') => StringType::Not,
                (StringType::Backtick, '`') => StringType::Not,
                _ => instr,
            };
            if instr == StringType::Not {
                if left != right - 1 {
                    let macro_str = file
                        .split_at(i!(left))
                        .1.split_at(i!(right - 1) - i!(left))
                        .0.to_string();
                    tokens.push(Token {
                        content: macro_str,
                        line,
                        col: match (col < 0, col > u64::MAX as i128) {
                                (true, _) => 0,
                                (_, true) => u64::MAX,
                                _ => col as u64
                            },
                        strtype: match ch {
                            '\'' => StringType::Char,
                            '\"' => StringType::String,
                            '`' => StringType::Backtick,
                            _ => StringType::Not,
                        },
                    });
                }
                left = right;
            } else {
                flush_token!();
            }
        }
        if instr != StringType::Not {
            if ch == '\\' {
                strescape = true;
            }
            cont!();
        }
        if ch == '\\' {
            intok = !intok;
            flush_token!();
            continue;
        }
        if intok {
            cont!();
        }

        if whitespace.contains(&ch) {
            flush_token!();
        } else if punctuation.contains(&ch) {
            let lastpunc = tokens.last().unwrap_or(&nopunc_tok);
            let filtered = multi_puncs
                .iter()
                .filter(
                    |x|
                        x.starts_with(&lastpunc.content) &&
                        x.chars().nth(lastpunc.content.len()) == Some(ch)
                )
                .collect::<Vec<_>>();
            if filtered.len() > 0 {
                let new_token = Token {
                    line,
                    col: match (col < 0, col > u64::MAX as i128) {
                            (true, _) => 0,
                            (_, true) => u64::MAX,
                            _ => col as u64
                        },
                    content: lastpunc.content.clone() + ch.to_string().as_str(),
                    ..Default::default()
                };
                _ = tokens.pop();
                tokens.push(new_token);
                left = right;
            } else {
                flush_token!();
                tokens.push(Token {
                    content: ch.to_string(),
                    line,
                    col: match (col+1 < 0, col+1 > u64::MAX as i128) {
                                (true, _) => 0,
                                (_, true) => u64::MAX,
                                _ => (col+1) as u64
                            },
                    ..Default::default()
                });
            }
        }
        cont!();
    }
    return tokens;
}
