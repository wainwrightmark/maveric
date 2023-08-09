#[macro_export]
macro_rules! impl_static_components {
    ($T:ty, $bundle:expr ) => {
        impl ComponentsAspect for $T{
            fn set_components<'r>(
                &self,
                _context: &<Self::Context as NodeContext>::Wrapper<'r>,
                commands: &mut impl ComponentCommands,
                event: SetComponentsEvent,
            ) {
                if event == SetComponentsEvent::Created{
                    commands.insert($bundle)
                }
            }
        }
    };
}








