use bevy::prelude::*;
use LifeCycle::Ready;
use crate::is;
use crate::life_cycle::LifeCycle;
use crate::map::Map;
use crate::map::Element;

pub struct ReadyScreenPlugin;

impl Plugin for ReadyScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Ready).with_system(spawn_screen)
            )
            .add_system_set(
                SystemSet::on_exit(Ready).with_system(despawn_screen)
            )
        ;
    }
}

#[derive(Component)]
struct ReadyScreen;

fn spawn_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>
) {
    let coordinates = map.coordinates_between_positions_matching(is!(Element::FruitSpawn));
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Ready!".to_string(),
                                 TextStyle {
                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                     font_size: 40.0,
                                     color: Color::rgb(1.0, 1.0, 0.0),
                                 },
                                 TextAlignment {
                                     vertical: VerticalAlign::Center,
                                     horizontal: HorizontalAlign::Center,
                                 }),
        transform: Transform::from_translation(coordinates),
        ..Default::default()
    })
        .insert(ReadyScreen);
}

fn despawn_screen(
    mut commands: Commands,
    query: Query<Entity, With<ReadyScreen>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}