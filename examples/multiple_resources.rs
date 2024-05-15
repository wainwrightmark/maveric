use bevy::prelude::*;
use maveric::{define_lens_transparent, prelude::*};
use maveric_macro::NodeContext;

use std::string::ToString;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .init_resource::<CounterState>()
        .init_resource::<ColorState>()
        .register_resource_transition::<ClearColorLens>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .add_systems(Update, clear_color_transition)
        .register_maveric::<Root>();
    app.run();
}
fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}

define_lens_transparent!(ClearColorLens, ClearColor, Color);

fn clear_color_transition(
    color_state: Res<ColorState>,
    mut clear_transition: ResMut<ResourceTransition<ClearColorLens>>,
) {
    if color_state.is_changed() {
        let new_color = color_state.get_clear_color();
        clear_transition.transition = Some(Transition::ThenEase {
            destination: new_color,
            speed: 1.0.into(),
            ease: Ease::CubicInOut,
            next: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Default, MavericRoot)]
pub struct Root;


#[derive(NodeContext)]
pub struct MyContext {
    pub counter_state: CounterState,
    pub color_state: ColorState,
}

impl MavericRootChildren for Root {
    type Context = MyContext;

    fn set_children<'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'_, '_>,
        commands: &mut impl ChildCommands,
    ) {
        let text = context.counter_state.number.to_string();
        let (color, color_name) = context.color_state.get_color_and_name();
        commands.add_child(
            0,
            ButtonNode {
                style: ButtonStyle { top: Val::Px(0.0) },
                background_color: color,
                border_color: BUTTON_BORDER,
                visibility: Visibility::Visible,
                marker: ButtonMarker::Number,
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
        );

        commands.add_child(
            1,
            ButtonNode {
                style: ButtonStyle {
                    top: Val::Px(100.0),
                },
                background_color: color,
                border_color: BUTTON_BORDER,
                visibility: Visibility::Visible,
                marker: ButtonMarker::Color,
                children: (TextNode {
                    text: color_name,
                    font_size: BUTTON_FONT_SIZE,
                    color: BUTTON_TEXT_COLOR,
                    font: FONT_PATH,
                    justify_text: JustifyText::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },),
            },
            &(),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Resource, Default, MavericContext)]
pub struct CounterState {
    number: usize,
}

#[derive(Debug, Clone, PartialEq, Resource, Default, MavericContext)]
pub struct ColorState {
    color_index: usize,
}

impl ColorState {
    pub fn get_color_and_name(&self) -> (Color, &'static str) {
        match self.color_index % 4 {
            0 => (Color::RED, "Red"),
            1 => (Color::GREEN, "Green"),
            2 => (Color::BLUE, "Blue"),
            _ => (Color::GOLD, "Gold"),
        }
    }

    pub fn get_clear_color(&self) -> Color {
        match self.color_index % 4 {
            0 => Color::Rgba {
                red: 0.6,
                green: 0.4,
                blue: 0.4,
                alpha: 1.0,
            },
            1 => Color::Rgba {
                red: 0.4,
                green: 0.6,
                blue: 0.4,
                alpha: 1.0,
            },
            2 => Color::Rgba {
                red: 0.4,
                green: 0.4,
                blue: 0.6,
                alpha: 1.0,
            },
            _ => Color::Rgba {
                red: 0.6,
                green: 0.6,
                blue: 0.2,
                alpha: 1.0,
            },
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &ButtonMarker),
        (Changed<Interaction>, With<Button>),
    >,
    mut counter_state: ResMut<CounterState>,
    mut color_state: ResMut<ColorState>,
) {
    for (interaction, marker) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => match marker {
                ButtonMarker::Number => counter_state.number += 1,
                ButtonMarker::Color => color_state.color_index += 1,
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Component)]
pub enum ButtonMarker {
    Number,
    Color,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonStyle {
    pub top: Val,
}
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
            top: self.top,

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
