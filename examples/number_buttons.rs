use bevy::prelude::*;
use maveric::prelude::*;

use std::f32::consts;
use std::string::ToString;
use std::time::Duration;
use strum::Display;
use strum::IntoStaticStr;

const DYNAMIC_BOX_WIDTH: f32 = 150.0;
const DYNAMIC_BOX_HEIGHT: f32 = 65.0;
const BOXES_PER_ROW: usize = 5;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .register_maveric::<Root>()
        .init_resource::<UIState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system);

    app.register_transition::<(TransformRotationZLens, TransformScaleLens)>();

    app.run();
}

#[derive(Debug, Clone, Resource, Default)]
pub struct UIState {
    pub next_button: u32,
    pub dynamic_buttons: Vec<u32>,
}

impl UIState {
    pub fn remove_or_readd(&mut self, next_number: u32) {
        match self.dynamic_buttons.binary_search(&next_number) {
            Ok(index) => {
                self.dynamic_buttons.remove(index);
                //info!("Removed button {index}");
            }
            Err(index) => self.dynamic_buttons.insert(index, next_number),
        }
    }

    pub fn reset(&mut self) {
        self.dynamic_buttons.clear();
        self.next_button = 0;
    }

    pub fn add(&mut self) {
        self.dynamic_buttons.push(self.next_button);
        //info!("Added button {}", self.next_index);
        self.next_button += 1;
    }
}

pub fn get_button_left_top(state: &UIState, number: &u32) -> (Val, Val) {
    let index = state
        .dynamic_buttons
        .binary_search(number)
        .map_or_else(|e| e, |f| f);

    let top = Val::Px((DYNAMIC_BOX_HEIGHT * (index / BOXES_PER_ROW) as f32) + 300.);
    let left = Val::Px(DYNAMIC_BOX_WIDTH * (index % BOXES_PER_ROW) as f32);

    (left, top)
}

#[derive(Debug, Eq, PartialEq, Component, Hash, Clone, Copy, IntoStaticStr, Display)]
pub enum Command {
    AddNew,
    Reset,
}

#[derive(Debug, Eq, PartialEq, Component, Hash, Clone, Copy)]
pub struct DynamicButtonComponent(u32);

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Root;

impl MavericRoot for Root {
    type Context<'w, 's> = Res<'w, UIState>;

    fn set_children(context: &Self::Context<'_, '_>, commands: &mut impl ChildCommands) {
        commands.add_child(0, CommandGrid, &());
        commands.add_child(1, DynamicGrid, context);
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct CommandGrid;

impl MavericNode for CommandGrid {
    type Context<'w, 's> = ();

    fn set_components(commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {
        commands.ignore_node().ignore_context().insert(NodeBundle {
            style: Style {
                height: Val::Percent(10.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        });
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>) {
        commands
            .ignore_node()
            .unordered_children_with_context(|context, commands| {
                for command in [Command::AddNew, Command::Reset] {
                    let key: &'static str = command.into();
                    let node = ButtonNode {
                        style: ButtonStyle,
                        visibility: Visibility::Visible,
                        border_color: BUTTON_BORDER,
                        background_color: TEXT_BUTTON_BACKGROUND,
                        marker: command,
                        children: (TextNode {
                            text: command.to_string(),
                            font: FONT_PATH,
                            font_size: BUTTON_FONT_SIZE,
                            color: BUTTON_TEXT_COLOR,
                            justify_text: JustifyText::Center,
                            linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                        },),
                    };

                    commands.add_child(key, node, &context);
                }
            })
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct DynamicGrid;

impl MavericNode for DynamicGrid {
    type Context<'w, 's> = Res<'w, UIState>;

    fn set_components(commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {
        commands.ignore_node().ignore_context().insert(NodeBundle {
            style: Style {
                top: Val::Percent(10.0),
                height: Val::Percent(90.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        });
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>) {
        commands
            .ignore_node()
            .ordered_children_with_context(|context, commands| {
                for number in context.dynamic_buttons.iter().cloned() {
                    let node = ButtonNode {
                        style: ButtonStyle,
                        visibility: Visibility::Visible,
                        border_color: BUTTON_BORDER,
                        background_color: TEXT_BUTTON_BACKGROUND,
                        marker: DynamicButtonComponent(number),
                        children: (TextNode {
                            text: number.to_string(),
                            font: FONT_PATH,
                            font_size: BUTTON_FONT_SIZE,
                            color: BUTTON_TEXT_COLOR,
                            justify_text: JustifyText::Center,
                            linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                        },),
                    };

                    let node = node
                        .with_transition_in_out::<(TransformRotationZLens, TransformScaleLens)>(
                            (-consts::FRAC_PI_8, Vec3::ONE),
                            (0.0, Vec3::ONE),
                            (consts::FRAC_PI_2, Vec3::ZERO),
                            Duration::from_secs_f32(0.5),
                            Duration::from_secs_f32(2.0),
                            Some(Ease::CubicIn),
                            Some(Ease::CubicIn),
                        );

                    commands.add_child(number, node, &());
                }
            })
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&Command>,
            Option<&DynamicButtonComponent>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<UIState>,
) {
    for (interaction, mut color, mut border_color, command, dynamic_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                if let Some(command) = command {
                    match command {
                        Command::AddNew => state.add(),
                        Command::Reset => state.reset(),
                    }
                };

                if let Some(DynamicButtonComponent(index)) = dynamic_button {
                    state.remove_or_readd(*index)
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonStyle;
impl IntoBundle for ButtonStyle {
    type B = Style;

    fn into_bundle(self) -> Self::B {
        Style {
            width: Val::Px(DYNAMIC_BOX_WIDTH),
            height: Val::Px(DYNAMIC_BOX_HEIGHT),
            border: UiRect::all(Val::Px(5.0)),
            position_type: PositionType::Relative,
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..Default::default()
        }
    }
}

pub const TEXT_BUTTON_WIDTH: f32 = 360.;
pub const TEXT_BUTTON_HEIGHT: f32 = 60.;

pub const UI_BORDER_WIDTH: Val = Val::Px(3.0);

pub const FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";

pub const BUTTON_FONT_SIZE: f32 = 32.0;
pub const BUTTON_BORDER: Color = Color::BLACK;
pub const BUTTON_TEXT_COLOR: Color = Color::WHITE;

pub const TEXT_BUTTON_BACKGROUND: Color = Color::WHITE;
