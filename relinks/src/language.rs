use cranelift_entity::entity_impl;

#[derive(Clone, Debug)]
pub struct Token<D> {
    pub id: TerminalId,

    // At parser runtime, if token has no data, then it is not included in CST
    pub data: D,
}

pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TerminalId(u32);
entity_impl!(TerminalId);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct NonTerminalId(u32);
entity_impl!(NonTerminalId);
