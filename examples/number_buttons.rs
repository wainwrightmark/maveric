use bevy::prelude::*;
use lazy_static::lazy_static;
use state_hierarchy::transition::prelude::*;
use state_hierarchy::transition::speed::{AngularSpeed, LinearSpeed};
use state_hierarchy::{prelude::*, register_state_tree, widgets::prelude::*};
use std::f32::consts;
use std::time::Duration;
use std::{string::ToString, sync::Arc};
use strum::Display;
use strum::IntoStaticStr;

const DYNAMIC_BOX_WIDTH: f32 = 150.0;
const DYNAMIC_BOX_HEIGHT: f32 = 65.0;
const BOXES_PER_ROW: usize = 5;

lazy_static! {
 static ref  BUTTON_NODE_STYLE: Arc<ButtonNodeStyle> = Arc::new(ButtonNodeStyle {
    style: Style {
        width: Val::Px(DYNAMIC_BOX_WIDTH),
        height: Val::Px(DYNAMIC_BOX_HEIGHT),
        border: UiRect::all(Val::Px(5.0)),
        position_type: PositionType::Relative,
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..Default::default()
    },
    visibility: Visibility::Visible,
    border_color: Color::BLUE,
    background_color: Color::WHITE,
});
}

lazy_static! {
    static ref TEXT_NODE_STYLE: Arc<TextNodeStyle> = Arc::new(TextNodeStyle {
        font_size: 32.0,
        color: Color::WHITE,
        font: "fonts/FiraSans-Bold.ttf",
    });
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_resource::<UIState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system);

    //app.add_plugins(TransitionPlugin::<Prism2<TransformRotationLens, QuatZLens>>::default());
    app.add_plugins(TransitionPlugin::<(
        TransformRotationLens,
        TransformScaleLens,
    )>::default());

    register_state_tree::<Root>(&mut app);
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
        .binary_search(&number)
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

impl HierarchyRoot for Root {
    type ContextParam<'c> = (Res<'c, UIState>, Res<'c, AssetServer>);

    fn get_context<'a, 'c, 'w: 'c, 's>(
        param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> Self::Context<'c> {
        param.into_inner()
    }
}
#[derive(Eq, PartialEq, Debug, Default)]
pub struct CommandGrid;

impl HierarchyNode for CommandGrid {
    type Context<'c> = Res<'c, AssetServer>;

    fn get_components<'c>(
        &self,
        _context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    ) {
        component_commands.insert(NodeBundle {
            style: Style {
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

    fn get_children<'c>(
        &self,
        context: &Self::Context<'c>,
        child_commands: &mut impl ChildCommands,
    ) {
        for command in [Command::AddNew, Command::Reset] {
            let key: &'static str = command.into();

            let node = ButtonNode {
                value: command.to_string(),
                text_node_style: TEXT_NODE_STYLE.clone(),
                button_node_style: BUTTON_NODE_STYLE.clone(),
                marker: command,
            };

            child_commands.add(key, context, node);
        }
    }

    fn on_deleted(&self, _component_commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct DynamicGrid;

impl HierarchyNode for DynamicGrid {
    type Context<'c> = (Res<'c, UIState>, Res<'c, AssetServer>);

    fn get_components<'c>(
        &self,
        _context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    ) {
        component_commands.insert(NodeBundle {
            style: Style {
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

    fn get_children<'c>(
        &self,
        context: &Self::Context<'c>,
        child_commands: &mut impl ChildCommands,
    ) {
        for number in context.0.dynamic_buttons.iter().cloned() {
            let node = ButtonNode {
                value: number.to_string(),
                text_node_style: TEXT_NODE_STYLE.clone(),
                button_node_style: BUTTON_NODE_STYLE.clone(),
                marker: DynamicButtonComponent(number),
            };

            let node = node.with_transition_in_out::<(TransformRotationLens, TransformScaleLens)>(
                Transform::from_rotation(Quat::from_rotation_z(-consts::FRAC_PI_8)),
                (Quat::default(), Vec3::ONE),
                (Quat::from_rotation_z(consts::FRAC_PI_2), Vec3::ONE * 0.0),
                Duration::from_secs_f32(0.5),
                Duration::from_secs_f32(2.0),
            );

            child_commands.add(number, &context.1, node);
        }
    }

    fn on_deleted(&self, _component_commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}

impl HierarchyNode for Root {
    type Context<'c> = (Res<'c, UIState>, Res<'c, AssetServer>);

    fn get_components<'b>(
        &self,
        _context: &Self::Context<'b>,
        component_commands: &mut impl ComponentCommands,
    ) {
        component_commands.insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        });
    }

    fn get_children<'b>(
        &self,
        context: &Self::Context<'b>,
        child_commands: &mut impl ChildCommands,
    ) {
        child_commands.add(0, &context.1, CommandGrid);
        child_commands.add(1, context, DynamicGrid);
    }

    fn on_deleted(&self, _component_commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
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