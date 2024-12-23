use crate::core::prelude::*;
use crate::core::system_sets::UpdateGameState;
use crate::game::ui::cutscene_screen::get_cutscene;
use bevy::prelude::*;

pub(super) struct GameStateTransitionPlugin;

impl Plugin for GameStateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            (update_state.in_set(UpdateGameState), update_state_timer),
        );
    }
}

/// Tells when to switch to the next state in the state machine
#[derive(Resource, Deref, DerefMut)]
pub(crate) struct StateTimer(Timer);

/// Update the current game state based on multiple factors.
///
/// The idea: There are currently two big groups of states: Loading and InGame. Loading does
/// just load assets, but InGame is the actual running game. Every state inside the game is represented
/// as a state on top of InGame in the stack. This enables systems to use InGame for stuff that should always be
/// run when playing the actual game, like updating ui.
fn update_state(
    mut commands: Commands,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    lives: Res<Lives>,
    state_timer: Option<Res<StateTimer>>,
    pacman_hit_events: EventReader<PacmanWasHit>,
    edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    ghost_eaten_events: EventReader<GhostWasEaten>,
    game_restartet_events: EventReader<GameWasRestarted>,
    level: Res<Level>,
) {
    match current_state.get() {
        Game(Start) => switch_when_timer_finished(
            &mut commands,
            &state_timer,
            &mut next_state,
            2.0,
            Game(Ready),
        ),
        Game(Ready) => switch_when_timer_finished(
            &mut commands,
            &state_timer,
            &mut next_state,
            2.5,
            Game(Running),
        ),
        Game(Running) => switch_states_based_on_events(
            &mut next_state,
            pacman_hit_events,
            edibles_eaten_events,
            ghost_eaten_events,
        ),
        Game(PacmanHit) => switch_when_timer_finished(
            &mut commands,
            &state_timer,
            &mut next_state,
            1.0,
            Game(PacmanDying),
        ),
        Game(PacmanDying) => switch_when_timer_finished(
            &mut commands,
            &state_timer,
            &mut next_state,
            1.5,
            Game(PacmanDead),
        ),
        Game(PacmanDead) => {
            switch_to_ready_or_game_over(&mut commands, &state_timer, &lives, &mut next_state)
        }
        Game(GameOver) => switch_to_start_after_game_over(&mut next_state, game_restartet_events),
        Game(LevelTransition) => {
            if should_play_cutscene(level.0 as i32) {
                switch_to_cutscene(&mut next_state);
            } else {
                switch_when_timer_finished(
                    &mut commands,
                    &state_timer,
                    &mut next_state,
                    3.0,
                    Game(Ready),
                );
            }
        }
        Game(Cutscene) => {
            let cutscene = get_cutscene(level);
            switch_when_timer_finished(
                &mut commands,
                &state_timer,
                &mut next_state,
                get_cutscene_length(cutscene),
                Game(Ready),
            );
        }
        Game(GhostEatenPause) => switch_when_timer_finished(
            &mut commands,
            &state_timer,
            &mut next_state,
            1.0,
            Game(Running),
        ),
        _ => {}
    }
}

fn should_play_cutscene(level: i32) -> bool {
    if level < 2 {
        return false; // The sequence starts at 2
    }
    let discriminant = 9 + 8 * level;
    let sqrt_discriminant = (discriminant as f64).sqrt() as i32;

    if sqrt_discriminant * sqrt_discriminant != discriminant {
        return false; // Discriminant must be a perfect square
    }

    let n = (-3 + sqrt_discriminant) / 2;
    n > 0 && (n * n + 3 * n) / 2 == level
}

fn get_cutscene_length(cutscene: i8) -> f32 {
    match cutscene {
        1 => 10.53,
        2 => 8.3,
        _ => 8.56, // Cutscene 3
    }
}

fn switch_when_timer_finished(
    commands: &mut Commands,
    state_timer: &Option<Res<StateTimer>>,
    game_state: &mut NextState<GameState>,
    time: f32,
    new_state: GameState,
) {
    match state_timer {
        Some(timer) => {
            if timer.finished() {
                commands.remove_resource::<StateTimer>();
                game_state.set(new_state);
            }
        }
        None => commands.insert_resource(StateTimer(Timer::from_seconds(time, TimerMode::Once))),
    }
}

fn switch_to_cutscene(game_state: &mut NextState<GameState>) {
    game_state.set(Game(Cutscene));
}

fn switch_to_ready_or_game_over(
    commands: &mut Commands,
    state_timer: &Option<Res<StateTimer>>,
    lives: &Lives,
    game_state: &mut NextState<GameState>,
) {
    match state_timer {
        Some(timer) => {
            if timer.finished() {
                commands.remove_resource::<StateTimer>();

                if **lives > 0 {
                    game_state.set(Game(Ready))
                } else {
                    game_state.set(Game(GameOver))
                }
            }
        }
        None => commands.insert_resource(StateTimer(Timer::from_seconds(1.0, TimerMode::Once))),
    }
}

fn switch_states_based_on_events(
    game_state: &mut NextState<GameState>,
    mut pacman_hit_events: EventReader<PacmanWasHit>,
    mut edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    mut ghost_eaten_events: EventReader<GhostWasEaten>,
) {
    if pacman_hit_events.read().count() > 0 {
        game_state.set(Game(PacmanHit));
        return;
    }

    if edibles_eaten_events.read().count() > 0 {
        game_state.set(Game(LevelTransition));
        return;
    }

    if ghost_eaten_events.read().count() > 0 {
        game_state.set(Game(GhostEatenPause));
        return;
    }
}

fn switch_to_start_after_game_over(
    game_state: &mut NextState<GameState>,
    mut game_restarted_events: EventReader<GameWasRestarted>,
) {
    if game_restarted_events.read().count() > 0 {
        game_state.set(Game(Start))
    }
}

fn update_state_timer(time: Res<Time>, state_timer: Option<ResMut<StateTimer>>) {
    if let Some(mut timer) = state_timer {
        timer.tick(time.delta());
    }
}
