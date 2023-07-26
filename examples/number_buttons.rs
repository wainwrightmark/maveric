use std::{f32::consts, time::Duration};

use bevy::{ecs::system::EntityCommands, prelude::*};
use state_hierarchy::{prelude::*, register_state_tree};
use bevy_tweening::{
    lens::{TransformRotateZLens, TransformScaleLens, UiPositionLens},
    Animator, EaseMethod, Tween, TweeningPlugin,
};




const TRANSITION_DURATION: Duration = Duration::from_secs(2);
const RESET_DURATION: Duration = Duration::from_secs(1);
const DYNAMIC_BOX_WIDTH: f32 = 150.0;
const DYNAMIC_BOX_HEIGHT: f32 = 65.0;
const BOXES_PER_ROW: usize = 5;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_resource::<UIState>()
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system);

    //app.add_systems(Update, text_node_parents);

    register_state_tree::<Root>(&mut app);
    app.run();
}

#[derive(Debug, Clone, Resource, Default)]
pub struct UIState {
    pub next_index: usize,
    pub dynamic_buttons: Vec<usize>,
}

// fn text_node_parents(query: Query<(&Text, &Parent)>) {
//     for ((text, parent)) in query.iter() {
//         info!("{text:?} has parent {parent:?}")
//     }
// }

impl UIState {
    pub fn remove_or_readd(&mut self, next_number: usize) {
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
        self.next_index = 0;
    }

    pub fn add(&mut self) {
        self.dynamic_buttons.push(self.next_index);
        //info!("Added button {}", self.next_index);
        self.next_index += 1;
    }
}

pub fn get_button_left_top(state: &UIState, number: &usize) -> (Val, Val) {
    let index = state
        .dynamic_buttons
        .binary_search(&number)
        .map_or_else(|e| e, |f| f);

    let top = Val::Px((DYNAMIC_BOX_HEIGHT * (index / BOXES_PER_ROW) as f32) + 300.);
    let left = Val::Px(DYNAMIC_BOX_WIDTH * (index % BOXES_PER_ROW) as f32);

    (left, top)
}

pub fn get_journey_duration(start: (Val, Val), end: (Val, Val)) -> Duration {
    match (start, end) {
        ((Val::Px(s_x), Val::Px(s_y)), (Val::Px(e_x), Val::Px(e_y))) => {
            let distance = Vec2 { x: s_x, y: s_y }.distance(Vec2 { x: e_x, y: e_y });
            let ratio = distance / 300.0;
            TRANSITION_DURATION.mul_f32(ratio)
        }
        _ => TRANSITION_DURATION,
    }
}

#[derive(Debug, Eq, PartialEq, Component, Hash, Clone, Copy)]
pub enum Command {
    AddNew,
    Reset,
}

fn add_text_node(
    commands: &mut EntityCommands,
    text: impl Into<String>,
    asset_server: &AssetServer,
) {
    commands.with_children(|s| {
        s.spawn_empty().insert(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    });
}

#[derive(Debug, Eq, PartialEq, Component, Hash, Clone, Copy)]
pub struct DynamicButtonComponent(usize);

// #[derive(Eq, PartialEq, Debug, Hash)]
// pub enum SimpleNode {
//     Root,
//     CommandButton(Command),
//     DynamicButton { number: usize },
// }

// impl StateTreeRoot for SimpleNode {

// }

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Root;

impl StateTreeRoot for Root{
    type ContextParam<'a> = (Res<'a, UIState>, Res<'a, AssetServer>);

    fn get_context<'a, 'c> (param: bevy::ecs::system::StaticSystemParam<Self::ContextParam<'a>>) -> Self::Context <'c>{
        (param.0, param.1)
    }
}

impl StateTreeNode for Root{
    type Context<'c> = (Res<'c, UIState>, Res<'c, AssetServer>);

    fn get_components<'b>(
        &self,
        context: &Self::Context<'b>,
        component_commands: &mut impl ComponentCommands,
    ) {
        todo!()
    }

    fn get_children<'b>(&self, context: &Self::Context<'b>, child_commands: &mut impl ChildCommands) {
        todo!()
    }

    fn get_child_deletion_policy<'b>(&self, context: &Self::Context<'b>) -> state_hierarchy::ChildDeletionPolicy {
        todo!()
    }


}


// impl StateTreeNode for Root {
//     type Context = (UIState, AssetServer);

//     fn get_children(&self, args: &Self::Context) -> Self::Children {
//         let mut children = vec![
//             CommandButton(Command::AddNew),
//             CommandButton(Command::Reset),
//         ];

