use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub struct SetComponentCommands<
    'n,
    'p,
    'c1,
    'c2,
    'world,
    'ec,
    'w,
    's,
    'a,
    N: PartialEq,
    C: NodeContext,
> {
    args: NodeArgs<'n, 'p, 'c1, 'c2, N, C>,
    world: &'world World,
    ec: &'ec mut EntityCommands<'w, 's, 'a>,
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C>
{
    pub(crate) fn new(
        args: NodeArgs<'n, 'p, 'c1, 'c2, N, C>,
        world: &'world World,
        ec: &'ec mut EntityCommands<'w, 's, 'a>,
    ) -> Self {
        Self { args, world, ec }
    }

    pub fn scope<'ec2, 'selfie>(
        &'selfie mut self,
        f: impl FnOnce(SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec2, 'w, 's, 'a, N, C>),
    ) where
        'n: 'ec2,
        'p: 'ec2,
        'c1: 'ec2,
        'c2: 'ec2,
        'world: 'ec2,
        'ec: 'ec2,
        'w: 'ec2,
        's: 'ec2,
        'a: 'ec2,
        'selfie: 'ec2,
    {
        let clone = SetComponentCommands {
            args: self.args.clone(),
            world: self.world,
            ec: self.ec,
        };
        f(clone);
    }

    #[must_use]
    pub fn ignore_node(
        self,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C> {
        self.map_args(|_| &())
    }

    #[must_use]
    pub fn ignore_context(
        self,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N2, C> {
        SetComponentCommands {
            args: self.args.map_node(map),

            world: self.world,
            ec: self.ec,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C2> {
        SetComponentCommands {
            args: self.args.map_context(map),

            world: self.world,
            ec: self.ec,
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C>
{
    pub const fn finish(self) {}

    #[allow(clippy::return_self_not_must_use)]
    pub fn insert_with_node_and_context<B: Bundle>(
        self,
        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>) -> B,
    ) -> Self {
        if self.args.is_hot() {
            self.advanced(|a, c| c.insert(f(a.node, a.context)))
        } else {
            self
        }
    }

    /// Gives you advanced access to the commands.
    /// You are responsible for checking if anything has changed.
    #[allow(clippy::return_self_not_must_use)]
    pub fn advanced(
        self,
        f: impl FnOnce(&NodeArgs<'n, 'p, 'c1, 'c2, N, C>, &mut ComponentCommands),
    ) -> Self {
        let mut occ = ComponentCommands::new(self.ec, self.world, self.args.event);
        f(&self.args, &mut occ);
        self
    }

    /// Animate a property based on the node value
    /// You may have call `ignore_context` before calling this
    #[allow(clippy::return_self_not_must_use)]
    pub fn animate<L: Lens + GetValueLens>(
        self,
        get_value: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>) -> L::Value,
        speed: Option<<L::Value as Tweenable>::Speed>,
    ) -> Self
    where
        L::Value: Tweenable + Clone,
        L::Object: Clone + Component,
    {
        self.advanced(|args, commands| {
            if !args.is_hot() {
                return;
            }

            let value = get_value(args.node, args.context);

            commands.transition_value::<L>(value.clone(), value, speed);
        })
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq + IntoBundle>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext>
{
    #[allow(clippy::return_self_not_must_use, clippy::must_use_candidate)]
    pub fn insert_bundle(self) -> Self {
        if self.args.is_hot() {
            self.ec.insert(self.args.node.clone().into_bundle());
        }

        self
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: Clone + PartialEq>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext>
{
    /// Animate a property based on the node value
    /// You may have call `ignore_context` before calling this
    #[allow(clippy::return_self_not_must_use)]
    pub fn animate_on_node<L: Lens<Value = N> + GetValueLens>(
        self,
        speed: Option<<L::Value as Tweenable>::Speed>,
    ) -> Self
    where
        L::Value: Tweenable + Clone,
        L::Object: Clone + Component,
    {
        self.advanced(|args, commands| {
            if !args.is_hot() {
                return;
            }

            commands.transition_value::<L>(args.node.clone(), args.node.clone(), speed);
        })
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), NoContext>
{
    #[allow(clippy::return_self_not_must_use)]
    pub fn insert<B: Bundle>(self, b: B) -> Self {
        self.insert_with_node_and_context(|_, _| b)
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext>
{
    #[allow(clippy::return_self_not_must_use)]
    pub fn insert_with_node<B: Bundle>(self, f: impl FnOnce(&'n N) -> B) -> Self {
        self.insert_with_node_and_context(|n, _| f(n))
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C>
{
    #[allow(clippy::return_self_not_must_use)]
    pub fn insert_with_context<B: Bundle>(self, f: impl FnOnce(&'c1 C::Wrapper<'c2>) -> B) -> Self {
        self.insert_with_node_and_context(|_, c| f(c))
    }
}
