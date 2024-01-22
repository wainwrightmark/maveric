use bevy::prelude::*;
use maveric::prelude::*;
use maveric_macro::NodeContext;

use std::string::ToString;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .init_resource::<CounterState>()
        .init_resource::<ColorState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .register_maveric::<Root>();
    app.run();
}
fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct Root;

impl MavericRoot for Root {
    type ContextParam<'c> = <<Self as maveric::prelude::MavericRootChildren>::Context as maveric::prelude::NodeContext>::Wrapper<'c>;

    fn get_context<'a, 'w, 's>(
        param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> <Self::Context as NodeContext>::Wrapper<'w> {
        param.into_inner()
    }
}

#[derive(NodeContext)]
pub struct MyContext {
    pub counter_state: CounterState,
    pub color_state: ColorState
}

impl MavericRootChildren for Root {
    type Context = MyContext;

    fn set_children<'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        let text = context.counter_state.number.to_string();
        let (color, color_name) = get_color_and_name(context.color_state.color_index);
        commands.add_child(
            0,
            ButtonNode {
                style: ButtonStyle{top: Val::Px(0.0)},
                background_color: color,
                border_color: BUTTON_BORDER,
                visibility: Visibility::Visible,
                marker: ButtonMarker::Number,
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
            ButtonNode {
                style: ButtonStyle{top: Val::Px(100.0)},
                background_color: color,
                border_color: BUTTON_BORDER,
                visibility: Visibility::Visible,
                marker: ButtonMarker::Color,
                children: (TextNode {
                    text: color_name,
                    font_size: BUTTON_FONT_SIZE,
                    color: BUTTON_TEXT_COLOR,
                    font: FONT_PATH,
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },),
            },
            &(),
        );
    }
}

pub fn get_color_and_name(index: usize)-> (Color, &'static str)
{
    match index % 4{
        0=> (Color::RED, "Red"),
        1=> (Color::GREEN, "Green"),
        2=> (Color::BLUE, "Blue"),
        _=> (Color::GOLD, "Gold"),
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

fn button_system(
    mut interaction_query: Query<(&Interaction, &ButtonMarker), (Changed<Interaction>, With<Button>)>,
    mut counter_state: ResMut<CounterState>,
    mut color_state: ResMut<ColorState>,
) {
    for (interaction, marker) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                match marker{
                    ButtonMarker::Number => counter_state.number += 1,
                    ButtonMarker::Color => color_state.color_index += 1,
                }

            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Component)]
pub enum ButtonMarker{
    Number,
    Color
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonStyle{pub top: Val}
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

