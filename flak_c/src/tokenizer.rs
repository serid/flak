#![feature(generators, generator_trait)]

use relinks::language::Token;
use std::ops::Generator;

pub fn tokenize() -> impl Generator<(), Yield = Token<FlakData>, Return = ()> {
    || {
        yield Token {
            id: todo!(),
            data: todo!(),
        };
    }
}

pub enum FlakData {
    Non,
    Int(i32),
    Char(String),
    String(String),
}
