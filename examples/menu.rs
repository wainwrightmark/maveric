use bevy::prelude::*;
use lazy_static::lazy_static;
use state_hierarchy::transition::prelude::*;
use state_hierarchy::{prelude::*, register_state_tree, impl_hierarchy_root};
use std::time::Duration;
use std::{string::ToString, sync::Arc};
use strum::{Display, EnumIs};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_resource::<MenuState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system);

    app.add_plugins(TransitionPlugin::<StyleLeftLens>::default());
    //app.add_plugins(TransitionPlugin::<StyleTopLens>::default());
    app.add_plugins(TransitionPlugin::<TransformScaleLens>::default());

    register_state_tree::<Root>(&mut app);
    app.run();
}
fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<MenuState>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            ButtonAction::OpenMenu => *state = MenuState::ShowMainMenu,
            ButtonAction::ChooseLevel => *state = MenuState::ShowLevelsPage(0),
            ButtonAction::NextLevelsPage => {
                match state.as_ref() {
                    MenuState::ShowLevelsPage(x) => *state = MenuState::ShowLevelsPage(x + 1),
                    _ => {}
                };
            }
            ButtonAction::PreviousLevelsPage => {
                match state.as_ref() {
                    MenuState::ShowLevelsPage(x) => {
                        *state = MenuState::ShowLevelsPage(x.saturating_sub(1))
                    }
                    _ => {}
                };
            }
            ButtonAction::None => {}
            _ => *state = MenuState::Closed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource, EnumIs)]
pub enum MenuState {
    #[default]
    Closed,
    ShowMainMenu,
    ShowLevelsPage(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Root;

impl_hierarchy_root!(Root);

#[derive(Component, Debug, Clone, Copy, Deref)]
pub struct RootPage(MenuState);

impl HierarchyNode for Root {
    type Context = NC2<MenuState, AssetServer>;

    fn update<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl HierarchyCommands) {
        commands.insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BACKGROUND_COLOR.into(),
            ..Default::default()
        });

        commands.insert(RootPage(context.0.clone()));

        match context.0.as_ref() {
            MenuState::Closed => {
                commands.child(
                    "open_icon",
                    &context.1,
                    icon_button_node(ButtonAction::OpenMenu),
                );
            }
            MenuState::ShowMainMenu => {
                commands.child("main_menu", context, MainMenu);
            }
            MenuState::ShowLevelsPage(n) => {
                let carousel = Carousel::new(*n as u32, |x| Some(LevelMenu(x)));
                commands.child("levels", context, carousel);
                // let duration: Duration = Duration::from_secs_f32(2.0);

                // let initial_left = match commands.get::<RootPage>(){
                //     Some(RootPage(MenuState::ShowLevelsPage(prev_page))) =>
                //     match prev_page.cmp(n){
                //         std::cmp::Ordering::Less => Val::Percent(00.0),
                //         std::cmp::Ordering::Equal => Val::Percent(50.0),
                //         std::cmp::Ordering::Greater => Val::Percent(100.0),
                //     },
                //     _ => Val::Percent(0.0),
                // };

                // let child_node = LevelMenu(*n)
                //     .with_transition_in_out::<StyleLeftLens>(
                //         Style {
                //             position_type: PositionType::Absolute,
                //             left: initial_left,
                //             right: Val::Percent(50.0), // Val::Px(MENU_OFFSET),
                //             top: Val::Px(MENU_OFFSET),
                //             display: Display::Flex,
                //             flex_direction: FlexDirection::Column,

                //             ..Default::default()
                //         },
                //         Val::Percent(50.0),
                //         Val::Percent(100.0),
                //         duration,
                //         duration,
                //     )//.with_transition_in_out(initial, destination, out_destination, in_duration, out_duration)

                //     // .with_transition_in_out::<TransformScaleLens>(
                //     //     Transform::from_scale(Vec3::new(1.0, 0.0, 1.0)),
                //     //     Vec3::ONE,
                //     //     Vec3::new(1.0, 0.0, 1.0),
                //     //     duration,
                //     //     duration,
                //     // )
                //     ;

                // commands.child(*n as u32, context, child_node);
            }
        }
    }

    
}


#[derive(Debug, PartialEq, Clone)]
pub struct CarouselDeletionPathMaker{
    pub page: u8,
    pub duration: Duration
}

// impl DeletionPathMaker for CarouselDeletionPathMaker {
//     fn get_path(
//         &self,
//         previous: &L::Value,
//         sibling_keys: &bevy::utils::HashSet<ChildKey>,
//     ) -> Option<TransitionPath<L>> {
//         todo!()
//     }
// }



