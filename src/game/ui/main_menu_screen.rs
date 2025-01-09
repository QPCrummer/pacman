use crate::core::constants::FONT;
use crate::core::game_state::GameState;
use crate::core::game_state::MainMenu::{Menu, Settings};
use crate::core::prelude::{CreateSpriteSheets, SpawnMapScene, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::music;
use crate::game::ui::settings_screen::Config;
use crate::GameState::{MainMenu, Spawn};
use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::color::Color;
use bevy::core::Name;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::Val::Percent;
use bevy::prelude::{
    default, in_state, Camera, Camera2dBundle, ClearColorConfig, Commands, Component, Condition,
    Deref, DerefMut, Entity, Image, ImageBundle, IntoSystemConfigs, KeyCode, NextState, NonSendMut,
    OnExit, PositionType, Query, Res, ResMut, Resource, SpriteBundle, Style, TextBundle, TextStyle,
    Time, Timer, TimerMode, Transform, With,
};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{TextureDimension, TextureFormat, TextureUsages};
use bevy::ui::ZIndex;
use bevy::utils::HashMap;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::frame::Video;
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{Context, Flags};
use rand::random;
use std::env;
use std::path::Path;
use std::time::Duration;

pub(super) struct MainMenuScreenPlugin;

impl Plugin for MainMenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Setup(CreateSpriteSheets)), setup)
            .add_systems(
                Update,
                (check_inputs, spawn_screen).run_if(in_state(MainMenu(Menu))),
            )
            .add_systems(
                Update,
                (get_next_theme, tick_button_pressed_timer)
                    .run_if(in_state(MainMenu(Menu)).or_else(in_state(MainMenu(Settings)))),
            )
            .add_systems(Update, render_frame.run_if(in_state(MainMenu(Menu))))
            .init_non_send_resource::<VideoResource>()
            .insert_resource(FrameTimer(Timer::new(
                Duration::from_secs_f32(1. / 30.),
                TimerMode::Repeating,
            )))
            .insert_resource(MainMenuTimer(Timer::from_seconds(8.0, TimerMode::Once)))
            .insert_resource(ButtonTimer(Timer::from_seconds(0.5, TimerMode::Once)))
            .insert_resource(ButtonCanPress(true));
    }
}

#[derive(Component)]
pub(crate) struct MainMenuScreen;

#[derive(Resource)]
struct MainMenuScreenResources {
    enabled: bool,
    next_frame: i8,
}

#[derive(Resource, Deref, DerefMut)]
struct MainMenuTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct ThemeTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct ButtonTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct ButtonCanPress(pub(crate) bool);

fn setup(mut commands: Commands) {
    initialize_ffmpeg();
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        },
        MainMenuScreen,
    ));

    commands.insert_resource(MainMenuScreenResources {
        enabled: true,
        next_frame: 8,
    });
}

fn initialize_ffmpeg() {
    ffmpeg::init().unwrap();
}

fn check_inputs(
    commands: Commands,
    query: Query<Entity, With<MainMenuScreen>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<MainMenuScreenResources>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut enable_buttons: ResMut<ButtonCanPress>,
) {
    if enable_buttons.0 {
        // Start game
        if keys.pressed(KeyCode::Enter) {
            despawn_screen(commands, query);
            toggle.enabled = false;
            next_game_state.set(Spawn(SpawnMapScene));
        }

        // Open settings
        if keys.pressed(KeyCode::Escape) {
            enable_buttons.0 = false;
            next_game_state.set(MainMenu(Settings));
        }
    }
}

