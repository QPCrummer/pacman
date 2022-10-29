use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use crate::common::position::Position;
use crate::constants::{GHOST_BASE_SPEED, PACMAN_BASE_SPEED};

use crate::edibles::energizer::EnergizerTimer;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::ghosts::state::State;
use crate::level::Level;
use crate::pacman::Pacman;
use crate::edibles::dots::EatenDots;
use crate::life_cycle::LifeCycle;
use crate::map::tunnel::{Tunnel, TunnelHallway};
use crate::specs_per_level::SpecsPerLevel;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(LifeCycle::Running)
                    .with_system(update_ghost_speed)
                    .with_system(update_pacman_speed)
            )
        ;
    }
}

/// The current speed of a moving entity
#[derive(Copy, Clone, Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(WorldQuery)]
#[world_query(mutable)]
struct GhostSpeedUpdateComponents<'a> {
    ghost: &'a Ghost,
    transform: &'a Transform,
    speed: &'a mut Speed,
    state: &'a State
}

fn update_ghost_speed(
    level: Res<Level>,
    eaten_dots: Res<EatenDots>,
    specs_per_level: Res<SpecsPerLevel>,
    mut ghost_query: Query<GhostSpeedUpdateComponents>,
    tunnel_query: Query<&Transform, Or<(With<Tunnel>, With<TunnelHallway>)>>
) {
    for mut comps in ghost_query.iter_mut() {
        match *comps.ghost {
            Blinky => update_blinky_speed(&level, &specs_per_level, &eaten_dots, &mut comps, &tunnel_query),
            _ => update_non_blinky_speed(&level, &specs_per_level, &mut comps, &tunnel_query)
        }
    }
}

/// Blinkys speed is set differently, as he has the elroy mode. He
/// gets two speed bonuses, depending on the remaining dots on the board.
/// The amount of dots to trigger elroy depends on the current level.
fn update_blinky_speed(
    level: &Level,
    specs_per_level: &SpecsPerLevel,
    eaten_dots: &EatenDots,
    comps: &mut GhostSpeedUpdateComponentsItem,
    tunnel_query: &Query<&Transform, Or<(With<Tunnel>, With<TunnelHallway>)>>
) {
    let spec = specs_per_level.get_for(&level);
    let remaining_dots = eaten_dots.get_remaining();

    if *comps.state == State::Eaten {
        *comps.speed = Speed(GHOST_BASE_SPEED * 2.0)
    } else if is_in_tunnel(&comps.transform, tunnel_query) {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.ghost_tunnel_speed_modifier);
    } else if *comps.state == State::Frightened {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.ghost_frightened_speed_modifier)
    } else if remaining_dots <= spec.elroy_2_dots_left {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.elroy_2_speed_modifier)
    } else if remaining_dots <= spec.elroy_1_dots_left {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.elroy_1_speed_modifier)
    } else {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.ghost_normal_speed_modifier)
    }
}

fn update_non_blinky_speed(
    level: &Level,
    specs_per_level: &SpecsPerLevel,
    comps: &mut GhostSpeedUpdateComponentsItem,
    tunnel_query: &Query<&Transform, Or<(With<Tunnel>, With<TunnelHallway>)>>
) {
    let spec = specs_per_level.get_for(&level);

    if *comps.state == State::Eaten {
        *comps.speed = Speed(GHOST_BASE_SPEED * 2.0)
    } else if is_in_tunnel(&comps.transform, tunnel_query) {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.ghost_tunnel_speed_modifier);
    } else if *comps.state == State::Frightened {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.ghost_frightened_speed_modifier)
    } else {
        *comps.speed = Speed(GHOST_BASE_SPEED * spec.ghost_normal_speed_modifier)
    }
}

fn is_in_tunnel(
    ghost_transform: &Transform,
    tunnel_query: &Query<&Transform, Or<(With<Tunnel>, With<TunnelHallway>)>>
) -> bool {
    tunnel_query
        .iter()
        .any(|transform| Position::from_vec(&transform.translation) == Position::from_vec(&ghost_transform.translation))
}

fn update_pacman_speed(
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    energizer_timer: Option<Res<EnergizerTimer>>,
    mut query: Query<&mut Speed, With<Pacman>>,
) {
    for mut speed in query.iter_mut() {
        let spec = specs_per_level.get_for(&level);

        if energizer_timer.is_some() {
            *speed = Speed(PACMAN_BASE_SPEED * spec.pacman_frightened_speed_modifier);
        } else {
            *speed = Speed(PACMAN_BASE_SPEED * spec.pacman_normal_speed_modifier);
        }
    }
}