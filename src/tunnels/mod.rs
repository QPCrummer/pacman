use bevy::prelude::*;
use crate::life_cylce::LifeCycle::*;

use crate::tunnels::movement::{move_ghost_trough_tunnel, move_pacman_through_tunnel};
use crate::tunnels::spawn::spawn_tunnels;

mod movement;
pub mod spawn;

pub struct TunnelPlugin;

impl Plugin for TunnelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GhostPassedTunnel>()
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_tunnels)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(move_pacman_through_tunnel)
                    .with_system(move_ghost_trough_tunnel)
            )
        ;
    }
}

#[derive(Component, Deref)]
struct Tunnel(usize);

/// Event. Fired when a ghost moved through a tunnel.
#[derive(Deref, DerefMut)]
pub struct GhostPassedTunnel(pub Entity);