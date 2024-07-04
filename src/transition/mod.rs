pub mod deletion_path_maker;
pub mod ease;
pub mod lens;
pub mod lenses;
pub mod next_step;
pub mod plugin;
pub mod speed;
pub mod step;
pub mod transition_value;
pub mod tweenable;
pub mod with;

#[cfg(any(feature = "bevy_text", test))]
pub mod text_lenses;
pub mod transition_builder;
#[cfg(any(feature = "bevy_ui", test))]
pub mod ui_lenses;

pub mod prelude {
    pub use crate::transition::deletion_path_maker::*;
    pub use crate::transition::lens::*;
    pub use crate::transition::lenses::*;

    pub use crate::transition::plugin::*;
    pub use crate::transition::step::*;
    pub use crate::transition::transition_builder::*;

    pub use crate::transition::ease::*;
    pub use crate::transition::tweenable::*;
    pub use crate::transition::with::*;

    #[cfg(any(feature = "bevy_ui", test))]
    pub use crate::transition::ui_lenses::*;

    #[cfg(any(feature = "bevy_text", test))]
    pub use crate::transition::text_lenses::*;
}

#[cfg(test)]
mod tests {
    #![allow(clippy::nursery)]
    use super::speed::calculate_speed;

    use crate::{transition::prelude::*, transition::speed::*, widgets::prelude::*};
    use bevy::{prelude::*, time::TimePlugin, time::TimeUpdateStrategy};
    use std::{fmt::Debug, time::Duration};

    #[test]
    pub fn test_calculate_speed() {
        let actual = calculate_speed::<f32>(&-1.0, &2.0, Duration::from_secs_f32(1.5));
        assert_eq!(actual, ScalarSpeed::new(2.0));
    }

    #[test]
    pub fn test_transition() {
        let mut value = -10.0;

        let result = <f32 as Tweenable>::transition_towards(&mut value, &10.0, &20.0.into(), 0.5);
        assert_eq!(result, None);
        assert_eq!(value, 0.0);
    }

    #[test]
    pub fn test_complete_transition() {
        let mut value = -1.0;
        let result =
            <f32 as Tweenable>::transition_towards(&mut value, &1.0, &ScalarSpeed::new(1.0), 3.0);

        assert_eq!(result, Some(1.0));
        assert_eq!(value, 1.0);
    }

    #[test]
    pub fn test_transition_transform() {
        let mut app = App::new();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();

        fn spawn(mut commands: Commands) {
            commands.spawn_empty().insert((
                Transform::default(),
                TransitionBuilder::<TransformTranslationLens>::default()
                    .then_tween(Vec3::X * 2.0, 10.0.into())
                    .build(),
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
    pub fn test_transition_wait() {
        let mut app = App::new();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();

        fn spawn(mut commands: Commands) {
            commands.spawn_empty().insert((
                Transform::default(),
                TransitionBuilder::<TransformTranslationLens>::default()
                    .then_wait(Duration::from_secs_f32(0.2))
                    .then_tween(Vec3::X * 2.0, 10.0.into())
                    .build(),
            ));
        }

        app.add_systems(Startup, spawn);
        assert_sequence(
            &mut app,
            &[
                [Transform::default()],
                [Transform::default()],
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
    pub fn test_transition_transform_set_value() {
        let mut app = App::new();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();

        fn spawn(mut commands: Commands) {
            commands.spawn_empty().insert((
                Transform::default(),
                TransitionBuilder::<TransformTranslationLens>::default()
                    .then_tween(Vec3::X * 10.0, 40.0.into())
                    .then_set_value(Vec3::ZERO)
                    .then_tween(Vec3::Y * 10.0, 50.0.into())
                    .build(),
            ));
        }

        app.add_systems(Startup, spawn);

        assert_sequence(
            &mut app,
            &[
                [Transform::default()],
                [Transform::from_translation(Vec3::X * 4.0)],
                [Transform::from_translation(Vec3::X * 8.0)],
                [Transform::from_translation(Vec3::Y * 2.5)],
                //[Transform::from_translation(Vec3::default())],
                [Transform::from_translation(Vec3::Y * 7.5)],
                [Transform::from_translation(Vec3::Y * 10.0)],
                [Transform::from_translation(Vec3::Y * 10.0)],
            ],
            "step",
        );
    }

    #[test]
    pub fn test_transition_cyclic() {
        let mut app = App::new();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )));
        app.add_plugins(TimePlugin);
        app.register_transition::<TransformTranslationLens>();

        fn spawn(mut commands: Commands) {
            commands.spawn_empty().insert((
                Transform::default(),
                TransitionBuilder::<TransformTranslationLens>::default()
                    .then_tween(Vec3::X * 3.0, 10.0.into())
                    .then_tween(Vec3::X * 1.0, 10.0.into())
                    .build_loop(),
            ));
        }

        app.add_systems(Startup, spawn);

        assert_sequence(
            &mut app,
            &[
                [Transform::default()],
                [Transform::from_translation(Vec3::X)],
                [Transform::from_translation(Vec3::X * 2.0)],
                [Transform::from_translation(Vec3::X * 3.0)],
                [Transform::from_translation(Vec3::X * 2.0)],
                [Transform::from_translation(Vec3::X * 1.0)],
                [Transform::from_translation(Vec3::X * 2.0)],
                [Transform::from_translation(Vec3::X * 3.0)],
                [Transform::from_translation(Vec3::X * 2.0)],
                [Transform::from_translation(Vec3::X * 1.0)],
                [Transform::from_translation(Vec3::X * 2.0)],
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
        app.world_mut()
            .query::<&T>()
            .iter(&app.world())
            .cloned()
            .collect()
    }

    #[test]
    fn test_transition_in_out() {
        #[derive(Debug, Resource)]
        struct ShouldHaveNodeResource(bool);

        #[derive(Debug)]
        struct MyRoot;

        impl MavericRoot for MyRoot {
            type Context<'w, 's> = Res<'w, ShouldHaveNodeResource>;

            fn set_children(
                context: &Self::Context<'_, '_>,
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
                            None,
                            None,
                        );

                    commands.add_child(0, child, &());
                }
            }
        }

        let mut app = App::new();
        app.insert_resource(ShouldHaveNodeResource(true));
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            100,
        )));
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

        app.world_mut().resource_mut::<ShouldHaveNodeResource>().0 = false;

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
