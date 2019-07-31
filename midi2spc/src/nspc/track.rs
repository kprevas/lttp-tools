use ghakuf::messages::*;
use simple_error::SimpleError;
use std::error::Error;
use std::io::Cursor;
use serde_derive::{Serialize, Deserialize};
use super::command::*;
use super::instruments::*;
use super::CallLoopRef;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub commands: Vec<ParameterizedCommand>,
}

#[derive(Debug)]
struct Duration {
    length: u8,
    quantized_ticks: u32,
    overflow_count: u8,
}

impl Track {
    fn get_duration(ticks: u32, ticks_per_beat: u16, ceil: bool) -> Duration {
        let length_beats = (ticks as f32) / (ticks_per_beat as f32);
        let length = length_beats * 24.0;
        let quantized_length = if ceil {
            length.ceil() as u32
        } else {
            length.floor() as u32
        };
        let quantized_ticks = quantized_length * (ticks_per_beat as u32) / 24;
        if quantized_length > 0x7f {
            let overflow_amount = quantized_length % 0x7f;
            let overflow_count = quantized_length / 0x7f;
            if overflow_amount == 0 {
                Duration {
                    length: 0x7f,
                    quantized_ticks,
                    overflow_count: (overflow_count - 1) as u8,
                }
            } else {
                Duration {
                    length: overflow_amount as u8,
                    quantized_ticks,
                    overflow_count: overflow_count as u8,
                }
            }
        } else {
            Duration {
                length: quantized_length as u8,
                quantized_ticks,
                overflow_count: 0,
            }
        }
    }

    fn insert_rest(
        commands: &mut Vec<ParameterizedCommand>,
        last_note_end: u32,
        abs_time: u32,
        ticks_per_beat: u16,
    ) -> u32 {
        if abs_time > last_note_end {
            let duration = Track::get_duration(abs_time - last_note_end, ticks_per_beat, false);
            for _ in 0..duration.overflow_count {
                commands.push(ParameterizedCommand::new(
                    Some(0x7f),
                    None,
                    None,
                    Command::Rest,
                ));
            }
            if duration.length > 0 {
                commands.push(ParameterizedCommand::new(
                    Some(duration.length),
                    None,
                    None,
                    Command::Rest,
                ));
            }
            last_note_end + duration.quantized_ticks
        } else {
            last_note_end
        }
    }

