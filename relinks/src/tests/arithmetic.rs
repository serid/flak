use crate::language::{TerminalId, Token};
use cranelift_entity::EntityRef;
use flak_util::{kvec::KArray, util::slice_plus};
use lazy_static::lazy_static;
use std::ops::Generator;

/*
///! This tokenizer is suitable for recognizing most languages. It does not support
///! significant whitespace, significant indentation or custom string and int literals.
*/

fn tokenize(s: &str) -> impl Generator<(), Yield = Token<ArithData>, Return = ()> + '_ {
    let plus_index = TerminalId::new(TERMINALS.0.iter().position(|r| r == "+").unwrap());
    let mul_index = TerminalId::new(TERMINALS.0.iter().position(|r| r == "*").unwrap());
    let int_index = TerminalId::new(TERMINALS.0.iter().position(|r| r == "<int>").unwrap());

    let mut iterator = s.chars();

    move || {
        loop {
            // More iterator.next() calls will follow
            let c = if let Some(c) = iterator.next() {
                c
            } else {
                break;
            };

            match c {
                ' ' => {}
                '+' => {
                    yield Token {
                        id: plus_index,
                        data: ArithData::Non,
                    }
                }
                '*' => {
                    yield Token {
                        id: mul_index,
                        data: ArithData::Non,
                    }
                }
                mut c if c.is_digit(10) => {
                    let mut n = 0;

                    loop {
                        n *= 10;
                        n += c.to_digit(10).unwrap();
                        c = if let Some(c) = iterator.next() {
                            c
                        } else {
                            break;
                        };
                        if !c.is_digit(10) {
                            break;
                        }
                    }

                    yield Token {
                        id: int_index,
                        data: ArithData::Int(n),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

lazy_static! {
    static ref KEYWORDS: KArray<TerminalId, String> =
        KArray::new(Box::new(["+".into(), "*".into()]));
    static ref TERMINALS: KArray<TerminalId, String> =
        KArray::new(slice_plus(&KEYWORDS.0, &["<int>".into()]));
}

#[derive(Clone, Copy, Debug)]
enum ArithData {
    Non,
    Int(u32),
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn t1() {
        std::dbg!(iter::from_generator(tokenize("1 + 2")).collect::<Vec<_>>());
    }
}
