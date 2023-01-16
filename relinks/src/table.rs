use cranelift_entity::entity_impl;
use flak_util::kvec::KArray;

use crate::language::{NonTerminalId, TerminalId};

pub(crate) struct Table(pub(crate) KArray<StateId, Row>);

pub(crate) struct Row {
    pub(crate) item_sets: KArray<RuleId, String>,
    pub(crate) action: KArray<TerminalId, Action>,
    pub(crate) goto: KArray<NonTerminalId, StateId>,
}

pub(crate) enum Action {
    Just(StackAction),
    Fork(Box<[StackAction]>),
    Error,
    Done,
}

pub(crate) enum StackAction {
    Shift(StateId),
    Reduce(RuleId),
}

// NOTE: the parsing loop can work without specifying precise rules, only LHS and number of symbols are needed.
// the symbols are still encoded to enable runtime correctness assertions
// pub(crate) Rule {
//     pub(crate) lhs: NonTerminalId,
//     pub(crate) rhs: Box<[Symbol]>,
// }
pub(crate) struct Rule {
    pub(crate) lhs: NonTerminalId,
    pub(crate) rhs_len: usize,
}
pub(crate) type Rules = KArray<RuleId, Rule>;

pub(crate) struct Symbols {
    pub(crate) terminals: KArray<TerminalId, NonTerminalId>,
    pub(crate) non_terminals: KArray<NonTerminalId, NonTerminalId>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct RuleId(u32);
entity_impl!(RuleId);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct StateId(u32);
entity_impl!(StateId);
