use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub(super) struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HighScoreWasBeaten>()
            .register_type::<Score>()
            .register_type::<HighScore>()
            .register_type::<HighScoreWasBeaten>()
            .register_type::<ScoreText>()
            .register_type::<ScoreTextTimer>()
            .register_type::<EatenGhostCounter>();
    }
}

/// Resource that saves how many points the player has collected so far
#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct Score(pub usize);

impl Score {
    pub fn add(&mut self, points: usize) {
        **self += points
    }
}

/// Stores the current high score of the game.
#[derive(Resource, Reflect)]
pub struct HighScore {
    /// The actual high score
    pub score: usize,
    /// Tells if the high score was beaten in the current game. This is necessary to tell if the player
    /// has just beaten the score or if the player broke it and continues to collect points.
    pub was_beaten: bool,
}

const HIGH_SCORE_PATH: &str = "./high_score.json";

#[derive(Serialize, Deserialize)]
pub struct HighScoreSerializable {
    score: usize,
}

impl HighScoreSerializable {
    pub fn save(high_score: Res<HighScore>) {
        let high_score_serializable = HighScoreSerializable {
            score: high_score.score,
        };

        // Serialize the high score to a JSON string
        let json = serde_json::to_string(&high_score_serializable)
            .expect("Failed to serialize high score");

        // Create or open the file at the specified path
        let path = Path::new(HIGH_SCORE_PATH);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // TODO Only overwrite the current user's data
            .open(path)
            .expect("Failed to create or open the high score file");

        // Write the JSON string to the file
        file.write_all(json.as_bytes())
            .expect("Failed to write high score to file");
    }

    pub fn load() -> HighScore {
        // Open the file at the specified path
        let path = Path::new(HIGH_SCORE_PATH);

        let file = if !path.exists() {
            File::create(path).unwrap()
        } else {
            File::open(path).unwrap()
        };

        // Deserialize the JSON data from the file into a HighScoreSerializable instance
        let high_score: HighScoreSerializable =
            serde_json::from_reader(file).unwrap_or(HighScoreSerializable { score: 0 });

        HighScore {
            score: high_score.score,
            was_beaten: false,
        }
    }
}

impl HighScore {
    pub fn new() -> Self {
        HighScoreSerializable::load()
    }
}

/// Fired when the player broke the current high score
#[derive(Event, Reflect)]
pub struct HighScoreWasBeaten;

/// Identifies floating text which pops up when pacman ate a ghost.
#[derive(Component, Reflect)]
pub struct ScoreText;

/// Tells how long a floating score text remains visible before it disappears.
#[derive(Component, Reflect, Deref, DerefMut)]
pub struct ScoreTextTimer(pub Timer);

/// Keeps track of how many ghosts pacman ate during an active energizer
#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct EatenGhostCounter(pub usize);
