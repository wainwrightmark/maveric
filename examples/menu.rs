use bevy::prelude::*;
use lazy_static::lazy_static;
use state_hierarchy::prelude::TransitionPlugin;
use state_hierarchy::{prelude::*, register_state_tree, widgets::prelude::*};
use std::f32::consts;
use std::{string::ToString, sync::Arc};
use strum::{Display, EnumIs};
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
    // let mut app = App::new();

    // app.add_plugins(DefaultPlugins)
    //     .init_resource::<MenuState>()
    //     .add_systems(Startup, setup)
    //     .add_systems(Update, button_system);

    // app.add_plugins(TransitionPlugin::<TransformVelocity>::default());

    // register_state_tree::<Root>(&mut app);
    // app.run();
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource, EnumIs)]
pub enum MenuState{
    #[default]
    Closed,
    ShowMainMenu,
    ShowLevelsPage(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Root;

// impl HierarchyRoot for Root{
//     type ContextParam<'c> = ;

//     fn get_context<'a, 'c, 'w: 'c, 's>(
//         param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
//     ) -> Self::Context<'c> {
//         todo!()
//     }
// }
