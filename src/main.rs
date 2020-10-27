use bevy::prelude::*;

use pacman::PacmanPlugin;

use crate::board::BoardPlugin;

mod pacman;
mod board;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(init.system())
        .add_plugin(BoardPlugin)
        .add_plugin(PacmanPlugin)
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
