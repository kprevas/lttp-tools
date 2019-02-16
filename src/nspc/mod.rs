use byteorder::*;
use failure::Error;
use midi::MidiHandler;
use std::fs::*;
use std::io::{Cursor, Write};
use std::path::*;

use serde_json;

mod command;
mod instruments;
mod seqtree;
mod track;

use self::command::*;
use self::seqtree::*;
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
    pub fn from_midi(
        midi: &MidiHandler,
        tempo_factor: f32,
        optimize_loops: bool,
        verbose: bool,
    ) -> Result<Song, Error> {
        let tracks: Result<Vec<Track>, Error> = (0..8)
            .filter_map(|voice| {
                match Track::new(
                    midi.events_for_voice(voice),
                    midi.ticks_per_beat,
                    tempo_factor,
                    voice,
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
                if optimize_loops {
                    let top_level_tracks = tracks.len();
                    Ok(Song {
                        parts,
                        tracks: Song::optimize_call_loops(tracks, top_level_tracks, verbose),
                    })
                } else {
                    Ok(Song { parts, tracks })
                }
            }
            Err(err) => Err(err),
        }
    }

    fn optimize_call_loops(
        tracks: Vec<Track>,
        top_level_tracks: usize,
        verbose: bool,
    ) -> Vec<Track> {
        let mut seqtree = SeqTree::new();
        for (i, track) in tracks.iter().take(top_level_tracks).enumerate() {
            seqtree.add_track(track, i);
        }
        let best_sequence = seqtree.best_sequence();
        if verbose {
            println!("optimal call loop sequence {:?}", best_sequence);
        };
        match best_sequence {
            None => tracks,
            Some(seq) => Song::optimize_call_loops(
                Song::extract_sequence(tracks, seq, top_level_tracks),
                top_level_tracks,
                verbose,
            ),
        }
    }

    fn extract_sequence(
        tracks: Vec<Track>,
        sequence: Sequence,
        top_level_tracks: usize,
    ) -> Vec<Track> {
        let sequence_length = sequence.commands.len();
        let mut new_tracks = Vec::new();
        for (i, track) in tracks.iter().take(top_level_tracks).enumerate() {
            let mut new_track = Track {
                commands: Vec::new(),
            };
            let locations = sequence.locations.iter().filter(|loc| loc.track_idx == i);
            let mut last_index: Option<usize> = None;
            for location in locations {
                new_track
                    .commands
                    .extend_from_slice(&track.commands[last_index.unwrap_or(0)..location.cmd_idx]);
                new_track.commands.push(ParameterizedCommand::new(
                    Some(0),
                    Some(0),
                    Some(0),
                    Command::CallLoop(tracks.len(), location.repeat_count),
                ));
                last_index =
                    Some(location.cmd_idx + sequence_length * location.repeat_count as usize);
            }
            if last_index.is_none() || last_index.unwrap() < track.commands.len() {
                new_track.commands.extend_from_slice(
                    &track.commands[last_index.unwrap_or(0)..track.commands.len()],
                );
            }
            new_tracks.push(new_track);
        }
        if tracks.len() > top_level_tracks {
            new_tracks.extend_from_slice(&tracks[top_level_tracks..tracks.len()]);
        }
        new_tracks.push(Track {
            commands: sequence.commands,
        });
        new_tracks
    }

    pub fn from_json(path: &Path) -> Song {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn write_to_json(&self, path: &Path) {
        let out = File::create(path).unwrap();
        serde_json::to_writer_pretty(out, self).unwrap()
    }

    pub fn empty() -> Result<Song, Error> {
        Ok(Song {
            parts: vec![Part { tracks: vec![0] }],
            tracks: vec![Track::new(&vec![], 24, 0.3, 0)?],
        })
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
            } else if self
                .parts
                .iter()
                .any(|part| part.tracks.contains(&track_idx))
            {
                out.write(&PREAMBLE_OTHER_TRACK)?;
            }
            track.write(out, call_loops)?;
            out.write_u8(0x00)?;
        }
        Ok(())
    }
}
