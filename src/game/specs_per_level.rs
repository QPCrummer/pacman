use bevy::prelude::*;
use crate::prelude::*;

pub (in crate::game) struct SpecsPerLevelPlugin;

impl Plugin for SpecsPerLevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpecsPerLevel::default())
        ;
    }
}