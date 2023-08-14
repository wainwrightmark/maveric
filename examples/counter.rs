use bevy::prelude::*;
use lazy_static::lazy_static;
use state_hierarchy::{impl_hierarchy_root, prelude::*};

use std::{string::ToString, sync::Arc};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_resource::<CounterState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .register_state_hierarchy::<Root>();
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

impl HasContext for Root {
    type Context = NC2<CounterState, AssetServer>;
}

impl ChildrenAspect for Root {
    fn set_children(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        let text = context.0.number.to_string();
        commands.add_child(
            0,
            TextButtonNode {
                text,
                text_node_style: TEXT_BUTTON_TEXT_STYLE.clone(),
                button_node_style: TEXT_BUTTON_STYLE.clone(),
                marker: Marker,
            },
            &context.1,
        )
    }
}

impl_hierarchy_root!(Root);

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

lazy_static! {
    static ref TEXT_BUTTON_STYLE: Arc<ButtonNodeStyle> = Arc::new(ButtonNodeStyle {
        style: Style {
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
        },
        background_color: TEXT_BUTTON_BACKGROUND,
        border_color: BUTTON_BORDER,
        ..Default::default()
    });
    static ref TEXT_BUTTON_TEXT_STYLE: Arc<TextNodeStyle> = Arc::new(TextNodeStyle {
        font_size: BUTTON_FONT_SIZE,
        color: BUTTON_TEXT_COLOR,
        font: FONT_PATH,
        alignment: TextAlignment::Center,
        linebreak_behavior: bevy::text::BreakLineOn::NoWrap
    });
}

pub const ICON_BUTTON_WIDTH: f32 = 65.;
pub const ICON_BUTTON_HEIGHT: f32 = 65.;

pub const TEXT_BUTTON_WIDTH: f32 = 360.;
pub const TEXT_BUTTON_HEIGHT: f32 = 60.;

pub const MENU_OFFSET: f32 = 10.;

pub const UI_BORDER_WIDTH: Val = Val::Px(3.0);

pub const FONT_PATH: &str = "fonts/merged-font.ttf";

pub const ICON_FONT_SIZE: f32 = 30.0;
pub const BUTTON_FONT_SIZE: f32 = 22.0;

pub const BACKGROUND_COLOR: Color = Color::hsla(216., 0.7, 0.72, 1.0); // #86AEEA
pub const ACCENT_COLOR: Color = Color::hsla(218., 0.69, 0.62, 1.0); // #5B8BE2
pub const WARN_COLOR: Color = Color::hsla(0., 0.81, 0.51, 1.0); // #FF6E5F
pub const TIMER_COLOR: Color = Color::BLACK;

pub const LEVEL_TEXT_COLOR: Color = Color::DARK_GRAY;
pub const LEVEL_TEXT_ALT_COLOR: Color = Color::WHITE;

pub const BUTTON_BORDER: Color = Color::BLACK;
pub const BUTTON_TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const ICON_BUTTON_BACKGROUND: Color = Color::NONE;
pub const TEXT_BUTTON_BACKGROUND: Color = Color::WHITE;
pub const DISABLED_BUTTON_BACKGROUND: Color = Color::GRAY;
