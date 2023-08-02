use bevy::prelude::*;
use lazy_static::lazy_static;
use state_hierarchy::transition::prelude::*;
use state_hierarchy::{prelude::*, impl_hierarchy_root};
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

impl_hierarchy_root!(Root);

#[derive(Eq, PartialEq, Debug, Default)]
pub struct CommandGrid;

impl HierarchyNode for CommandGrid {
    type Context = AssetServer;

    fn update<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl HierarchyCommands) {
        commands.insert(NodeBundle {
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

        for command in [Command::AddNew, Command::Reset] {
            let key: &'static str = command.into();

            let node = ButtonNode {
                value: command.to_string(),
                text_node_style: TEXT_NODE_STYLE.clone(),
                button_node_style: BUTTON_NODE_STYLE.clone(),
                marker: command,
            };

            commands.child(key, context, node);
        }
    }

}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct DynamicGrid;


#[derive(Debug, PartialEq, Clone)]
pub struct MenuSlideDeletionPathMaker{page: u8}

impl DeletionPathMaker<StyleLeftLens> for MenuSlideDeletionPathMaker {
    fn get_path(
        &self,
        previous: &<StyleLeftLens as Lens>::Value,
        sibling_keys: &bevy::utils::HashSet<ChildKey>,
    ) -> Option<TransitionPath<StyleLeftLens>> {
        todo!()
    }
}



impl HierarchyNode for DynamicGrid {
    type Context = NC2<UIState, AssetServer>;

    fn update<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl HierarchyCommands) {
        commands.insert(NodeBundle {
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

        for number in context.0.dynamic_buttons.iter().cloned() {
            let node = ButtonNode {
                value: number.to_string(),
                text_node_style: TEXT_NODE_STYLE.clone(),
                button_node_style: BUTTON_NODE_STYLE.clone(),
                marker: DynamicButtonComponent(number),
            };

            let node = node.with_transition_in_out::<(TransformRotationLens, TransformScaleLens)>(
                (Quat::from_rotation_z(-consts::FRAC_PI_8), Vec3::ONE),
                (Quat::default(), Vec3::ONE),
                (Quat::from_rotation_z(consts::FRAC_PI_2), Vec3::ONE * 0.0),
                Duration::from_secs_f32(0.5),
                Duration::from_secs_f32(2.0),
            );

            commands.child(number, &context.1, node);
        }
    }

}

impl HierarchyNode for Root {
    type Context = NC2<UIState, AssetServer>;

    fn update<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl HierarchyCommands) {
        commands.insert(NodeBundle {
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

        commands.child(0, &context.1, CommandGrid);
        commands.child(1, context, DynamicGrid);
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
