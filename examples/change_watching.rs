use bevy::prelude::*;
use maveric::{helpers::ScheduledForDeletion, prelude::*};

use std::{string::ToString, time::Duration};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .init_resource::<CounterState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .register_maveric::<Root>();

    app.register_transition::<TransformScaleLens>();
    app.register_transition::<TransformTranslationLens>();
    app.run();
}
fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct Marker;

#[derive(Debug, Clone, PartialEq, Default, MavericRoot)]
pub struct Root;

impl MavericRootChildren for Root {
    type Context = CounterState;

    fn set_children<'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        let text = context.number.to_string();
        commands.add_child(
            0,
            ButtonNode {
                style: ButtonStyle,
                background_color: TEXT_BUTTON_BACKGROUND,
                border_color: BUTTON_BORDER,
                visibility: Visibility::Visible,
                marker: Marker,
                children: (TextNode {
                    text,
                    font_size: BUTTON_FONT_SIZE,
                    color: BUTTON_TEXT_COLOR,
                    font: FONT_PATH,
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },),
            },
            &(),
        );
        commands.add_child(
            1,
            ChangeWatcher {
                number: context.number,
            },
            &(),
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeWatcher {
    number: usize,
}

impl MavericNode for ChangeWatcher {
    type Context = ();

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.insert_static_bundle(SpatialBundle::default())
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        commands.no_children()
    }

    fn on_changed(
        &self,
        _previous: &Self,
        _context: &<Self::Context as NodeContext>::Wrapper<'_>,
        world: &World,
        entity_commands: &mut bevy::ecs::system::EntityCommands,
    ) {
        entity_commands.with_children(|cb| {
            let asset_server = world.resource::<AssetServer>();

            cb.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("{}", self.number),
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 128.0,
                            color: Color::GREEN,
                        },
                    ),
                    ..default()
                },
                ScheduledForDeletion {
                    remaining: Duration::from_secs_f32(2.0),
                },
                TransitionBuilder::<TransformScaleLens>::default()
                    .then_tween(Vec3::ZERO, 2.0.into())
                    .build(),
                TransitionBuilder::<TransformTranslationLens>::default()
                    .then_ease(
                        Vec3 {
                            x: 0.0,
                            y: 500.0,
                            z: 0.0,
                        },
                        500.0.into(),
                        Ease::CircIn,
                    )
                    .build(),
            ));
        });
    }
}

#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct CounterState {
    number: usize,
}

impl MavericContext for CounterState {}

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<CounterState>,
) {
    for interaction in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                state.number += 1;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonStyle;
impl IntoBundle for ButtonStyle {
    type B = Style;

    fn into_bundle(self) -> Self::B {
        Style {
            width: Val::Px(TEXT_BUTTON_WIDTH),
            height: Val::Px(TEXT_BUTTON_HEIGHT),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            border: UiRect::all(UI_BORDER_WIDTH),

            ..Default::default()
        }
    }
}

pub const TEXT_BUTTON_WIDTH: f32 = 360.;
pub const TEXT_BUTTON_HEIGHT: f32 = 60.;

pub const UI_BORDER_WIDTH: Val = Val::Px(3.0);

pub const FONT_PATH: &str = "fonts/merged-font.ttf";

pub const BUTTON_FONT_SIZE: f32 = 22.0;
pub const BUTTON_BORDER: Color = Color::BLACK;
pub const BUTTON_TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const TEXT_BUTTON_BACKGROUND: Color = Color::WHITE;
