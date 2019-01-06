use std::fs::File;
use std::path::{Path, PathBuf};
use serde_json;
use serde_json::Value;

const DEFAULT_TEMPO_ADJUST: f64 = 0.3;

const OVERWORLD_SONGS: [&str;15] = ["Title", "World Map", "Beginning", "Rabbit", "Forest", "Intro", "Town", "Warp", "Dark World", "Master Sword", "File Select", "Soldier", "Mountain", "Shop", "Fanfare"];
const INDOOR_SONGS: [&str;16] = ["Castle", "Palace", "Cave", "Clear", "Church", "Boss", "Dungeon", "Psychic", "Secret Way", "Rescue", "Crystal", "Fountain", "Pyramid", "Kill Agahnim", "Ganon Room", "Last Boss"];
const ENDING_SONGS: [&str;3] = ["Triforce", "Ending", "Staff"];

#[derive(Debug)]
pub struct Song {
    pub input: PathBuf,
    pub tempo_factor: f32,
    pub loops: bool,
}

impl Song {
    pub fn new(input: &Value) -> Song {
        Song {
            input: PathBuf::from(input["input"].as_str().unwrap()),
            tempo_factor: input["tempoAdjust"].as_f64().unwrap_or(DEFAULT_TEMPO_ADJUST) as f32,
            loops: input["loop"].as_bool().unwrap_or(true),
        }
    }
}

#[derive(Debug)]
pub struct Bank {
    pub songs: Vec<Song>,
}

impl Bank {
    pub fn new(input: &Value, song_names: &[&str]) -> Bank {
        Bank {
            songs: song_names.iter().map(|&song_name|
                Song::new(&input[song_name])
            ).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Manifest {
    pub banks: [Bank;3],
}

impl Manifest {
    pub fn new(path: &Path) -> Manifest {
        let reader = File::open(path).unwrap();
        let json: Value = serde_json::from_reader(reader).unwrap();
        Manifest {
            banks: [
                Bank::new(&json["overworld"], &OVERWORLD_SONGS),
                Bank::new(&json["indoor"], &INDOOR_SONGS),
                Bank::new(&json["ending"], &ENDING_SONGS),
            ],
        }
    }

    pub fn single_song(song_path: &Path) -> Manifest {
        Manifest {
            banks: [
                Bank {
                    songs: vec![
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                    ],
                },
                Bank {
                    songs: vec![
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                    ],
                },
                Bank {
                    songs: vec![
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                        Song { input: song_path.to_path_buf(), tempo_factor: 0.3, loops: true },
                    ],
                },
            ]
        }
    }
}