use bevy::prelude::*;

use crate::constants::ENERGIZER_DIMENSION;
use crate::map::board::Board;
use crate::map::FieldType;
use crate::pacman::Pacman;
use crate::common::Position;

pub struct EnergizerPlugin;

impl Plugin for EnergizerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnergizerEaten>()
            .add_startup_system(spawn_energizer)
            .add_system(pacman_eat_energizer);
    }
}

/// An energizer that allows pacman to eat ghosts.
#[derive(Component)]
pub struct Energizer;

/// Fired when pacman eats an energizer.
pub struct EnergizerEaten;

fn spawn_energizer(mut commands: Commands, board: Res<Board>) {
    let point_dimension = Vec2::new(ENERGIZER_DIMENSION, ENERGIZER_DIMENSION);
    for position in board.positions_of_type(FieldType::Energizer) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.0, 0.9),
                    custom_size: Some(point_dimension),
                    ..default()
                },
                transform: Transform::from_translation(board.coordinates_of_position(position)),
                ..Default::default()
            })
            .insert(Energizer)
            .insert(position.clone());
    }
}

fn pacman_eat_energizer(
    mut commands: Commands,
    mut event_writer: EventWriter<EnergizerEaten>,
    pacman_positions: Query<&Position, With<Pacman>>,
    energizer_positions: Query<(Entity, &Position), With<Energizer>>,
) {
    for pacman_position in pacman_positions.iter() {
        for (energizer_entity, energizer_position) in energizer_positions.iter() {
            if energizer_position == pacman_position {
                commands.entity(energizer_entity).despawn();
                event_writer.send(EnergizerEaten)
            }
        }
    }
}