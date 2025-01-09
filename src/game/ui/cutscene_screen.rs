use crate::core::prelude::*;
use crate::game::music;
use crate::game::ui::settings_screen::Config;
use bevy::prelude::Val::Percent;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{TextureDimension, TextureFormat, TextureUsages};
use bevy::utils::HashMap;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::frame::Video;
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{Context, Flags};
use std::env;
use std::path::Path;
use std::time::Duration;

pub(super) struct CutsceneScreenPlugin;

impl Plugin for CutsceneScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Game(Cutscene)), (spawn_screens,))
            .add_systems(Update, render_frame.run_if(in_state(Game(Cutscene))))
            .add_systems(OnExit(Game(Cutscene)), despawn_screens)
            .init_non_send_resource::<VideoResource>()
            .insert_resource(FrameTimer(Timer::new(
                Duration::from_secs_f32(1. / 30.),
                TimerMode::Repeating,
            )));
    }
}

/// Shows the cutscene animation
#[derive(Component)]
struct CutsceneComponent;

fn spawn_screens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    config: Res<Config>,
    images: ResMut<Assets<Image>>,
    mut video_resource: NonSendMut<VideoResource>,
) {
    let cutscene = get_cutscene(level);

    let (video_player, video_player_non_send) = VideoPlayer::new(
        env::current_dir()
            .unwrap()
            .join(format!("cutscenes/cutscene{}.mp4", cutscene)),
        images,
    )
    .unwrap();

    let entity = commands
        .spawn((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Percent(-55.0),
                    top: Percent(-35.0),
                    ..default()
                },
                z_index: ZIndex::Global(301),
                transform: Transform {
                    scale: Vec3::splat(0.5),
                    ..Default::default()
                },
                image: video_player.image_handle.clone().into(),
                ..default()
            },
            CutsceneComponent,
        ))
        .insert(video_player)
        .id();
    video_resource
        .video_players
        .insert(entity, video_player_non_send);

    commands.spawn((
        Name::new("BlackLayer"),
        CutsceneComponent,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(WINDOW_WIDTH * 2., WINDOW_HEIGHT * 2.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(WINDOW_WIDTH - 50.0, WINDOW_HEIGHT - 50.0, 199.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    music::play_cutscene_sound(&mut commands, &asset_server, cutscene, config);
}

pub fn get_cutscene(level: Res<Level>) -> i8 {
    match level.0 {
        3 => 1, // Level 2
        6 => 2, // Level 5
        _ => 3, // Level 9+
    }
}

fn despawn_screens(mut commands: Commands, query: Query<Entity, With<CutsceneComponent>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

/// This code belongs to https://gist.github.com/rgon/18d99f414a6524ff8db21fe07eca2db9
#[derive(Default)]
struct VideoResource {
    video_players: HashMap<Entity, VideoPlayerNonSendData>,
}

struct VideoPlayerNonSendData {
    decoder: ffmpeg::decoder::Video,
    input_context: ffmpeg::format::context::Input,
    scaler_context: Context,
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
    mut next_game_state: ResMut<NextState<GameState>>,
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
                Err(ffmpeg::Error::Eof) => {}
                other => other.unwrap(),
            }
            next_game_state.set(Game(Ready));
        }
    }
}
