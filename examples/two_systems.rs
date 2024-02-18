use bevy::prelude::*;
use maveric::prelude::*;

use std::string::ToString;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .init_resource::<CounterState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .register_maveric::<Root>()
        .register_maveric::<Root2>();
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
                style: TextButtonStyle,
                background_color: TEXT_BUTTON_BACKGROUND,
                border_color: BUTTON_BORDER,
                visibility: Visibility::Visible,
                marker: Marker,
                children: (TextNode {
                    text,
                    font_size: BUTTON_FONT_SIZE,
                    color: BUTTON_TEXT_COLOR,
                    font: FONT_PATH,
                    justify_text: JustifyText::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },),
            },
            &(),
        )
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

#[derive(Debug, Clone, PartialEq, Default, MavericRoot)]
pub struct Root2;

impl MavericRootChildren for Root2 {
    type Context = CounterState;

    fn set_children(
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        let path = match context.number % 4 {
            0 => r#"images\MedalsBlack.png"#,
            1 => r#"images\MedalsBronze.png"#,
            2 => r#"images\MedalsSilver.png"#,
            _ => r#"images\MedalsGold.png"#,
        };

        commands.add_child(
            0,
            ButtonNode {
                style: ImageStyle,
                background_color: TEXT_BUTTON_BACKGROUND,
                border_color: BUTTON_BORDER,
                visibility: Visibility::Visible,
                marker: Marker,
                children: (ImageNode {
                    style: ImageStyle,
                    path,
                    background_color: Color::WHITE,
                },),
            },
            &(),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageStyle;
impl IntoBundle for ImageStyle {
    type B = Style;

    fn into_bundle(self) -> Self::B {
        Style {
            width: Val::Px(BUTTON_HEIGHT * 2.0),
            height: Val::Px(BUTTON_HEIGHT),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            border: UiRect::all(UI_BORDER_WIDTH),
            ..default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextButtonStyle;
impl IntoBundle for TextButtonStyle {
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

pub const BUTTON_WIDTH: f32 = 256.;
pub const BUTTON_HEIGHT: f32 = 128.;

pub const UI_BORDER_WIDTH: Val = Val::Px(3.0);

pub const FONT_PATH: &str = "fonts/merged-font.ttf";

pub const BUTTON_FONT_SIZE: f32 = 22.0;
pub const BUTTON_BORDER: Color = Color::BLACK;
pub const BUTTON_TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const TEXT_BUTTON_BACKGROUND: Color = Color::WHITE;
