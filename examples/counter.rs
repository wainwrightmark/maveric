use bevy::prelude::*;
use maveric::{impl_maveric_root, prelude::*};

use std::string::ToString;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_resource::<CounterState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .register_maveric::<Root>();
    app.run();
}
fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct Marker;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Root;

impl MavericRootChildren for Root {
    type Context = NC2<CounterState, AssetServer>;

    fn set_children<'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        let text = context.0.number.to_string();
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
            &context.1,
        )
    }
}

impl_maveric_root!(Root);

#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct CounterState {
    number: usize,
}

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
