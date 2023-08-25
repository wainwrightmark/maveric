use bevy::ecs::system::EntityCommands;

use crate::prelude::*;
pub struct SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext> {
    args: &'n N,
    previous: Option<&'p N>,
    context: &'c1 C::Wrapper<'c2>,
    event: SetEvent,
    world: &'world World,
    ec: &'ec mut EntityCommands<'w, 's, 'a>,
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C>
{
    pub(crate) fn new(
        args: &'n N,
        previous: Option<&'p N>,
        context: &'c1 C::Wrapper<'c2>,
        event: SetEvent,
        world: &'world World,
        ec: &'ec mut EntityCommands<'w, 's, 'a>,
    ) -> Self {
        Self {
            args,
            previous,
            context,
            event,
            world,
            ec,
        }
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
            args: self.args,
            previous: self.previous,
            context: self.context,
            event: self.event,
            world: self.world,
            ec: self.ec,
        };
        f(clone)
    }

    pub fn ignore_args(self) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C> {
        self.map_args(|_| &())
    }

    pub fn ignore_context(
        self,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N2, C> {
        let new_args = map(self.args);

        SetComponentCommands {
            args: new_args,
            previous: self.previous.map(map),
            context: self.context,
            event: self.event,
            world: self.world,
            ec: self.ec,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C2> {
        let new_context = map(self.context);
        SetComponentCommands {
            args: self.args,
            previous: self.previous,
            context: new_context,
            event: self.event,
            world: self.world,
            ec: self.ec,
        }
    }

    /// Returns true if this is a creation or undeletion, or if the context or args have changed
    fn is_hot(&self) -> bool {
        match self.event {
            SetEvent::Created | SetEvent::Undeleted => true,
            SetEvent::Updated => {
                C::has_changed(self.context)
                    || self.previous.map(|p| !p.eq(self.args)).unwrap_or(true)
            }
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C>
{
    pub fn insert_with_args_and_context<B: Bundle>(
        &mut self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>) -> B,
    ) {
        // if self.is_hot(){
        //     info!("insert {}", std::any::type_name::<B>());
        // }

        self.components_advanced(|n, _, c, _, cc| cc.insert(f(n, c)))
    }

    pub fn components_advanced(
        &mut self,

        f: impl FnOnce(&'n N, Option<&'p N>, &'c1 C::Wrapper<'c2>, SetEvent, &mut ComponentCommands),
    ) {
        if self.is_hot() {
            let mut occ = ComponentCommands::new(self.ec, self.world, self.event);
            f(self.args, self.previous, self.context, self.event, &mut occ);
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq + IntoBundle>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext>
{
    pub fn insert_bundle(&mut self) {
        if self.is_hot() {
            //info!("insert {}", std::any::type_name::<N>());
            self.ec.insert(self.args.clone().into_bundle());
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), NoContext>
{
    pub fn insert<B: Bundle>(&mut self, b: B) {
        self.insert_with_args_and_context(|_, _| b)
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext>
{
    pub fn insert_with_args<B: Bundle>(&mut self, f: impl FnOnce(&'n N) -> B) {
        self.insert_with_args_and_context(|n, _| f(n))
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C>
{
    pub fn insert_with_context<B: Bundle>(&mut self, f: impl FnOnce(&'c1 C::Wrapper<'c2>) -> B) {
        self.insert_with_args_and_context(|_, c| f(c))
    }
}
