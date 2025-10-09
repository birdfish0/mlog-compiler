use std::fmt::Display;

pub struct Token {
    pub content: String,
    pub line: i64,
    pub col: i64,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Token \"{}\" {}:{}>", self.content, self.line, self.col)
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
    let string_specifiers = ['\"', '`'];
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
    let mut instr = false;
    let mut intok = false;
    let mut line = 1;
    let mut col = -1;
    macro_rules! flush_token {
        () => {
            if left != right - 1 {
                tokens.push(Token {
                    content: file
                        .split_at(left)
                        .1.split_at(right - left - 1)
                        .0.to_string(),
                    line,
                    col
                });
            }
        };
    }
    while let Some(ch) = chars.next() {
        right += 1;
        col += 1;

        if ch == '\\' {
            intok = !intok;
            flush_token!();
            left = right;
            continue;
        }
        if intok {
            continue;
        }

        if whitespace.contains(&ch) {
            flush_token!();
            left = right;
        } else if punctuation.contains(&ch) {
            flush_token!();
            tokens.push(Token {
                content: ch.to_string(),
                line,
                col: col + 1,
            });
            left = right;
        }
        if ch == '\n' {
            col = -1;
            line += 1;
        }
    }
    println!("");
    return tokens;
}
