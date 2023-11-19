pub mod deletion_path_maker;
pub mod lens;
pub mod lenses;
pub mod next_step;
pub mod plugin;
pub mod speed;
pub mod step;
pub mod transition_value;
pub mod tweenable;
pub mod with;

#[cfg(any(feature = "bevy_ui", test))]
pub mod ui_lenses;

pub mod prelude {
    pub use crate::transition::deletion_path_maker::*;
    pub use crate::transition::lens::*;
    pub use crate::transition::lenses::*;
    pub use crate::transition::next_step::*;
    pub use crate::transition::plugin::*;
    pub use crate::transition::step::*;
    pub use crate::transition::transition_value::*;
    pub use crate::transition::tweenable::*;
    pub use crate::transition::with::*;

    #[cfg(any(feature = "bevy_ui", test))]
    pub use crate::transition::ui_lenses::*;
}

#[cfg(test)]
mod tests {
    #![allow(clippy::nursery)]
    use std::{fmt::Debug, time::Duration};

    use bevy::{prelude::*, time::TimePlugin, time::TimeUpdateStrategy};

    use crate::{
        impl_maveric_root, transition::prelude::*, transition::speed::*, widgets::prelude::*,
    };

    use super::speed::calculate_speed;

    #[test]
    pub fn test_calculate_speed() {
        let actual = calculate_speed::<f32>(&-1.0, &2.0, Duration::from_secs_f32(1.5));
        assert_eq!(actual, ScalarSpeed::new(2.0));
    }

    #[test]
    pub fn test_transition() {
        let transitioned =
            <f32 as Tweenable>::transition_towards(&-10.0, &10.0, &ScalarSpeed::new(20.0), &0.5);
        assert_eq!(transitioned, 0.0);
    }

    #[test]
    pub fn test_complete_transition() {
        let transitioned =
            <f32 as Tweenable>::transition_towards(&-1.0, &1.0, &ScalarSpeed::new(20.0), &0.5);
        assert_eq!(transitioned, 1.0);
    }

    #[test]
    pub fn test_transition_transform() {
        let mut app = App::new();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();

        fn spawn(mut commands: Commands) {
            commands.spawn_empty().insert((
                Transform::default(),
                Transition::new(TransitionStep::<TransformTranslationLens>::new_arc(
                    Vec3::X * 2.0,
                    Some(LinearSpeed::new(10.0)),
                    NextStep::None,
                )),
            ));
        }

        app.add_systems(Startup, spawn);
        assert_sequence(
            &mut app,
            &[
                [Transform::default()],
                [Transform::from_translation(Vec3::X)],
                [Transform::from_translation(Vec3::X * 2.0)],
                // has not moved any further
                [Transform::from_translation(Vec3::X * 2.0)],
                [Transform::from_translation(Vec3::X * 2.0)],
            ],
            "step",
        );
    }

    #[test]
    pub fn test_transition_transform_two_step() {
        let mut app = App::new();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();

        fn spawn(mut commands: Commands) {
            commands.spawn_empty().insert((
                Transform::default(),
                Transition::new(TransitionStep::<TransformTranslationLens>::new_arc(
                    Vec3::X * 2.0,
                    Some(LinearSpeed::new(10.0)),
                    NextStep::Step(TransitionStep::<TransformTranslationLens>::new_arc(
                        Vec3::default(),
                        None,
                        NextStep::Step(TransitionStep::<TransformTranslationLens>::new_arc(
                            Vec3::Y * 4.0,
                            Some(LinearSpeed::new(20.0)),
                            NextStep::None,
                        )),
                    )),
                )),
            ));
        }

        app.add_systems(Startup, spawn);

        assert_sequence(
            &mut app,
            &[
                [Transform::default()],
                [Transform::from_translation(Vec3::X)],
                [Transform::from_translation(Vec3::X * 2.0)],
                //[Transform::from_translation(Vec3::default())],
                [Transform::from_translation(Vec3::Y * 2.0)],
                [Transform::from_translation(Vec3::Y * 4.0)],
            ],
            "step",
        );
    }

    #[test]
    pub fn test_transition_cyclic() {
        let mut app = App::new();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();

        fn spawn(mut commands: Commands) {
            commands.spawn_empty().insert((
                Transform::default(),
                Transition::new(TransitionStep::<TransformTranslationLens>::new_cycle(
                    [
                        (Vec3::X * 2.0, LinearSpeed::new(10.0)),
                        (Vec3::X * -2.0, LinearSpeed::new(20.0)),
                    ]
                    .into_iter(),
                )),
            ));
        }

        app.add_systems(Startup, spawn);

        assert_sequence(
            &mut app,
            &[
                [Transform::default()],
                [Transform::from_translation(Vec3::X)],
                [Transform::from_translation(Vec3::X * 2.0)],
                [Transform::from_translation(Vec3::default())],
                [Transform::from_translation(Vec3::X * -2.0)],
                [Transform::from_translation(Vec3::X * -1.0)],
            ],
            "step",
        );
    }

    fn assert_sequence<T: Component + Clone + PartialEq + Debug, const COUNT: usize>(
        app: &mut App,
        sequence: &[[T; COUNT]],
        name: &str,
    ) {
        for (index, expected) in sequence.iter().enumerate() {
            app.update();
            assert_components(app, expected, format!("{name} {index}"));
        }
    }

    fn assert_components<T: Component + Clone + PartialEq + Debug>(
        app: &mut App,
        expected: &[T],
        message: String,
    ) {
        let actual: Vec<T> = query_all(app);
        assert_eq!(actual, expected, "{message}");
    }

    fn query_all<T: Component + Clone>(app: &mut App) -> Vec<T> {
        app.world.query::<&T>().iter(&app.world).cloned().collect()
    }

    #[test]
    fn test_transition_in_out() {
        #[derive(Debug, Resource)]
        struct ShouldHaveNodeResource(bool);

        struct MyRoot;

        impl MavericRootChildren for MyRoot {
            type Context = ShouldHaveNodeResource;

            fn set_children(
                context: &<Self::Context as crate::widgets::prelude::NodeContext>::Wrapper<'_>,
                commands: &mut impl crate::widgets::prelude::ChildCommands,
            ) {
                if context.0 {
                    let child = Transform::default()
                        .with_transition_in_out::<TransformTranslationLens>(
                            Vec3::default(),
                            Vec3::X * 2.0,
                            Vec3::X * -2.0,
                            Duration::from_millis(200),
                            Duration::from_millis(400),
                        );

                    commands.add_child(0, child, &());
                }
            }
        }

        impl_maveric_root!(MyRoot);

        let mut app = App::new();
        app.insert_resource(ShouldHaveNodeResource(true));
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();
        app.register_maveric::<MyRoot>();
        //app.update();

        assert_sequence(
            &mut app,
            &[
                [Transform::default()],
                [Transform::from_translation(Vec3::X)],
                [Transform::from_translation(Vec3::X * 2.0)],
                [Transform::from_translation(Vec3::X * 2.0)],
            ],
            "transition inward",
        );

        app.world.resource_mut::<ShouldHaveNodeResource>().0 = false;

        assert_sequence(
            &mut app,
            &[
                [Transform::from_translation(Vec3::X * 1.0)],
                [Transform::from_translation(Vec3::X * 0.0)],
                [Transform::from_translation(Vec3::X * -1.0)],
            ],
            "transition outward",
        );

        app.update();

        assert_sequence::<Transform, 0>(&mut app, &[[], []], "after deleted");
    }
}
