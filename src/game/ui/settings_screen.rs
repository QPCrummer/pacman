use crate::core::constants::FONT;
use crate::core::game_state::GameState;
use crate::core::game_state::MainMenu::{Menu, Settings};
use crate::GameState::MainMenu;
use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::prelude::Val::Percent;
use bevy::prelude::{
    default, in_state, AssetServer, Commands, Component, Entity, IntoSystemConfigs, KeyCode,
    NextState, OnEnter, OnExit, PositionType, Query, Res, ResMut, Resource, TextBundle, TextStyle,
    Visibility, With, Without,
};
use bevy::text::Text;
use bevy::ui::Style;
use serde::{Deserialize, Serialize};
use std::fs;

pub(super) struct SettingsScreenPlugin;

#[derive(Component)]
struct SettingsScreen;

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct ConfigVal;

#[derive(Resource)]
struct Editing(bool);

impl Plugin for SettingsScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenu(Settings)), (setup, hide_screen))
            .add_systems(OnExit(MainMenu(Settings)), show_screen)
            .add_systems(Update, check_inputs.run_if(in_state(MainMenu(Settings))))
            .insert_resource(Editing(false));
    }
}

fn setup(mut commands: Commands, config: ResMut<Config>, asset_server: Res<AssetServer>) {
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "CONFIGURATION",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 20.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(40.0),
            top: Percent(5.0),
            ..default()
        }),
    ));

    // Background Music
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "BACKGROUND MUSIC                - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(10.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_bool(config.background_music),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(10.0),
            ..default()
        }),
    ));

    // Game Sounds
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "GAME SOUNDS                     - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(15.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_bool(config.game_sounds),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(15.0),
            ..default()
        }),
    ));

    // Starting Lives
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "STARTING LIVES                  - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(20.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_integer(config.starting_lives),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(20.0),
            ..default()
        }),
    ));

    // Starting Level
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "STARTING LEVEL                  - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(25.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_integer(config.starting_level),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(25.0),
            ..default()
        }),
    ));

    // Pacman Speed
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "PACMAN SPEED MODIFIER           - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(30.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_float(config.pacman_speed_modifier),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(30.0),
            ..default()
        }),
    ));

    // Ghost Speed
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "GHOST SPEED MODIFIER            - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(35.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_float(config.ghost_speed_modifier),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(35.0),
            ..default()
        }),
    ));

    // Ghost Tunnel Speed
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "GHOST TUNNEL SPEED MODIFIER     - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(40.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_float(config.ghost_tunnel_speed_modifier),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(40.0),
            ..default()
        }),
    ));

    // Frightened Ghost Speed
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "FRIGHTENED GHOST SPEED MODIFIER - ",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(25.0),
            top: Percent(45.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        ConfigVal,
        TextBundle::from_section(
            value_of_float(config.frightened_ghost_speed_modifier),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(55.0),
            top: Percent(45.0),
            ..default()
        }),
    ));

    // Controls
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "PRESS UP ARROW TO INCREMENT",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(40.0),
            top: Percent(70.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "PRESS DOWN ARROW TO DECREMENT",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(40.0),
            top: Percent(73.0),
            ..default()
        }),
    ));

    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "PRESS ENTER TO SELECT",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(40.0),
            top: Percent(76.0),
            ..default()
        }),
    ));

    // Escape message
    commands.spawn((
        SettingsScreen,
        TextBundle::from_section(
            "PRESS ESCAPE TO EXIT",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 15.0,
                color: Color::srgb(1.0, 1.0, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(40.0),
            top: Percent(80.0),
            ..default()
        }),
    ));

    // Cursor
    commands.spawn((
        SettingsScreen,
        Cursor,
        TextBundle::from_section(
            "->",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 10.0,
                color: Color::srgb(1.0, 1.0, 0.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(20.0),
            top: Percent(10.0),
            ..default()
        }),
    ));
}

const CONFIG_PATH: &str = "./config.json";