//         children.extend(
//             args.0
//                 .dynamic_buttons
//                 .iter()
//                 .map(|index| DynamicButton { number: *index }),
//         );
//         children.into_iter()
//     }

//     fn create(&self, commands: &mut EntityCommands, args: &Self::Context) {
//         todo!()
//     }

//     fn update(
//         &self,
//         commands: &mut EntityCommands,
//         args: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     ) {
//         todo!()
//     }

//     fn delete(
//         &self,
//         commands: &mut EntityCommands,
//         args: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     ) -> DeleteResult {
//         todo!()
//     }

//     fn cancel_delete(
//         &self,
//         commands: &mut EntityCommands,
//         args: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     ) {
//         todo!()
//     }

//     fn should_update(&self, args: &Self::Context, previous: &Self::Context) -> StateTreeShouldUpdate {
//         todo!()
//     }
// }

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct CommandButton(Command);

// impl StateTreeNode for CommandButton {
//     type Context = (AssetServer,);

//     type Children = std::iter::Empty<Self>;

//     fn get_children(&self, args: &Self::Context) -> Self::Children {
//         std::iter::empty()
//     }

//     fn create(&self, commands: &mut EntityCommands, args: &Self::Context) {
//         todo!()
//     }

//     fn update(
//         &self,
//         commands: &mut EntityCommands,
//         args: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     ) {
//     }

//     fn delete(
//         &self,
//         commands: &mut EntityCommands,
//         args: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     ) -> DeleteResult {
//         DeleteResult { linger_time: None }
//     }

//     fn cancel_delete(
//         &self,
//         commands: &mut EntityCommands,
//         args: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     ) {
//     }

//     fn should_update(&self, args: &Self::Context, previous: &Self::Context) -> StateTreeShouldUpdate {
//         StateTreeShouldUpdate::SELF_ONLY
//     }
// }

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct DynamicButton {
    number: usize,
}

impl StateTreeNode for DynamicButton {}

// impl StateTreeNode for SimpleNode {
//     type Args = (UIState, AssetServer);
//     type Children = std::vec::IntoIter<Self>;

//     fn get_children(&self, args: &Self::Args) -> Self::Children {
//         match self {
//             SimpleNode::Root => {
//                 let mut children = vec![
//                     SimpleNode::CommandButton(Command::AddNew),
//                     SimpleNode::CommandButton(Command::Reset),
//                 ];

//                 children.extend(
//                     args.0
//                         .dynamic_buttons
//                         .iter()
//                         .map(|index| SimpleNode::DynamicButton { number: *index }),
//                 );

//                 children
//             }

//             SimpleNode::DynamicButton { number: _ } => {
//                 vec![]
//             }
//             SimpleNode::CommandButton(_) => vec![],
//         }.into_iter()
//     }

//     fn create(&self, commands: &mut bevy::ecs::system::EntityCommands, args: &Self::Args) {
//         match self {
//             SimpleNode::Root => {
//                 commands.insert(NodeBundle {
//                     style: Style {
//                         width: Val::Percent(100.0),
//                         align_items: AlignItems::Center,
//                         justify_content: JustifyContent::Center,
//                         ..default()
//                     },
//                     ..default()
//                 });
//             }
//             SimpleNode::CommandButton(command) => {
//                 let left = match command {
//                     Command::AddNew => Val::Percent(30.),
//                     Command::Reset => Val::Percent(70.),
//                 };
//                 commands
//                     .insert(ButtonBundle {
//                         style: Style {
//                             width: Val::Px(DYNAMIC_BOX_WIDTH),
//                             height: Val::Px(DYNAMIC_BOX_HEIGHT),
//                             border: UiRect::all(Val::Px(5.0)),
//                             position_type: PositionType::Absolute,
//                             left,
//                             top: Val::Px(100.),
//                             // horizontally center child text
//                             justify_content: JustifyContent::Center,
//                             // vertically center child text
//                             align_items: AlignItems::Center,
//                             ..default()
//                         },
//                         border_color: BorderColor(Color::BLACK),
//                         background_color: NORMAL_BUTTON.into(),
//                         ..default()
//                     })
//                     .insert(*command);
//                 let text = match command {
//                     Command::AddNew => "Add",
//                     Command::Reset => "Reset",
//                 };
//                 add_text_node(commands, text, &args.1)
//             }
//             SimpleNode::DynamicButton { number } => {
//                 let (left, top) = get_button_left_top(&args.0, number);
//                 commands
//                     .insert(ButtonBundle {
//                         style: Style {
//                             width: Val::Px(DYNAMIC_BOX_WIDTH),
//                             height: Val::Px(DYNAMIC_BOX_HEIGHT),
//                             border: UiRect::all(Val::Px(5.0)),
//                             position_type: PositionType::Absolute,

