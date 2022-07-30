use bevy::prelude::*;
use crate::is;
use crate::life_cycle::LifeCycle::GameOver;
use crate::map::Map;
use crate::map::Element;

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameOver).with_system(spawn_screen)
            )
        ;
    }
}

#[derive(Component)]
struct GameOverScreen;

fn spawn_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>
) {
    let coordinates = map.coordinates_between_positions_matching(is!(Element::FruitSpawn));
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("GAME OVER".to_string(),
                                 TextStyle {
                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                     font_size: 40.0,
                                     color: Color::rgb(1.0, 0.0, 0.0),
                                 },
                                 TextAlignment {
                                     vertical: VerticalAlign::Center,
                                     horizontal: HorizontalAlign::Center,
                                 }),
        transform: Transform::from_translation(coordinates),
        ..Default::default()
    })
        .insert(GameOverScreen);
}