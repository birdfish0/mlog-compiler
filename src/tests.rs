#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{ compile::{ parse_tokens, Val, ValType, VarType }, tokenize::{ tokenize }, unwrap };
    use crate::*;
    use std::process::exit;

    macro_rules! opts {
        () => {
            &HashMap::new()
        };
    }
    macro_rules! id {
        ($s:expr) => {
            Some($s.to_string())
        };
    }
    macro_rules! svec {
        ($($arg:expr),*) => {
            Some(vec!($($arg),*))
        };
    }
    macro_rules! num {
        ($n:expr) => {
            Val {
                t: ValType::Const,
                ident: id!($n),
                vt: VarType::Num,
                ..Default::default()
            }
        };
    }

    #[test]
    fn numbers() {
        let res = parse_tokens(
            &tokenize(include_str!("tests/test-numbers.txt").to_string()).iter().collect(),
            opts!(),
            0
        );
        let res = unwrap!(res);
        assert_eq!(res, Val {
            t: ValType::FuncCall,
            ident: id!("func"),
            args: svec!(
                num!("1"),
                num!("1.1"),
                num!("-1"),
                num!("-1.1"),
                num!("1e1"),
                num!("-1e1"),
                num!("1.1e1"),
                num!("-1.1e1"),
                num!("1e-1"),
                num!("1.1e-1"),
                num!("-1e-1"),
                num!("-1.1e-1")
            ),
            ..Default::default()
        });
    }
}