//                             // horizontally center child text
//                             justify_content: JustifyContent::Center,
//                             // vertically center child text
//                             align_items: AlignItems::Center,
//                             left,
//                             top: Val::Px(0.0),
//                             ..default()
//                         },
//                         border_color: BorderColor(Color::BLACK),
//                         background_color: NORMAL_BUTTON.into(),
//                         ..default()
//                     })
//                     .insert(DynamicButtonComponent(*number));

//                 let start = UiRect {
//                     left: left,
//                     top: Val::Px(0.0),
//                     ..Default::default()
//                 };
//                 let end = UiRect {
//                     left,
//                     top,
//                     ..Default::default()
//                 };

//                 let duration = get_journey_duration((start.left, start.top), (end.left, end.top));

//                 commands.insert(Animator::new(Tween::new(
//                     EaseMethod::Linear,
//                     duration,
//                     UiPositionLens { start, end },
//                 )));

//                 add_text_node(commands, number.to_string(), &args.1)
//             }
//         }
//     }

//     fn should_update(
//         &self,
//         _args: &Self::Args,
//         _previous: &Self::Args,
//     ) -> bevy_state_tree::should_update::StateTreeShouldUpdate {
//         match self {
//             SimpleNode::Root => StateTreeShouldUpdate::SELF_AND_CHILDREN,
//             SimpleNode::CommandButton(_) => StateTreeShouldUpdate::SELF_ONLY,
//             SimpleNode::DynamicButton { .. } => StateTreeShouldUpdate::SELF_ONLY,
//         }
//     }

//     fn update(
//         &self,
//         commands: &mut bevy::ecs::system::EntityCommands,
//         args: &Self::Args,
//         _previous: &Self::Args,
//         entity_ref: EntityRef,
//     ) {
//         match self {
//             SimpleNode::Root => {}
//             SimpleNode::CommandButton(_) => {}
//             SimpleNode::DynamicButton { number } => {
//                 let style = entity_ref.get::<Style>().cloned().unwrap_or_default();

//                 let start = UiRect {
//                     left: style.left,
//                     top: style.top,
//                     ..Default::default()
//                 };
//                 let (left, top) = get_button_left_top(&args.0, number);
//                 let end = UiRect {
//                     left,
//                     top,
//                     ..Default::default()
//                 };

//                 let duration = get_journey_duration((start.left, start.top), (end.left, end.top));

//                 commands.insert(Animator::new(Tween::new(
//                     EaseMethod::Linear,
//                     duration,
//                     UiPositionLens { start, end },
//                 )));

//                 //commands.insert(style);
//             }
//         }
//     }

//     fn delete(
//         &self,
//         commands: &mut bevy::ecs::system::EntityCommands,
//         _args: &Self::Args,
//         _previous: &Self::Args,
//         _entity_ref: EntityRef,
//     ) -> DeleteResult {
//         if _args.0.next_index == 0 {
//             commands.insert(Animator::new(Tween::new(
//                 EaseMethod::Linear,
//                 TRANSITION_DURATION,
//                 TransformRotateZLens {
//                     start: 0.0,
//                     end: consts::TAU * 1.0,
//                 },
//             )));

//             DeleteResult {
//                 linger_time: Some(RESET_DURATION),
//             }
//         } else {
//             commands.insert(Animator::new(Tween::new(
//                 EaseMethod::Linear,
//                 TRANSITION_DURATION,
//                 TransformScaleLens {
//                     start: Vec3::ONE,
//                     end: Vec3::ZERO,
//                 },
//             )));

//             DeleteResult {
//                 linger_time: Some(TRANSITION_DURATION),
//             }
//         }
//     }

//     fn cancel_delete(
//         &self,
//         commands: &mut bevy::ecs::system::EntityCommands,
//         _args: &Self::Args,
//         _previous: &Self::Args,
//         entity_ref: EntityRef,
//     ) {
//         let scale = entity_ref
//             .get::<Transform>()
//             .map(|x| x.scale.x)
//             .unwrap_or_default();

//         let duration = TRANSITION_DURATION.mul_f32(1. - scale);

//         commands.insert(Animator::new(Tween::new(
//             EaseMethod::Linear,
//             duration,
//             TransformScaleLens {
//                 start: Vec3::ONE * scale,
//                 end: Vec3::ONE,
//             },
//         )));
//     }
// }

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