#[derive(Serialize, Deserialize, Resource)]
pub struct Config {
    pub starting_lives: u8,
    pub starting_level: u8,
    pub pacman_speed_modifier: f32,
    pub ghost_speed_modifier: f32,
    pub ghost_tunnel_speed_modifier: f32,
    pub frightened_ghost_speed_modifier: f32,
    pub background_music: bool,
    pub game_sounds: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            starting_lives: 3,
            starting_level: 1,
            pacman_speed_modifier: 1.0,
            ghost_speed_modifier: 1.0,
            ghost_tunnel_speed_modifier: 1.0,
            frightened_ghost_speed_modifier: 1.0,
            background_music: true,
            game_sounds: true,
        }
    }
}

impl Config {
    pub(crate) fn save(&self) -> std::io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        fs::write(CONFIG_PATH, serialized)
    }

    pub(crate) fn load() -> std::io::Result<Self> {
        if !fs::exists(CONFIG_PATH)? {
            let serialized = serde_json::to_string(&Config::default())?;
            fs::write(CONFIG_PATH, serialized)?;
        }
        let data = fs::read_to_string(CONFIG_PATH)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }
}

fn check_inputs(
    mut commands: Commands,
    query: Query<Entity, With<SettingsScreen>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    config: ResMut<Config>,
    mut enable_buttons: ResMut<crate::game::ui::main_menu_screen::ButtonCanPress>,
    query1: Query<(&Cursor, &mut Style)>,
    query2: Query<(&Cursor, &mut Text)>,
    value_query: Query<(&ConfigVal, &mut Text, &Style), Without<Cursor>>,
    mut editing: ResMut<Editing>,
) {
    // Back to main menu
    if enable_buttons.0 {
        if keys.pressed(KeyCode::Escape) {
            enable_buttons.0 = false;
            despawn_screen(&mut commands, query);
            config.save().expect("Failed to Save Config!");
            next_game_state.set(MainMenu(Menu));
        }

        if keys.pressed(KeyCode::Enter) {
            editing.0 = !editing.0;
            change_cursor_color(query2, editing.0);
            enable_buttons.0 = false;
        }

        // Editing Functions
        if editing.0 {
            if keys.pressed(KeyCode::ArrowUp) {
                enable_buttons.0 = false;
                // Increment value
                change_config_value(query1, value_query, config, true);
            } else if keys.pressed(KeyCode::ArrowDown) {
                enable_buttons.0 = false;
                // Decrement value
                change_config_value(query1, value_query, config, false);
            }
        } else if keys.pressed(KeyCode::ArrowUp) {
            enable_buttons.0 = false;
            negative_iterator_cursor(query1);
        } else if keys.pressed(KeyCode::ArrowDown) {
            enable_buttons.0 = false;
            positive_iterator_cursor(query1);
        }
    }
}

