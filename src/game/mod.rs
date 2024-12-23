use crate::core::random::RandomPlugin;
use crate::game::animate_walls::AnimateWallsPlugin;
use crate::game::camera::CameraPlugin;
use crate::game::edibles::EdiblePlugin;
use crate::game::game_state_transition::GameStateTransitionPlugin;
use crate::game::ghost_house_gate::GhostHouseGatePlugin;
use crate::game::ghosts::GhostPlugin;
use crate::game::interactions::InteractionsPlugin;
use crate::game::level::LevelPlugin;
use crate::game::lives::LivesPlugin;
use crate::game::move_through_tunnel::MoveThroughTunnelPlugin;
use crate::game::music::MusicPlugin;
use crate::game::pacman::PacmanPlugin;
use crate::game::restart_game::RestartGamePlugin;
use crate::game::schedule::SchedulePlugin;
use crate::game::score::ScorePlugin;
use crate::game::sound_effect::SoundEffectPlugin;
use crate::game::specs_per_level::SpecsPerLevelPlugin;
use crate::game::speed::SpeedPlugin;
use crate::game::state::StatePlugin;
use crate::game::target::TargetPlugin;
use crate::game::ui::UIPlugin;
use bevy::prelude::*;

pub mod animate_walls;
pub mod camera;
pub mod edibles;
pub mod game_state_transition;
pub mod ghost_house_gate;
pub mod ghosts;
pub mod interactions;
pub mod level;
pub mod lives;
mod move_through_tunnel;
pub mod music;
pub mod pacman;
mod restart_game;
mod schedule;
pub mod score;
pub mod sound_effect;
pub mod specs_per_level;
pub mod speed;
pub mod state;
pub mod target;
pub mod ui;

/// Contains the entire gameplay logic for pacman.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EdiblePlugin,
            GhostPlugin,
            GhostHouseGatePlugin,
            InteractionsPlugin,
            LevelPlugin,
            LivesPlugin,
            AnimateWallsPlugin,
            PacmanPlugin,
            RandomPlugin,
            SchedulePlugin,
            ScorePlugin,
            SpecsPerLevelPlugin,
            SpeedPlugin,
            StatePlugin,
            TargetPlugin,
        ))
        .add_plugins((
            CameraPlugin,
            MoveThroughTunnelPlugin,
            GameStateTransitionPlugin,
            SoundEffectPlugin,
            MusicPlugin,
            RestartGamePlugin,
            UIPlugin,
        ));
    }
}