fn icon_button_node(button_action: ButtonAction) -> ButtonNode<ButtonAction, String> {
    ButtonNode {
        value: button_action.icon(),
        text_node_style: ICON_BUTTON_TEXT_STYLE.clone(),
        button_node_style: ICON_BUTTON_STYLE.clone(),
        marker: button_action,
    }
}

fn text_button_node(button_action: ButtonAction) -> ButtonNode<ButtonAction, String> {
    ButtonNode {
        value: button_action.text(),
        text_node_style: TEXT_BUTTON_TEXT_STYLE.clone(),
        button_node_style: TEXT_BUTTON_STYLE.clone(),
        marker: button_action,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MainMenu;

impl HierarchyNode for MainMenu {
    type Context = NC2<MenuState, AssetServer>;

    fn update<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl HierarchyCommands) {
        commands.insert(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),  // Val::Px(MENU_OFFSET),
                right: Val::Percent(50.0), // Val::Px(MENU_OFFSET),
                top: Val::Px(MENU_OFFSET),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,

                ..Default::default()
            },
            z_index: ZIndex::Global(10),
            ..Default::default()
        });

        for (key, action) in ButtonAction::main_buttons().into_iter().enumerate() {
            commands.child(key as u32, &context.1, text_button_node(*action))
        }
    }

    
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LevelMenu(u32);

impl HierarchyNode for LevelMenu {
    type Context = NC2<MenuState, AssetServer>;

    fn update<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl HierarchyCommands) {
        commands.insert(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),  // Val::Px(MENU_OFFSET),
                right: Val::Percent(50.0), // Val::Px(MENU_OFFSET),
                top: Val::Px(MENU_OFFSET),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,

                ..Default::default()
            },
            z_index: ZIndex::Global(10),
            ..Default::default()
        });

        let start = self.0 * LEVELS_PER_PAGE;
        let end = start + LEVELS_PER_PAGE;

        for (key, level) in (start..end).enumerate() {
            commands.child(
                key as u32,
                &context.1,
                text_button_node(ButtonAction::GotoLevel { level }),
            )
        }

        commands.child("buttons", context, LevelMenuArrows);
    }


}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LevelMenuArrows;

impl HierarchyNode for LevelMenuArrows {
    type Context = NC2<MenuState, AssetServer>;

    fn update<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl HierarchyCommands) {
        commands.insert(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                left: Val::Percent(0.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,

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
            background_color: BackgroundColor(TEXT_BUTTON_BACKGROUND),
            border_color: BorderColor(BUTTON_BORDER),
            ..Default::default()
        });

        if let MenuState::ShowLevelsPage(page) = context.0.as_ref() {
            if *page == 0 {
                commands.child("left", &context.1, icon_button_node(ButtonAction::OpenMenu))
            } else {
                commands.child(
                    "left",
                    &context.1,
                    icon_button_node(ButtonAction::PreviousLevelsPage),
                )
            }

            if *page < 4 {
                commands.child(
                    "right",
                    &context.1,
                    icon_button_node(ButtonAction::NextLevelsPage),
                )
            } else {
                commands.child("right", &context.1, icon_button_node(ButtonAction::None))
            }
        }
    }

    
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

const LEVELS_PER_PAGE: u32 = 8;

pub const BACKGROUND_COLOR: Color = Color::hsla(216., 0.7, 0.72, 1.0); // #86AEEA
pub const ACCENT_COLOR: Color = Color::hsla(218., 0.69, 0.62, 1.0); // #5B8BE2
pub const WARN_COLOR: Color = Color::hsla(0., 0.81, 0.51, 1.0); // #FF6E5F
pub const TIMER_COLOR: Color = Color::BLACK;

pub const FIXED_SHAPE_FILL: Color = Color::WHITE;
pub const VOID_SHAPE_FILL: Color = Color::BLACK;

pub const FIXED_SHAPE_STROKE: Color = Color::BLACK;
pub const VOID_SHAPE_STROKE: Color = WARN_COLOR;
pub const ICE_SHAPE_STROKE: Color = Color::WHITE;

pub const SHADOW_STROKE: Color = Color::BLACK;

pub const LEVEL_TEXT_COLOR: Color = Color::DARK_GRAY;
pub const LEVEL_TEXT_ALT_COLOR: Color = Color::WHITE;

pub const BUTTON_BORDER: Color = Color::BLACK;
pub const BUTTON_TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const ICON_BUTTON_BACKGROUND: Color = Color::NONE;
pub const TEXT_BUTTON_BACKGROUND: Color = Color::WHITE;
pub const DISABLED_BUTTON_BACKGROUND: Color = Color::GRAY;