fn spawn_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut toggle: ResMut<MainMenuScreenResources>,
    mut timer: ResMut<MainMenuTimer>,
    time: Res<Time>,
    config: Res<Config>,
    images: ResMut<Assets<Image>>,
    mut video_resource: NonSendMut<VideoResource>,
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
            )
            .with_style(Style {
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
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(15.0),
                ..default()
            }),
        ));

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/ghost/blinky.png"),
                transform: Transform {
                    translation: Vec3::new(get_relative_x(0.25), get_relative_y(0.15), 0.0),
                    scale: Vec3::splat(2.0), // Doubling the size
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenuScreen,
        ));
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
                    color: Color::srgb_u8(255, 183, 255),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(25.0),
                ..default()
            }),
        ));

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/ghost/pinky.png"),
                transform: Transform {
                    translation: Vec3::new(get_relative_x(0.25), get_relative_y(0.25), 0.0),
                    scale: Vec3::splat(2.0), // Doubling the size
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenuScreen,
        ));
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
                    color: Color::srgb_u8(0, 255, 255),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(35.0),
                ..default()
            }),
        ));

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/ghost/inky.png"),
                transform: Transform {
                    translation: Vec3::new(get_relative_x(0.25), get_relative_y(0.35), 0.0),
                    scale: Vec3::splat(2.0), // Doubling the size
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenuScreen,
        ));
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
                    color: Color::srgb_u8(255, 183, 81),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(45.0),
                ..default()
            }),
        ));

        commands.spawn((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Percent(25.0),
                    top: Percent(45.0),
                    ..default()
                },
                z_index: ZIndex::Global(1),
                transform: Transform {
                    scale: Vec3::splat(2.0), // Doubling the size
                    ..Default::default()
                },
                image: asset_server.load("textures/ghost/clyde.png").into(),
                ..default()
            },
            MainMenuScreen,
        ));
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
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(45.0),
                top: Percent(75.0),
                ..default()
            }),
        ));

        commands.spawn((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Percent(40.0),
                    top: Percent(75.0),
                    ..default()
                },
                z_index: ZIndex::Global(1),
                transform: Transform {
                    scale: Vec3::splat(2.0), // Doubling the size
                    ..Default::default()
                },
                image: asset_server.load("textures/dot.png").into(),
                ..default()
            },
            MainMenuScreen,
        ));

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
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(45.0),
                top: Percent(80.0),
                ..default()
            }),
        ));

        commands.spawn((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Percent(40.0),
                    top: Percent(80.0),
                    ..default()
                },
                z_index: ZIndex::Global(1),
                transform: Transform {
                    scale: Vec3::splat(2.0), // Doubling the size
                    ..Default::default()
                },
                image: asset_server.load("textures/energizer.png").into(),
                ..default()
            },
            MainMenuScreen,
        ));
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
                "PRESS ENTER TO PLAY",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(30.0),
                top: Percent(90.0),
                ..default()
            }),
        ));

        commands.spawn((
            Name::new("ConfigText"),
            MainMenuScreen,
            TextBundle::from_section(
                "PRESS ESCAPE TO CONFIGURE",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 10.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Percent(35.0),
                top: Percent(95.0),
                ..default()
            }),
        ));

        let (video_player, video_player_non_send) = VideoPlayer::new(
            env::current_dir().unwrap().join("cutscenes/main_menu.mp4"),
            images,
        )
        .unwrap();

        let entity = commands
            .spawn((
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Percent(-58.0),
                        top: Percent(-15.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(-1),
                    transform: Transform {
                        scale: Vec3::splat(0.3), // Scale the sprite to its original size
                        ..Default::default()
                    },
                    image: video_player.image_handle.clone().into(),
                    ..default()
                },
                MainMenuScreen,
            ))
            .insert(video_player)
            .id();
        video_resource
            .video_players
            .insert(entity, video_player_non_send);

        // Start music
        let theme = (random::<u8>() % 3 + 1) as i8;
        let duration = get_theme_duration(theme);
        music::play_theme_sound(&mut commands, &asset_server, config, theme, duration);
        commands.insert_resource(ThemeTimer(Timer::from_seconds(
            duration as f32,
            TimerMode::Once,
        )));
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
    config: Res<Config>,
) {
    if let Some(mut timer) = theme_timer {
        timer.tick(time.delta());
        if timer.0.finished() {
            let theme = (random::<u8>() % 3 + 1) as i8;
            let duration = get_theme_duration(theme);
            music::play_theme_sound(&mut commands, &asset_server, config, theme, duration);
            timer.0.set_duration(Duration::from_secs(duration));
            timer.0.reset();
        }
    }
}

