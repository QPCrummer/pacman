use bevy::prelude::*;
use crate::game_assets::loaded_assets::LoadedAssets;

use crate::game::ghosts::movement::MovePlugin;
use crate::game::ghosts::spawn::spawn_ghosts;
use crate::game::target::Target;
use crate::game::ghosts::textures::{start_animation, update_ghost_appearance};
use crate::game::interactions::{EGhostEaten, LPacmanGhostHitDetection};
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::map::tunnel::GhostPassedTunnel;
use crate::game_state::in_game;

pub mod movement;
pub mod spawn;
mod textures;

pub(in crate::game) struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MovePlugin)
            .add_systems(OnEnter(Game(Ready)), spawn_ghosts)
            .add_systems(OnEnter(Game(Running)), start_animation)
            .add_systems(Update, (
                ghost_passed_tunnel,
                play_ghost_eaten_sound_when_ghost_was_eaten.after(LPacmanGhostHitDetection)
            ).run_if(in_state(Game(Running))))
            .add_systems(Update, update_ghost_appearance.run_if(in_game))
            .add_systems(OnEnter(Game(PacmanDying)), despawn_ghosts)
            .add_systems(OnEnter(Game(LevelTransition)), despawn_ghosts)
            .add_systems(OnEnter(Game(GhostEatenPause)), set_currently_eaten_ghost_invisible)
            .add_systems(OnExit(Game(GhostEatenPause)), (
                remove_currently_eaten_ghost,
                set_currently_eaten_ghost_visible
            ))
        ;
    }
}

fn ghost_passed_tunnel(
    mut event_reader: EventReader<GhostPassedTunnel>,
    mut query: Query<(Entity, &mut Target), With<Ghost>>,
) {
    for event in event_reader.iter() {
        for (entity, mut target) in query.iter_mut() {
            if entity == **event {
                target.clear();
            }
        }
    }
}

fn despawn_ghosts(
    mut commands: Commands,
    query: Query<Entity, With<Ghost>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn remove_currently_eaten_ghost(
    mut commands: Commands
) {
    commands.remove_resource::<CurrentlyEatenGhost>()
}

fn set_currently_eaten_ghost_invisible(
    currently_eaten_ghost: Res<CurrentlyEatenGhost>,
    mut query: Query<(Entity, &mut Visibility), With<Ghost>>,
) {
    for (entity, mut vis) in &mut query {
        if **currently_eaten_ghost == entity {
            *vis = Visibility::Hidden
        }
    }
}

fn set_currently_eaten_ghost_visible(
    currently_eaten_ghost: Res<CurrentlyEatenGhost>,
    mut query: Query<(Entity, &mut Visibility), With<Ghost>>,
) {
    for (entity, mut vis) in &mut query {
        if **currently_eaten_ghost == entity {
            *vis = Visibility::Visible
        }
    }
}

fn play_ghost_eaten_sound_when_ghost_was_eaten(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    mut event_reader: EventReader<EGhostEaten>,
) {
    if event_reader.iter().count() > 0 {
        commands.spawn(
            AudioBundle {
                source: loaded_assets.get_handle("sounds/ghost_eaten.ogg"),
                ..default()
            }
        );
    }
}

#[derive(Copy, Clone, Component, Eq, PartialEq, Hash)]
pub enum Ghost {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

/// Resource that holds the entity id of the ghost that is currently eaten by pacman
#[derive(Deref, Resource)]
pub struct CurrentlyEatenGhost(pub Entity);