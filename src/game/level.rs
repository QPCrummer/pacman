use bevy::prelude::*;
use crate::core::game_state::MainMenu::Menu;
use crate::core::prelude::*;
use crate::game::ui::settings_screen::Config;

pub(in crate::game) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Level(1))
            .add_systems(
                OnExit(Game(LevelTransition)), (
                HighScoreSerializable::save,
                increase_level
                )
            )
            .add_systems(
                OnExit(Game(GameOver)),
                reset_level
            )
            .add_systems(
                OnEnter(MainMenu(Menu)),
                reset_level
            )
        ;
    }
}

fn increase_level(
    mut level: ResMut<Level>,
) {
    level.increase();
}

fn reset_level(
    mut level: ResMut<Level>,
    config: Res<Config>,
) {
    // Set from config
    level.0 = config.starting_level as usize;
}