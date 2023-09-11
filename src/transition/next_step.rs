use crate::transition::prelude::*;
use std::sync::{Arc, Weak};

#[derive(Clone)]
pub enum NextStep<L: Lens>
where
    L::Value: Tweenable,
{
    None,
    Step(Arc<TransitionStep<L>>),
    Cycle(Weak<TransitionStep<L>>),
}

impl<L: Lens> PartialEq for NextStep<L>
where
    L::Value: Tweenable,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NextStep::None, NextStep::None) => true,
            (NextStep::None, _) => false,
            (_, NextStep::None) => false,

            (NextStep::Step(a), NextStep::Step(b)) => a.eq(b),
            (NextStep::Step(a), NextStep::Cycle(b)) => b.upgrade().is_some_and(|b| b.eq(a)),
            (NextStep::Cycle(a), NextStep::Step(b)) => a.upgrade().is_some_and(|a| a.eq(b)),
            (NextStep::Cycle(a), NextStep::Cycle(b)) => a.ptr_eq(b) || a.upgrade().eq(&b.upgrade()),
        }
    }
}

impl<L: Lens> From<Arc<TransitionStep<L>>> for NextStep<L>
where
    L::Value: Tweenable,
{
    fn from(value: Arc<TransitionStep<L>>) -> Self {
        Self::Step(value)
    }
}

impl<L: Lens> From<Weak<TransitionStep<L>>> for NextStep<L>
where
    L::Value: Tweenable,
{
    fn from(value: Weak<TransitionStep<L>>) -> Self {
        Self::Cycle(value)
    }
}

impl<L: Lens> NextStep<L>
where
    L::Value: Tweenable,
{
    pub fn is_none(&self) -> bool {
        matches!(self, NextStep::None)
    }
}
