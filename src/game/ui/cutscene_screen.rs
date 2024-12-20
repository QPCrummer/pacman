use crate::core::prelude::*;
use crate::game::music;
use crate::game::ui::main_menu_screen;
use bevy::prelude::*;
use vleue_kinetoscope::AnimatedImageBundle;

pub(super) struct CutsceneScreenPlugin;

impl Plugin for CutsceneScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(Cutscene)), (
                    spawn_screens,
                )
            )
            .add_systems(
                OnExit(Game(Cutscene)),
                despawn_screens
            )
        ;
    }
}

/// Shows the cutscene animation
#[derive(Component)]
struct CutsceneComponent;

fn spawn_screens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
) {
    let cutscene = get_cutscene(level);
    commands.spawn((
        Name::new(format!("CutsceneScreen{}", cutscene)),
        CutsceneComponent,
        AnimatedImageBundle {
            animated_image: asset_server.load(format!("cutscene/cutscene{}.gif", cutscene)),
            transform: Transform {
                translation: Vec3::new(main_menu_screen::get_relative_x(0.7), main_menu_screen::get_relative_y(0.15), 100.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    commands.spawn((
        Name::new("BlackLayer"),
        CutsceneComponent,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)), // Adjust size as needed
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 99.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    music::play_cutscene_sound(&mut commands, &asset_server, cutscene);
}

pub fn get_cutscene(level: Res<Level>) -> i8 {
    match level.0 {
        2 => 1,
        5 => 2,
        _ => 3,
    }
}

fn despawn_screens(
    mut commands: Commands,
    query: Query<Entity, With<CutsceneComponent>>,
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}