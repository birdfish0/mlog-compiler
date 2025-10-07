use std::fmt::Display;

pub struct Token {
    pub content: String,
    pub line: i32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Token \"{}\" l{}>", self.content, self.line)
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
    macro_rules! flush_token {
        () => {
            if left != right - 1 {
                tokens.push(Token {
                    content: file
                        .split_at(left)
                        .1.split_at(right - left - 1)
                        .0.to_string(),
                    line
                });
            }
        };
    }
    while let Some(ch) = chars.next() {
        right += 1;

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
            });
            left = right;
        }
        if ch == '\n' {
            line += 1;
        }
    }
    println!("");
    return tokens;
}
