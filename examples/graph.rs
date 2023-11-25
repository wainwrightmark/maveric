use bevy::prelude::*;
use maveric::{prelude::*, widgets::text2d_node::Text2DNode};
use strum::{Display, EnumIs, EnumIter, IntoEnumIterator, IntoStaticStr};

use std::time::Duration;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .init_resource::<GraphState>()
        .register_transition::<BackgroundColorLens>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .add_systems(Update, organize_graph)
        .register_maveric::<Root>();
    app.run();
}
fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Display, EnumIs, EnumIter, IntoStaticStr)]
pub enum ButtonMarker {
    Increase,
    Decrease,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Default, MavericRoot)]
pub struct Root;

impl MavericRootChildren for Root {
    type Context = GraphState;

    fn set_children<'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        commands.add_child("Buttons", Buttons, &());

        for i in get_factors(context.number) {
            commands.add_child(
                i,
                NumberNode(i).with_transition_in_out::<BackgroundColorLens>(
                    Color::WHITE.with_a(0.5),
                    Color::WHITE.with_a(1.0),
                    Color::WHITE.with_a(0.0),
                    Duration::from_secs_f32(0.5),
                    Duration::from_secs_f32(2.0),
                ),
                &(),
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Buttons;

impl MavericNode for Buttons {
    type Context = ();

    fn set_components(commands: SetComponentCommands<Self, Self::Context>) {
        commands
            .ignore_context()
            .ignore_node()
            .insert(NodeBundle::default());
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        commands
            .ignore_node()
            .unordered_children_with_context(|context, commands| {
                for button_marker in ButtonMarker::iter() {
                    let text: &'static str = button_marker.into();
                    commands.add_child(
                        text,
                        ButtonNode {
                            style: ButtonStyle,
                            background_color: TEXT_BUTTON_BACKGROUND,
                            border_color: BUTTON_BORDER,
                            visibility: Visibility::Visible,
                            marker: button_marker,
                            children: (TextNode {
                                text,
                                font_size: BUTTON_FONT_SIZE,
                                color: BUTTON_TEXT_COLOR,
                                font: FONT_PATH,
                                alignment: TextAlignment::Center,
                                linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                            },),
                        },
                        &context,
                    )
                }
            });
    }
}

fn get_factors(num: u32) -> Vec<u32> {
    let mut vec = vec![];

    let root = (num as f32).sqrt().floor() as u32;

    for i in 1..=root {
        if num % i == 0 {
            vec.push(i);
            let other = num / i;
            if other != i {
                vec.push(other);
            }
        }
    }

    vec.sort();
    //info!("{num}: {vec:?}");
    vec
}

#[derive(Debug, PartialEq, Clone)]
struct NumberNode(u32);

#[derive(Debug, PartialEq, Clone, Component)]
struct GraphNode(u32);

impl MavericNode for NumberNode {
    type Context = ();

    fn set_components(commands: SetComponentCommands<Self, Self::Context>) {
        commands
            .ignore_context()
            .insert_with_node(|a| GraphNode(a.0))
            .advanced(|a, c| {
                if a.event != SetEvent::Created {
                    return;
                }

                let x = (a.node.0 % 2) as f32 - 0.5;
                let y = (a.node.0 % 3) as f32 - 1.0;
                // we vary the starting position slightly so let the graph have something to work from

                let translation = Vec3::new(x, y, 0.0);

                c.insert((
                    TransformBundle::from_transform(Transform::from_translation(translation)),
                    VisibilityBundle::default(),
                ))
            });
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        commands.unordered_children_with_node_and_context(|args, context, commands| {
            commands.add_child(
                0,
                Text2DNode {
                    text: format!(" {} ", args.0),
                    font_size: TEXT_FONT_SIZE,
                    color: BUTTON_TEXT_COLOR,
                    font: FONT_PATH,
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },
                context,
            );
        });
    }
}

#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct GraphState {
    number: u32,
}

impl MavericContext for GraphState{}

fn organize_graph(time: Res<Time>, mut nodes: Query<(&mut Transform, &GraphNode)>) {
    const ATTRACTION: f32 = 0.01;
    const REPULSION: f32 = 200.0;
    const MAX_REPULSION: f32 = 500.0;

    let mut combinations = nodes.iter_combinations_mut();
    while let Some([mut left, mut right]) = combinations.fetch_next() {
        let difference = left.0.translation.truncate() - right.0.translation.truncate();
        let attraction =
            ATTRACTION * attraction(left.1 .0, right.1 .0) * (difference).length_squared();
        let repulsion = REPULSION * difference.length_recip().min(MAX_REPULSION);

        let force = (repulsion - attraction)
            * difference.try_normalize().unwrap_or(if left.1 .0 % 2 == 0 {
                Vec2::X
            } else {
                Vec2::Y
            })
            * time.delta_seconds();

        update_position(left.0.as_mut(), force);
        update_position(right.0.as_mut(), -force);
    }
}

fn update_position(transform: &mut Transform, force: Vec2) {
    transform.translation += force.extend(0.0);
}

fn attraction(left: u32, right: u32) -> f32 {
    let (l, r) = if left < right {
        (left, right)
    } else {
        (right, left)
    };

    if r % l == 0 {
        if r == l * l {
            2.0
        } else {
            1.0
        }
    } else {
        0.0
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &ButtonMarker),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<GraphState>,
) {
    for (interaction, marker) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                state.number = match marker {
                    ButtonMarker::Increase => state.number.saturating_add(1),
                    ButtonMarker::Decrease => state.number.saturating_sub(1),
                    ButtonMarker::Reset => 0,
                };
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
pub const TEXT_FONT_SIZE: f32 = 32.0;
pub const BUTTON_BORDER: Color = Color::BLACK;
pub const BUTTON_TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const TEXT_BUTTON_BACKGROUND: Color = Color::WHITE;

pub const NODE_SIZE: f32 = 60.;
pub const NODE_BORDER: f32 = 1.;