fn tick_button_pressed_timer(
    timer_pre: Option<ResMut<ButtonTimer>>,
    time: Res<Time>,
    enable_buttons: Option<ResMut<ButtonCanPress>>,
) {
    if let Some(mut timer) = timer_pre {
        if let Some(mut buttons) = enable_buttons {
            if !buttons.0 {
                timer.0.tick(time.delta());
                if timer.0.finished() {
                    buttons.0 = true;
                    timer.0.reset();
                }
            }
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

fn despawn_screen(mut commands: Commands, query: Query<Entity, With<MainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// This code belongs to https://gist.github.com/rgon/18d99f414a6524ff8db21fe07eca2db9
/// I did some slight modifications such as looping
#[derive(Default)]
struct VideoResource {
    video_players: HashMap<Entity, VideoPlayerNonSendData>,
}

struct VideoPlayerNonSendData {
    decoder: ffmpeg::decoder::Video,
    input_context: ffmpeg::format::context::Input,
    scaler_context: Context,
    file_path: String,
}

#[derive(Component)]
struct VideoPlayer {
    image_handle: Handle<Image>,
    video_stream_index: usize,
}

impl VideoPlayer {
    fn new<P>(
        path: P,
        mut images: ResMut<Assets<Image>>,
    ) -> Result<(VideoPlayer, VideoPlayerNonSendData), ffmpeg::Error>
    where
        P: AsRef<Path>,
    {
        let input_context = input(&path)?;

        // initialize decoder
        let input_stream = input_context
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input_stream.index();

        let context_decoder =
            ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
        let decoder = context_decoder.decoder().video()?;

        // initialize scaler
        let scaler_context = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGBA,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        // create image texture
        let mut image = Image::new_fill(
            bevy::render::render_resource::Extent3d {
                width: decoder.width(),
                height: decoder.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[0; 4],
            // Color::BLACK.
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;

        let image_handle = images.add(image);

        Ok((
            VideoPlayer {
                image_handle,
                video_stream_index,
            },
            VideoPlayerNonSendData {
                decoder,
                input_context,
                scaler_context,
                file_path: path.as_ref().to_string_lossy().into_owned(),
            },
        ))
    }
}

#[derive(Resource)]
struct FrameTimer(Timer);

fn render_frame(
    time: Res<Time>,
    mut frame_timer: ResMut<FrameTimer>,
    mut video_player_query: Query<(&mut VideoPlayer, Entity)>,
    mut video_resource: NonSendMut<VideoResource>,
    mut images: ResMut<Assets<Image>>,
) {
    // Update the timer with the time delta
    frame_timer.0.tick(time.delta());

    if frame_timer.0.finished() {
        for (video_player, entity) in video_player_query.iter_mut() {
            let video_player_non_send = video_resource.video_players.get_mut(&entity).unwrap();
            // read packets from stream until complete frame received
            while let Some((stream, packet)) = video_player_non_send.input_context.packets().next()
            {
                // check if packets is for the selected video stream
                if stream.index() == video_player.video_stream_index {
                    // pass packet to decoder
                    video_player_non_send.decoder.send_packet(&packet).unwrap();
                    let mut decoded = Video::empty();
                    // check if complete frame was received
                    if let Ok(()) = video_player_non_send.decoder.receive_frame(&mut decoded) {
                        let mut rgb_frame = Video::empty();
                        // run frame through scaler for color space conversion
                        video_player_non_send
                            .scaler_context
                            .run(&decoded, &mut rgb_frame)
                            .unwrap();
                        // update data of image texture
                        let image = images.get_mut(&video_player.image_handle).unwrap();
                        image.data.copy_from_slice(rgb_frame.data(0));
                        return;
                    }
                }
            }
            // no frame received
            // signal end of playback to decoder
            match video_player_non_send.decoder.send_eof() {
                Err(ffmpeg::Error::Eof) => {
                    // Reset the input context and decoder to loop the video
                    let new_input_context = input(&video_player_non_send.file_path).unwrap();
                    let input_stream = new_input_context
                        .streams()
                        .best(Type::Video)
                        .ok_or(ffmpeg::Error::StreamNotFound)
                        .unwrap();
                    let context_decoder =
                        ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())
                            .unwrap();
                    let new_decoder = context_decoder.decoder().video().unwrap();

                    // Update the video player non-send data
                    video_player_non_send.input_context = new_input_context;
                    video_player_non_send.decoder = new_decoder;
                }
                other => other.unwrap(),
            }
        }
    }
}