fn despawn_screen(commands: &mut Commands, query: Query<Entity, With<SettingsScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn hide_screen(
    mut query: Query<&mut Visibility, With<crate::game::ui::main_menu_screen::MainMenuScreen>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn show_screen(
    mut query: Query<&mut Visibility, With<crate::game::ui::main_menu_screen::MainMenuScreen>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn positive_iterator_cursor(mut query: Query<(&Cursor, &mut Style)>) {
    for (_cursor, mut style) in query.iter_mut() {
        if let Percent(current_top) = style.top {
            // Increment the y position by the interval
            let new_top = current_top + 5.0;

            // Loop back to the minimum position if it exceeds the maximum
            style.top = if new_top > 45.0 {
                Percent(10.0)
            } else {
                Percent(new_top)
            };
        }
    }
}

fn negative_iterator_cursor(mut query: Query<(&Cursor, &mut Style)>) {
    for (_cursor, mut style) in query.iter_mut() {
        if let Percent(current_top) = style.top {
            // Decrement the y position by the interval
            let new_top = current_top - 5.0;

            // Loop back to the max position if it goes below the min
            style.top = if new_top < 10.0 {
                Percent(45.0)
            } else {
                Percent(new_top)
            };
        }
    }
}

fn change_cursor_color(mut query: Query<(&Cursor, &mut Text)>, editing: bool) {
    for (_cursor, mut text) in query.iter_mut() {
        text.sections[0].style.color = if editing {
            Color::srgb(1.0, 0.0, 0.0)
        } else {
            Color::srgb(1.0, 1.0, 0.0)
        };
    }
}

fn change_config_value(
    query: Query<(&Cursor, &mut Style)>,
    mut value_query: Query<(&ConfigVal, &mut Text, &Style), Without<Cursor>>,
    mut config: ResMut<Config>,
    increment: bool,
) {
    for (_cursor, style) in query.iter() {
        if let Percent(current_top) = style.top {
            match current_top {
                10.0 => {
                    config.background_music = !config.background_music;
                    update_config_value_text(
                        &mut value_query,
                        10.0,
                        value_of_bool(config.background_music),
                    );
                }
                15.0 => {
                    config.game_sounds = !config.game_sounds;
                    update_config_value_text(
                        &mut value_query,
                        15.0,
                        value_of_bool(config.game_sounds),
                    );
                }
                20.0 => {
                    if increment {
                        config.starting_lives = (config.starting_lives % 10) + 1;
                    } else {
                        config.starting_lives = if config.starting_lives == 1 {
                            10
                        } else {
                            config.starting_lives - 1
                        };
                    }
                    update_config_value_text(
                        &mut value_query,
                        20.0,
                        value_of_integer(config.starting_lives),
                    );
                }
                25.0 => {
                    if increment {
                        config.starting_level = (config.starting_level % 255) + 1;
                    } else {
                        config.starting_level = if config.starting_level == 1 {
                            255
                        } else {
                            config.starting_level - 1
                        };
                    }
                    update_config_value_text(
                        &mut value_query,
                        25.0,
                        value_of_integer(config.starting_level),
                    );
                }
                30.0 => {
                    if increment {
                        if config.pacman_speed_modifier < 10.0 {
                            config.pacman_speed_modifier += 0.01;
                        }
                    } else if config.pacman_speed_modifier > 0.01 {
                        config.pacman_speed_modifier -= 0.01;
                    }
                    update_config_value_text(
                        &mut value_query,
                        30.0,
                        value_of_float(config.pacman_speed_modifier),
                    );
                }
                35.0 => {
                    if increment {
                        if config.ghost_speed_modifier < 10.0 {
                            config.ghost_speed_modifier += 0.01;
                        }
                    } else if config.ghost_speed_modifier > 0.01 {
                        config.ghost_speed_modifier -= 0.01;
                    }
                    update_config_value_text(
                        &mut value_query,
                        35.0,
                        value_of_float(config.ghost_speed_modifier),
                    );
                }
                40.0 => {
                    if increment {
                        if config.ghost_tunnel_speed_modifier < 10.0 {
                            config.ghost_tunnel_speed_modifier += 0.01;
                        }
                    } else if config.ghost_tunnel_speed_modifier > 0.01 {
                        config.ghost_tunnel_speed_modifier -= 0.01;
                    }
                    update_config_value_text(
                        &mut value_query,
                        40.0,
                        value_of_float(config.ghost_tunnel_speed_modifier),
                    );
                }
                _ => {
                    //45.0
                    if increment {
                        if config.frightened_ghost_speed_modifier < 10.0 {
                            config.frightened_ghost_speed_modifier += 0.01;
                        }
                    } else if config.frightened_ghost_speed_modifier > 0.01 {
                        config.frightened_ghost_speed_modifier -= 0.01;
                    }
                    update_config_value_text(
                        &mut value_query,
                        45.0,
                        value_of_float(config.frightened_ghost_speed_modifier),
                    );
                }
            }
        }
    }
}

fn value_of_bool(value: bool) -> String {
    value.to_string().to_uppercase()
}

fn value_of_integer(value: u8) -> String {
    value.to_string()
}

fn value_of_float(value: f32) -> String {
    format!("{:.2}", value)
}

fn update_config_value_text(
    value_query: &mut Query<(&ConfigVal, &mut Text, &Style), Without<Cursor>>,
    cursor_y_value: f32,
    new_config_val: String,
) {
    for (_, mut text, style) in value_query.iter_mut() {
        if let Percent(current_top) = style.top {
            if current_top == cursor_y_value {
                text.sections[0].value = new_config_val.clone();
                return;
            }
        }
    }
}
