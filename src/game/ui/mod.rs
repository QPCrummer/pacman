use bevy::prelude::*;
use crate::game::ui::bottom::BottomUIPlugin;
use crate::game::ui::game_over_screen::GameOverScreenPlugin;
use crate::game::ui::main_menu_screen::MainMenuScreenPlugin;
use crate::game::ui::ready_screen::ReadyScreenPlugin;
use crate::game::ui::top::TopUIPlugin;

mod top;
mod bottom;
mod game_over_screen;
mod ready_screen;
pub mod main_menu_screen;

pub(super) struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TopUIPlugin,
                BottomUIPlugin,
                ReadyScreenPlugin,
                MainMenuScreenPlugin,
                GameOverScreenPlugin
            ))
        ;
    }
}

