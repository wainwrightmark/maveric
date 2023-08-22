use crate::prelude::*;
use bevy::ecs::system::EntityCommands;

pub struct SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N: PartialEq, C: NodeContext>
{
    args: &'n N,
    previous: Option<&'p N>,
    context: &'c1 C::Wrapper<'c2>,
    event: SetEvent,
    commands: EntityCommands<'w, 's, 'a>,
    world: &'world World,
}

pub struct SetComponentsFinishToken<'w, 's, 'a, 'world> {
    pub (crate) commands: EntityCommands<'w, 's, 'a>,
    pub (crate) world: &'world World,
}


impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N: PartialEq, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, C>
{
    pub(crate) fn new(
        args: &'n N,
        previous: Option<&'p N>,
        context: &'c1 C::Wrapper<'c2>,
        event: SetEvent,
        commands: EntityCommands<'w, 's, 'a>,
        world: &'world World,
    ) -> Self {
        Self {
            args,
            previous,
            context,
            event,
            commands,
            world,
        }
    }

    pub fn scope(self, f: impl FnOnce(Self)-> SetComponentsFinishToken<'w,'s,'a,'world>)-> Self{
        let (args, previous, context, event)= (self.args, self.previous, self.context, self.event).clone();
        let token = f(self);
        Self { args,previous,context, event, commands: token.commands, world: token.world }
    }

    pub fn finish(self)-> SetComponentsFinishToken<'w,'s,'a,'world>{
        SetComponentsFinishToken { commands: self.commands, world: self.world }
    }

    pub fn ignore_args(self) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, (), C> {
        self.map_args(|_| &())
    }

    pub fn ignore_context(
        self,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, NoContext> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N2, C> {
        let new_args = map(self.args);

        SetComponentCommands {
            args: new_args,
            previous: self.previous.map(map),
            context: self.context,
            event: self.event,
            commands: self.commands,
            world: self.world,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, C2> {
        let new_context = map(self.context);
        SetComponentCommands {
            args: self.args,
            previous: self.previous,
            context: new_context,
            event: self.event,
            commands: self.commands,
            world: self.world,
        }
    }

    /// Returns true if this is a creation or undeletion, or if the context or args have changed
    fn is_hot(&self) -> bool {
        match self.event {
            SetEvent::Created | SetEvent::Undeleted => return true,
            SetEvent::Updated => {
                C::has_changed(self.context)
                    || self.previous.map(|p| !p.eq(self.args)).unwrap_or(true)
            }
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N: PartialEq, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, C>
{
    pub fn insert_with_args_and_context<B : Bundle>(
        self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>)-> B,
    ) -> Self {
        self.components_advanced(|n, _, c, _, cc| cc.insert(f(n, c)) )
    }

    pub fn components_advanced(
        mut self,

        f: impl FnOnce(
            &'n N,
            Option<&'p N>,
            &'c1 C::Wrapper<'c2>,
            SetEvent,
            &mut ConcreteComponentCommands,
        ),
    )-> Self {
        if !self.is_hot() {
            return self;
        }
        let mut occ = ConcreteComponentCommands::new(&mut self.commands, self.world);
        f(self.args, self.previous, self.context, self.event, &mut occ);

        self
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, (), NoContext>
{
    pub fn insert<B: Bundle>(self, b: B) -> Self {
        self.insert_with_args_and_context(|_, _| b)
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N: PartialEq>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, NoContext>
{
    pub fn insert_with_args<B: Bundle>(
        self,
        f: impl FnOnce(&'n N)-> B,
    ) -> Self {
        self.insert_with_args_and_context(|n, _| f(n))
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, C: NodeContext>
    SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, (), C>
{
    pub fn insert_with_context<B: Bundle>(
        self,

        f: impl FnOnce(&'c1 C::Wrapper<'c2>)-> B,
    ) -> Self {
        self.insert_with_args_and_context(|_, c| f(c))
    }
}
