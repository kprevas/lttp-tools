use byteorder::*;
use failure::Error;
use midi::MidiHandler;
use std::fs::*;
use std::io::{Cursor, Write};
use std::path::*;

use serde_json;

mod command;
mod instruments;
mod track;

use self::track::*;

const PREAMBLE_TRACK_0: [u8; 6] = [
    0xfa, 0x19, // percussion offset
    0xe5, 0xc8, // global volume
    0xed, 0xc8, // channel volume
];

const PREAMBLE_OTHER_TRACK: [u8; 2] = [
    0xed, 0xc8, // channel volume
];

#[derive(Copy, Clone, Debug)]
pub struct CallLoopRef {
    pub target_track: usize,
    pub ref_pos: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    tracks: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    parts: Vec<Part>,
    tracks: Vec<Track>,
}

impl Song {
    pub fn from_midi(midi: &MidiHandler, tempo_factor: f32) -> Result<Song, Error> {
        let tracks: Result<Vec<Track>, Error> = (0..16)
            .filter_map(|voice| {
                match Track::new(
                    midi.events_for_voice(voice),
                    midi.ticks_per_beat,
                    tempo_factor,
                ) {
                    Ok(track) => {
                        if track.commands.is_empty() {
                            None
                        } else {
                            Some(Ok(track))
                        }
                    }
                    Err(err) => Some(Err(err)),
                }
            })
            .collect();
        match tracks {
            Ok(tracks) => {
                let mut parts = Vec::new();
                let part = Part {
                    tracks: tracks.iter().enumerate().map(|(i, _)| i).collect(),
                };
                parts.push(part);
                let tracks = Song::optimize_call_loops(tracks);
                Ok(Song { parts, tracks })
            }
            Err(err) => Err(err),
        }
    }

    fn optimize_call_loops(tracks: Vec<Track>) -> Vec<Track> {
        tracks
    }

    pub fn from_json(path: &Path) -> Song {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn write_to_json(&self, path: &Path) {
        let out = File::create(path).unwrap();
        serde_json::to_writer_pretty(out, self).unwrap()
    }

    pub fn get_part_tracks(&self, part_idx: usize) -> &[usize] {
        self.parts[part_idx].tracks.as_slice()
    }

    pub fn get_num_tracks(&self) -> usize {
        self.tracks.len()
    }

    pub fn write_track(
        &self,
        out: &mut Cursor<Vec<u8>>,
        track_idx: usize,
        call_loops: &mut Vec<CallLoopRef>,
    ) -> Result<(), Error> {
        let track = &self.tracks[track_idx];
        if !track.commands.is_empty() {
            if self.parts.iter().any(|part| part.tracks[0] == track_idx) {
                out.write(&PREAMBLE_TRACK_0)?;
            } else {
                out.write(&PREAMBLE_OTHER_TRACK)?;
            }
            track.write(out, call_loops)?;
            out.write_u8(0x00)?;
            out.write_u8(0x00)?;
        }
        Ok(())
    }
}
