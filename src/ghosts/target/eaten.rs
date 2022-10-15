use bevy::prelude::*;
use crate::ghost_house::GhostHouse;
use crate::ghosts::target::{get_nearest_neighbour, TargetComponents, TargetComponentsItem};
use crate::common::Direction::*;
use crate::ghosts::state::State;
use crate::{state_skip_if, target_skip_if};
use crate::board_dimensions::BoardDimensions;
use crate::map::board::Board;
use crate::common::XYEqual;

/// Determine the next target coordinates for a ghost when in "Eaten" state.
///
/// When eaten, a ghost walks to the ghost house and enters it. When at the ghost house, he aligns perfectly
/// before the entrance, moves than to the house center and finally to his spawn coordinates, which depend on the ghost type.
pub fn set_eaten_target(
    board: Res<Board>,
    ghost_house: Res<GhostHouse>,
    dimensions: Res<BoardDimensions>,
    mut query: Query<TargetComponents>,
) {
    for mut components in query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Eaten);

        if is_directly_before_entrance(&components, &ghost_house) {
            move_in_house_center(&mut components, &ghost_house)
        } else if is_before_entrance(&components, &ghost_house, &dimensions) {
            move_directly_before_entrance(&mut components, &ghost_house, &dimensions)
        } else if is_in_center(&components, &ghost_house) {
            move_to_respawn(&mut components, &ghost_house)
        } else {
            // TODO: Maybe only take this branch when not already in the ghost house, just to avoid bugs
            move_to_nearest_position_before_entrance(&mut components, &ghost_house, &board, &dimensions)
        }
    }
}

/// Return if the ghost is perfectly centered in front of the ghost house entrance.
fn is_directly_before_entrance(components: &TargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    components.transform.translation.xy_equal_to(&ghost_house.coordinates_in_front_of_entrance())
}

fn move_in_house_center(components: &mut TargetComponentsItem, ghost_house: &GhostHouse) {
    *components.direction = ghost_house.entrance_direction.opposite();
    components.target.set(ghost_house.center_coordinates());
}

/// Return if the ghost is just on a position in front of the house.
fn is_before_entrance(components: &TargetComponentsItem, ghost_house: &GhostHouse, dimensions: &BoardDimensions) -> bool {
    ghost_house.positions_in_front_of_entrance().into_iter().any(|pos| pos == &dimensions.trans_to_pos(&components.transform))
}

fn move_directly_before_entrance(components: &mut TargetComponentsItem, ghost_house: &GhostHouse, dimension: &BoardDimensions) {
    let in_front_of_house = ghost_house.coordinates_in_front_of_entrance();
    let position_coordinates = dimension.pos_center(&components.transform.translation);

    *components.direction = match ghost_house.entrance_direction {
        Up | Down => match in_front_of_house.x < position_coordinates.x {
            true => Left,
            false => Right
        },
        Left | Right => match in_front_of_house.y < position_coordinates.y {
            true => Down,
            false => Up
        }
    };
    components.target.set(in_front_of_house);
}

fn is_in_center(components: &TargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    components.transform.translation.xy_equal_to(&ghost_house.center_coordinates())
}

fn move_to_respawn(components: &mut TargetComponentsItem, ghost_house: &GhostHouse) {
    let center = ghost_house.center_coordinates();
    let respawn = ghost_house.respawn_coordinates_of(components.ghost);

    *components.direction = match ghost_house.entrance_direction {
        Up | Down => match respawn.x < center.x {
            true => Left,
            false => Right
        },
        Left | Right => match respawn.y < center.y {
            true => Down,
            false => Up
        }
    };
    components.target.set(respawn);
}

fn move_to_nearest_position_before_entrance(components: &mut TargetComponentsItem, ghost_house: &GhostHouse, board: &Board, dimensions: &BoardDimensions) {
    let nearest_spawn_position = dimensions.trans_to_pos(components.transform).get_nearest_position_from(ghost_house.positions_in_front_of_entrance());
    let next_target_neighbour = get_nearest_neighbour(
        components,
        nearest_spawn_position,
        dimensions,
        |n| !board.position_is_wall(&n.position)
    );

    *components.direction = next_target_neighbour.direction;
    components.target.set(dimensions.pos_to_vec(&next_target_neighbour.position, 0.0));
}