    pub fn new(
        events: &Vec<(Message, u32)>,
        ticks_per_beat: u16,
        max_time: u32,
        tempo_factor: f32,
        voice: usize,
    ) -> Result<Track, Box<Error>> {
        let mut commands = Vec::new();
        let mut note_start: Option<u32> = None;
        let mut note_velocity = 0;
        let mut last_note_end = 0u32;
        let mut portamento = false;
        let mut port_time = 0u16;
        for &(ref message, abs_time) in events {
            match *message {
                Message::MetaEvent {
                    ref event,
                    ref data,
                    ..
                } => {
                    if let MetaEvent::SetTempo = *event {
                        let usec_per_beat = (data[0] as u32) * 0x10000
                            + (data[1] as u32) * 0x100
                            + (data[2] as u32);
                        let bpm = (6e7 / (usec_per_beat as f32)).ceil();
                        commands.push(ParameterizedCommand::new(
                            None,
                            None,
                            None,
                            Command::Tempo((bpm * tempo_factor) as u8),
                        ))
                    }
                }
                Message::MidiEvent { ref event, .. } => {
                    match *event {
                        MidiEvent::NoteOff { note, .. } => {
                            if let Some(start) = note_start {
                                let duration =
                                    Track::get_duration(abs_time - start, ticks_per_beat, true);
                                let push_as_tie =
                                    commands.last().map_or(false, |cmd| cmd.is_slide());
                                commands.push(ParameterizedCommand::new(
                                    Some(if duration.overflow_count > 0 {
                                        0x7f
                                    } else {
                                        duration.length
                                    }),
                                    Some(note_velocity / 8),
                                    Some(7),
                                    if push_as_tie {
                                        Command::Tie
                                    } else {
                                        Command::Note(note + 0x68)
                                    },
                                ));
                                for i in 0..duration.overflow_count {
                                    commands.push(ParameterizedCommand::new(
                                        Some(if i < duration.overflow_count - 1 {
                                            0x7f
                                        } else {
                                            duration.length
                                        }),
                                        None,
                                        None,
                                        Command::Tie,
                                    ));
                                }
                                last_note_end = start + duration.quantized_ticks;
                                note_start = None;
                            }
                        }
                        MidiEvent::NoteOn { velocity, note, .. } => {
                            last_note_end = Track::insert_rest(
                                &mut commands,
                                last_note_end,
                                abs_time,
                                ticks_per_beat,
                            );
                            if portamento && !commands.is_empty() {
                                let pitch_slide: Option<ParameterizedCommand>;
                                {
                                    let last_command = commands.last();
                                    pitch_slide = last_command.and_then(|cmd| {
                                        cmd.create_pitch_slide(
                                            Track::get_duration(
                                                port_time as u32,
                                                ticks_per_beat,
                                                true,
                                            )
                                            .length,
                                            note + 0x68,
                                        )
                                    });
                                }
                                pitch_slide.map(|slide| commands.push(slide));
                            }
                            if note_start.is_some() {
                                return Err(Box::from(SimpleError::new(format!("More than one voice needed on voice {}: notes start at {} and {}", voice, note_start.unwrap(), abs_time))));
                            }
                            note_start = Some(last_note_end);
                            note_velocity = velocity;
                        }
                        MidiEvent::PolyphonicKeyPressure { .. } => {
                            // TODO
                        }
                        MidiEvent::ControlChange { control, data, .. } => {
                            match control {
                                5 => {
                                    // portamento time high byte
                                    port_time = ((data as u16) << 8) | (port_time & 0xFF);
                                }
                                7 => {
                                    // channel volume
                                    commands.push(ParameterizedCommand::new(
                                        None,
                                        None,
                                        None,
                                        Command::ChannelVolume(data * 2),
                                    ));
                                }
                                37 => {
                                    // portamento time low byte
                                    port_time = (port_time & 0xFF00) | (data as u16);
                                }
                                65 => {
                                    // portamento on/off
                                    portamento = data >= 64;
                                }
                                _ => {}
                            }
                            // TODO
                        }
                        MidiEvent::ProgramChange { ch, program, .. } => {
                            last_note_end = Track::insert_rest(
                                &mut commands,
                                last_note_end,
                                abs_time,
                                ticks_per_beat,
                            );
                            let instrument = if ch == 9 {
                                program
                            } else {
                                INSTRUMENT_MAP[program as usize]
                            };
                            commands.push(ParameterizedCommand::new(
                                None,
                                None,
                                None,
                                Command::SetInstrument(instrument),
                            ));
                        }
                        MidiEvent::ChannelPressure { .. } => {
                            // TODO
                        }
                        MidiEvent::PitchBendChange { .. } => {
                            // TODO
                        }
                        _ => {}
                    }
                }
                Message::SysExEvent { .. } => {
                    // TODO (global volume?)
                }
                _ => {}
            }
        }
        let mut commands_with_sustain = Vec::new();
        if !commands.is_empty() {
            if max_time > last_note_end {
                Track::insert_rest(&mut commands, last_note_end, max_time, ticks_per_beat);
            }
            let mut skip_next_rest = false;
            for i in 0..commands.len() - 1 {
                if skip_next_rest {
                    skip_next_rest = false;
                    continue;
                }
                let next = &commands[i + 1].clone();
                let command = &mut commands[i];
                skip_next_rest = command.set_sustain(next);
                commands_with_sustain.push(command.clone());
            }
            if !skip_next_rest {
                commands_with_sustain.push(commands.last().unwrap().clone());
            }
        }
        Ok(Track {
            commands: commands_with_sustain,
        })
    }

    pub fn write(
        &self,
        out: &mut Cursor<Vec<u8>>,
        call_loops: &mut Vec<CallLoopRef>,
    ) -> Result<(), Box<Error>> {
        let mut duration = 0xff;
        let mut velocity = None;
        for cmd in &self.commands {
            let (duration_out, velocity_out) = cmd.write(out, duration, velocity, call_loops)?;
            duration = duration_out;
            velocity = velocity_out;
        }
        Ok(())
    }
}
