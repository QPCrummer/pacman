use crate::core::game_state::MainMenu::Menu;
use crate::core::CorePlugin;
use crate::game::ui::settings_screen::Config;
use crate::game::GamePlugin;
use crate::map_creator::create_map;
use crate::spawn::SpawnPlugin;
use bevy::prelude::*;
use bevy_asset_preload::{load_assets, AssetPreloadPlugin};
use bevy_sprite_sheet::SpriteSheetPlugin;
use bevy_window_icon::WindowIconPlugin;
use core::prelude::*;
use std::env;

mod core;
pub mod game;
mod map_creator;
mod spawn;

fn main() {
    let mut app = App::new();
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("assets/icon.png");
    let icon_path = path.to_str().unwrap();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    title: "PacMan".to_string(),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    resizable: false,
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        WindowIconPlugin::new(icon_path),
    ))
    .insert_resource(Config::load().unwrap())
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
    .add_plugins((
        CorePlugin,
        GamePlugin,
        SpawnPlugin,
        AssetPreloadPlugin::load_given_paths(
            Setup(PreloadAssets),
            Setup(CreateSpriteSheets),
            load_assets!(),
        ),
        SpriteSheetPlugin::new(Setup(CreateSpriteSheets), MainMenu(Menu)),
    ));

    if should_create_map() {
        create_map(&mut app);
    } else {
        app.run();
    }
}

fn should_create_map() -> bool {
    std::env::args().any(|arg| arg.contains("create_map"))
}
