use bevy::prelude::*;
use pad::Position;

use crate::core::animation::AnimationPlugin;
use crate::core::direction::Dir;
use crate::core::edibles::EdiblesPlugin;
use crate::core::game_state::GameStatePlugin;
use crate::core::ghost_schedule::GhostSchedulePlugin;
use crate::core::ghost_state::GhostStatePlugin;
use crate::core::ghosts::GhostPlugin;
use crate::core::interactions::InteractionsPlugin;
use crate::core::level::LevelPlugin;
use crate::core::lives::LivesPlugin;
use crate::core::map::MapPlugin;
use crate::core::music::MusicPlugin;
use crate::core::pacman::PacmanPlugin;
use crate::core::position::Pos;
use crate::core::restart_game::RestartGamePlugin;
use crate::core::score::ScorePlugin;
use crate::core::sound_effect::SoundEffectPlugin;
use crate::core::specs_per_level::SpecsPerLevelPlugin;
use crate::core::speed::SpeedPlugin;
use crate::core::system_sets::SystemSetsPlugin;
use crate::core::target::TargetPlugin;

pub mod animation;
pub mod constants;
pub mod direction;
pub mod edibles;
pub mod game_state;
pub mod ghost_house_gate;
pub mod ghost_schedule;
pub mod ghost_state;
pub mod ghosts;
pub mod helper;
pub mod interactions;
pub mod level;
pub mod lives;
pub mod map;
pub mod music;
pub mod pacman;
pub mod position;
pub mod prelude;
pub mod random;
mod restart_game;
pub mod score;
pub mod sound_effect;
pub mod specs_per_level;
pub mod speed;
pub mod system_sets;
pub mod target;

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pos>()
            .register_type::<Position>()
            .register_type::<Dir>()
            .add_plugins((
                GameStatePlugin,
                AnimationPlugin,
                PacmanPlugin,
                GhostPlugin,
                EdiblesPlugin,
                MapPlugin,
                TargetPlugin,
                InteractionsPlugin,
                LevelPlugin,
                LivesPlugin,
                GhostSchedulePlugin,
                ScorePlugin,
                SpecsPerLevelPlugin,
                SpeedPlugin,
                GhostStatePlugin,
            ))
            .add_plugins((
                SoundEffectPlugin,
                MusicPlugin,
                RestartGamePlugin,
                SystemSetsPlugin,
            ));
    }
}
