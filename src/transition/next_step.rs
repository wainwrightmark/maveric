// use crate::transition::prelude::*;
// use std::sync::{Arc, Weak};

// #[derive(Clone)]
// pub enum NextStep<L: Lens>
// where
//     L::Value: Tweenable,
// {
//     None,
//     Step(Arc<TransitionStep<L>>),
//     Cycle(Weak<TransitionStep<L>>),
// }

// impl<L: Lens> PartialEq for NextStep<L>
// where
//     L::Value: Tweenable,
// {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::None, Self::None) => true,
//             (Self::None, _) | (_, Self::None) => false,

//             (Self::Step(a), Self::Step(b)) => a.eq(b),
//             (Self::Step(a), Self::Cycle(b)) => b.upgrade().is_some_and(|b| b.eq(a)),
//             (Self::Cycle(a), Self::Step(b)) => a.upgrade().is_some_and(|a| a.eq(b)),
//             (Self::Cycle(a), Self::Cycle(b)) => a.ptr_eq(b) || a.upgrade().eq(&b.upgrade()),
//         }
//     }
// }

// impl<L: Lens> From<Arc<TransitionStep<L>>> for NextStep<L>
// where
//     L::Value: Tweenable,
// {
//     fn from(value: Arc<TransitionStep<L>>) -> Self {
//         Self::Step(value)
//     }
// }

// impl<L: Lens> From<Weak<TransitionStep<L>>> for NextStep<L>
// where
//     L::Value: Tweenable,
// {
//     fn from(value: Weak<TransitionStep<L>>) -> Self {
//         Self::Cycle(value)
//     }
// }

// impl<L: Lens> NextStep<L>
// where
//     L::Value: Tweenable,
// {
//     #[must_use] pub const fn is_none(&self) -> bool {
//         matches!(self, Self::None)
//     }
// }
