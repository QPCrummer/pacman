use bevy::prelude::*;

use crate::music::MusicPlugin;
use crate::camera::CameraPlugin;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use crate::game_assets::GameAssetsPlugin;
use crate::game_state::GameStatePlugin;
use crate::system_sets::SystemSetsPlugin;

use crate::ui::UIPlugin;

mod camera;
mod constants;
mod game_assets;
mod music;
mod debug;
mod ui;
mod game;
mod game_state;
pub mod system_sets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    title: "PacMan".to_string(),
                    resizable: false,
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins((
            GamePlugin,
            GameStatePlugin,
            SystemSetsPlugin,
            GameAssetsPlugin,
            CameraPlugin,
            DebugPlugin,
            UIPlugin,
            MusicPlugin
        ))
        .run()
}
