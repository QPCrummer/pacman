use std::time::Duration;
use crate::core::constants::FONT;
use crate::core::game_state::GameState;
use crate::core::prelude::{SpawnMapScene, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::GameState::{MainMenu, Spawn};
use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::core::Name;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::Val::Percent;
use bevy::prelude::{default, in_state, Camera, Camera2dBundle, ClearColorConfig, Commands, Component, Deref, DerefMut, Entity, IntoSystemConfigs, KeyCode, NextState, OnEnter, PositionType, Query, Res, ResMut, Resource, SpriteBundle, Style, TextBundle, TextStyle, Time, Timer, TimerMode, Transform, With};
use rand::random;
use vleue_kinetoscope::AnimatedImageBundle;
use crate::game::music;

pub(super) struct MainMenuScreenPlugin;

impl Plugin for MainMenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(MainMenu),
                setup
            )
            .add_systems(
                Update,
                (check_inputs, spawn_screen, get_next_theme).run_if(in_state(MainMenu))
            )
            .insert_resource(MainMenuTimer(Timer::from_seconds(8.0, TimerMode::Once)))
        ;
    }
}

#[derive(Component)]
struct MainMenuScreen;

#[derive(Resource)]
struct MainMenuScreenResources {
    enabled: bool,
    next_frame: i8,
}

#[derive(Resource, Deref, DerefMut)]
struct MainMenuTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct ThemeTimer(Timer);

fn setup(
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..Default::default()
        },
        ..Default::default()
    }, MainMenuScreen));

    commands.insert_resource(MainMenuScreenResources {
        enabled: true,
        next_frame: 8,
    });
}

fn check_inputs(
    commands: Commands,
    query: Query<Entity, With<MainMenuScreen>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<MainMenuScreenResources>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keys.pressed(KeyCode::Enter) {
        despawn_screen(commands, query);
        toggle.enabled = false;
        next_game_state.set(Spawn(SpawnMapScene));
    }
}