lazy_static! {
    static ref ICON_BUTTON_STYLE: Arc<ButtonNodeStyle> = Arc::new(ButtonNodeStyle {
        style: Style {
            width: Val::Px(ICON_BUTTON_WIDTH),
            height: Val::Px(ICON_BUTTON_HEIGHT),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0,

            ..Default::default()
        },
        background_color: Color::NONE,
        ..default()
    });
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
        background_color: TEXT_BUTTON_BACKGROUND.into(),
        border_color: BUTTON_BORDER.into(),
        ..Default::default()
    });
}

lazy_static! {
    static ref TEXT_BUTTON_TEXT_STYLE: Arc<TextNodeStyle> = Arc::new(TextNodeStyle {
        font_size: BUTTON_FONT_SIZE,
        color: BUTTON_TEXT_COLOR,
        font: FONT_PATH,
    });
}

lazy_static! {
    static ref ICON_BUTTON_TEXT_STYLE: Arc<TextNodeStyle> = Arc::new(TextNodeStyle {
        font_size: ICON_FONT_SIZE,
        color: BUTTON_TEXT_COLOR,
        font: FONT_PATH,
    });
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Component)]
pub enum ButtonAction {
    OpenMenu,
    Resume,
    ResetLevel,
    GoFullscreen,
    Tutorial,
    Infinite,
    DailyChallenge,
    Share,
    ChooseLevel,
    ClipboardImport,
    GotoLevel { level: u32 },
    NextLevel,
    MinimizeSplash,
    RestoreSplash,
    MinimizeApp,

    NextLevelsPage,
    PreviousLevelsPage,
    Credits,

    GooglePlay,
    Apple,
    Steam,

    None,
}

impl ButtonAction {
    pub fn main_buttons() -> &'static [Self] {
        use ButtonAction::*;
        &[
            Resume,
            ChooseLevel,
            DailyChallenge,
            Infinite,
            Tutorial,
            Share,
            ClipboardImport, //TODO
            #[cfg(all(feature = "web", target_arch = "wasm32"))]
            GoFullscreen,
            #[cfg(all(feature = "android", target_arch = "wasm32"))]
            MinimizeApp,
            Credits,
        ]
    }

    pub fn icon(&self) -> String {
        use ButtonAction::*;
        match self {
            OpenMenu => "\u{f0c9}".to_string(),       // "Menu",
            Resume => "\u{e817}".to_string(),         // "Menu",
            ResetLevel => "\u{e800}".to_string(),     //"Reset Level",image
            GoFullscreen => "\u{f0b2}".to_string(),   //"Fullscreen",
            Tutorial => "\u{e801}".to_string(),       //"Tutorial",
            Infinite => "\u{e802}".to_string(),       //"Infinite",
            DailyChallenge => "\u{e803}".to_string(), // "Challenge",
            Share => "\u{f1e0}".to_string(),          // "Share",
            ChooseLevel => "\u{e812}".to_string(),    // "\u{e812};".to_string(),
            GotoLevel { level } => level.to_string(),
            NextLevel => "\u{e808}".to_string(), //play

            MinimizeApp => "\u{e813}".to_string(),     //logout
            ClipboardImport => "\u{e818}".to_string(), //clipboard
            PreviousLevelsPage => "\u{e81b}".to_string(),
            NextLevelsPage => "\u{e81a}".to_string(),
            Credits => "\u{e811}".to_string(),
            RestoreSplash => "\u{f149}".to_string(),
            MinimizeSplash => "\u{f148}".to_string(),

            GooglePlay => "\u{f1a0}".to_string(),
            Apple => "\u{f179}".to_string(),
            Steam => "\u{f1b6}".to_string(),
            None => "".to_string(),
        }
    }

    pub fn text(&self) -> String {
        use ButtonAction::*;
        match self {
            OpenMenu => "Menu".to_string(),
            Resume => "Resume".to_string(),
            ResetLevel => "Reset".to_string(),
            GoFullscreen => "Fullscreen".to_string(),
            Tutorial => "Tutorial".to_string(),
            Infinite => "Infinite Mode".to_string(),
            DailyChallenge => "Daily Challenge".to_string(),
            Share => "Share".to_string(),
            ChooseLevel => "Choose Level".to_string(),
            ClipboardImport => "Import Level".to_string(),
            GotoLevel { level } => {
                format!("Level {level}")
            }
            NextLevel => "Next Level".to_string(),
            MinimizeSplash => "Minimize Splash".to_string(),
            RestoreSplash => "Restore Splash".to_string(),
            MinimizeApp => "Quit".to_string(),
            NextLevelsPage => "Next Levels".to_string(),
            PreviousLevelsPage => "Previous Levels".to_string(),
            Credits => "Credits".to_string(),

            GooglePlay => "Google Play".to_string(),
            Apple => "Apple".to_string(),
            Steam => "Steam".to_string(),
            None => "".to_string(),
        }
    }
}
