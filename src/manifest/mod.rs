use serde_json;
use serde_json::Value;
use std::fs::File;
use std::path::{Path, PathBuf};

pub const DEFAULT_TEMPO_ADJUST: f32 = 0.3;

const OVERWORLD_SONGS: [&str; 15] = [
    "Title",
    "World Map",
    "Beginning",
    "Rabbit",
    "Forest",
    "Intro",
    "Town",
    "Warp",
    "Dark World",
    "Master Sword",
    "File Select",
    "Soldier",
    "Mountain",
    "Shop",
    "Fanfare",
];
const INDOOR_SONGS: [&str; 16] = [
    "Castle",
    "Palace",
    "Cave",
    "Clear",
    "Church",
    "Boss",
    "Dungeon",
    "Psychic",
    "Secret Way",
    "Rescue",
    "Crystal",
    "Fountain",
    "Pyramid",
    "Kill Agahnim",
    "Ganon Room",
    "Last Boss",
];
const ENDING_SONGS: [&str; 3] = ["Triforce", "Ending", "Staff"];

#[derive(Debug)]
pub struct Song {
    pub input: Option<PathBuf>,
    pub tempo_factor: f32,
    pub loops: bool,
}

impl Song {
    pub fn new(input: &Value) -> Song {
        Song {
            input: Some(PathBuf::from(input["input"].as_str().unwrap())),
            tempo_factor: input["tempoAdjust"]
                .as_f64()
                .unwrap_or(DEFAULT_TEMPO_ADJUST as f64) as f32,
            loops: input["loop"].as_bool().unwrap_or(true),
        }
    }

    pub fn default(path: &Path) -> Song {
        Song {
            input: Some(path.to_path_buf()),
            tempo_factor: DEFAULT_TEMPO_ADJUST,
            loops: true,
        }
    }

    pub fn empty() -> Song {
        Song {
            input: None,
            tempo_factor: DEFAULT_TEMPO_ADJUST,
            loops: false,
        }
    }
}

#[derive(Debug)]
pub struct Bank {
    pub name: &'static str,
    pub songs: Vec<Song>,
}

impl Bank {
    pub fn new(input: &Value, name: &'static str, song_names: &[&str]) -> Bank {
        Bank {
            name,
            songs: song_names
                .iter()
                .map(|&song_name| Song::new(&input[song_name]))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Manifest {
    pub banks: [Bank; 3],
}

impl Manifest {
    pub fn new(path: &Path) -> Manifest {
        let reader = File::open(path).unwrap();
        let json: Value = serde_json::from_reader(reader).unwrap();
        Manifest {
            banks: [
                Bank::new(&json["overworld"], "overworld", &OVERWORLD_SONGS),
                Bank::new(&json["indoor"], "indoor", &INDOOR_SONGS),
                Bank::new(&json["ending"], "ending", &ENDING_SONGS),
            ],
        }
    }

    pub fn single_song(song_path: &Path) -> Manifest {
        Manifest {
            banks: [
                Bank {
                    name: "Overworld",
                    songs: vec![
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                    ],
                },
                Bank {
                    name: "Indoor",
                    songs: vec![
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                    ],
                },
                Bank {
                    name: "Ending",
                    songs: vec![
                        Song::default(song_path),
                        Song::default(song_path),
                        Song::default(song_path),
                    ],
                },
            ],
        }
    }

    pub fn file_select(song_path: &Path) -> Manifest {
        Manifest {
            banks: [
                Bank {
                    name: "Overworld",
                    songs: vec![
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::default(song_path),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                    ],
                },
                Bank {
                    name: "Indoor",
                    songs: vec![
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                        Song::empty(),
                    ],
                },
                Bank {
                    name: "Ending",
                    songs: vec![Song::empty(), Song::empty(), Song::empty()],
                },
            ],
        }
    }
}