fn spawn_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut toggle: ResMut<MainMenuScreenResources>,
    mut timer: ResMut<MainMenuTimer>,
    time: Res<Time>,
) {
    if !toggle.enabled {
        return;
    }

    timer.tick(time.delta());
    let time_left = timer.remaining().as_secs();

    if time_left < 8 && toggle.next_frame == 8 {
        toggle.next_frame -= 1;
        commands.spawn((
            Name::new("CharNickText"),
            MainMenuScreen,
            TextBundle::from_section(
                "CHARACTER   /   NICKNAME",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(10.0),
                ..default()
            }),
        ));
    }

    if time_left < 7 && toggle.next_frame == 7 {
        toggle.next_frame -= 1;
        commands.spawn((
            Name::new("BlinkyText"),
            MainMenuScreen,
            TextBundle::from_section(
                "- SHADOW       \"BLINKY\"",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 20.0,
                    color: Color::srgb(1.0, 0.0, 0.0),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(15.0),
                ..default()
            }),
        ));

        commands.spawn((SpriteBundle {
            texture: asset_server.load("textures/ghost/blinky.png"),
            transform: Transform {
                translation: Vec3::new(get_relative_x(0.25), get_relative_y(0.15), 0.0),
                scale: Vec3::splat(2.0), // Doubling the size
                ..Default::default()
            },
            ..Default::default()
        }, MainMenuScreen,));
    }

    if time_left < 6 && toggle.next_frame == 6 {
        toggle.next_frame -= 1;
        commands.spawn((
            Name::new("PinkyText"),
            MainMenuScreen,
            TextBundle::from_section(
                "- SPEEDY       \"PINKY\"",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 20.0,
                    color: Color::srgb_u8(255,183,255),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(25.0),
                ..default()
            }),
        ));

        commands.spawn((SpriteBundle {
            texture: asset_server.load("textures/ghost/pinky.png"),
            transform: Transform {
                translation: Vec3::new(get_relative_x(0.25), get_relative_y(0.25), 0.0),
                scale: Vec3::splat(2.0), // Doubling the size
                ..Default::default()
            },
            ..Default::default()
        }, MainMenuScreen,));
    }

    if time_left < 5 && toggle.next_frame == 5 {
        toggle.next_frame -= 1;
        commands.spawn((
            Name::new("InkyText"),
            MainMenuScreen,
            TextBundle::from_section(
                "- BASHFUL      \"INKY\"",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 20.0,
                    color: Color::srgb_u8(0,255,255),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(35.0),
                ..default()
            }),
        ));

        commands.spawn((SpriteBundle {
            texture: asset_server.load("textures/ghost/inky.png"),
            transform: Transform {
                translation: Vec3::new(get_relative_x(0.25), get_relative_y(0.35), 0.0),
                scale: Vec3::splat(2.0), // Doubling the size
                ..Default::default()
            },
            ..Default::default()
        }, MainMenuScreen,));
    }

    if time_left < 4 && toggle.next_frame == 4 {
        toggle.next_frame -= 1;
        commands.spawn((
            Name::new("ClydeText"),
            MainMenuScreen,
            TextBundle::from_section(
                "- POKEY        \"CLYDE\"",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 20.0,
                    color: Color::srgb_u8(255,183,81),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(45.0),
                ..default()
            }),
        ));

        commands.spawn((SpriteBundle {
            texture: asset_server.load("textures/ghost/clyde.png"),
            transform: Transform {
                translation: Vec3::new(get_relative_x(0.25), get_relative_y(0.45), 0.0),
                scale: Vec3::splat(2.0), // Doubling the size
                ..Default::default()
            },
            ..Default::default()
        }, MainMenuScreen,));
    }

    if time_left < 3 && toggle.next_frame == 3 {
        toggle.next_frame -= 1;
        commands.spawn((
            Name::new("PelletText"),
            MainMenuScreen,
            TextBundle::from_section(
                "10 PTS",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 15.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(45.0),
                top: Percent(75.0),
                ..default()
            }),
        ));

        commands.spawn((SpriteBundle {
            texture: asset_server.load("textures/dot.png"),
            transform: Transform {
                translation: Vec3::new(get_relative_x(0.40), get_relative_y(0.75), 0.0),
                scale: Vec3::splat(2.0), // Doubling the size
                ..Default::default()
            },
            ..Default::default()
        }, MainMenuScreen,));

        commands.spawn((
            Name::new("PowerPelletText"),
            MainMenuScreen,
            TextBundle::from_section(
                "50 PTS",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 15.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(45.0),
                top: Percent(80.0),
                ..default()
            }),
        ));

        commands.spawn((SpriteBundle {
            texture: asset_server.load("textures/energizer.png"),
            transform: Transform {
                translation: Vec3::new(get_relative_x(0.40), get_relative_y(0.80), 1.0),
                scale: Vec3::splat(2.0), // Doubling the size
                ..Default::default()
            },
            ..Default::default()
        }, MainMenuScreen,));
    }

    if time_left < 2 && toggle.next_frame == 2 {
        toggle.next_frame -= 1;
    }

    if time_left < 1 && toggle.next_frame == 1 {
        toggle.next_frame -= 1;
        commands.spawn((
            Name::new("StartText"),
            MainMenuScreen,
            TextBundle::from_section(
                "PRESS ENTER",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(40.0),
                top: Percent(90.0),
                ..default()
            }),
        ));

        commands.spawn((AnimatedImageBundle {
            animated_image: asset_server.load("cutscene/main_menu_animation.gif"),
            transform: Transform {
                translation: Vec3::new(get_relative_x(0.45), get_relative_y(0.60), 0.0),
                scale: Vec3::splat(0.3), // Scale the sprite to its original size
                ..Default::default()
            },
            ..Default::default()
        }, MainMenuScreen,));

        // Start music
        let theme = (random::<u8>() % 3 + 1) as i8;
        let duration = get_theme_duration(theme);
        music::play_theme_sound(&mut commands, &asset_server, theme, duration);
        commands.insert_resource(ThemeTimer(Timer::from_seconds(duration as f32, TimerMode::Once)));
    }
}

fn get_theme_duration(theme: i8) -> u64 {
    match theme {
        1 => 130,
        2 => 151,
        _ => 251,
    }
}

fn get_next_theme(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme_timer: Option<ResMut<ThemeTimer>>,
    time: Res<Time>,
) {
    if let Some(mut timer) = theme_timer {
        timer.tick(time.delta());
        if timer.0.finished() {
            let theme = (random::<u8>() % 3 + 1) as i8;
            let duration = get_theme_duration(theme);
            music::play_theme_sound(&mut commands, &asset_server, theme, duration);
            timer.0.set_duration(Duration::from_secs(duration));
            timer.0.reset();
        }
    }
}

const HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
pub fn get_relative_y(percent: f32) -> f32 {
    HALF_HEIGHT - (WINDOW_HEIGHT * percent) - 8.0
}

const HALF_WIDTH: f32 = WINDOW_WIDTH / 2.0;
pub fn get_relative_x(percent: f32) -> f32 {
    -HALF_WIDTH + (WINDOW_WIDTH * percent) + 8.0
}

fn despawn_screen(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}