use failure::Error;
use ghakuf::messages::*;
use std::io::Cursor;

use super::command::*;
use super::instruments::*;
use super::CallLoopRef;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub commands: Vec<ParameterizedCommand>,
}

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
            Duration {
                length: (quantized_length % 0x7f) as u8,
                quantized_ticks,
                overflow_count: (quantized_length / 0x7f) as u8,
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
                commands.push(ParameterizedCommand::new(Some(0x7f), None, Command::Rest));
            }
            if duration.length > 0 {
                commands.push(ParameterizedCommand::new(
                    Some(duration.length),
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
        tempo_factor: f32,
        voice: usize,
    ) -> Result<Track, Error> {
        let mut commands = Vec::new();
        let mut note_start: Option<u32> = None;
        let mut last_note_end = 0u32;
        let mut last_ch11_instr = 0;
        for &(ref message, abs_time) in events {
            match *message {
                Message::MetaEvent {
                    ref event,
                    ref data,
                    ..
                } => {
                    if let MetaEvent::SetTempo = *event {
                        last_note_end = Track::insert_rest(
                            &mut commands,
                            last_note_end,
                            abs_time,
                            ticks_per_beat,
                        );
                        let usec_per_beat = (data[0] as u32) * 0x10000
                            + (data[1] as u32) * 0x100
                            + (data[2] as u32);
                        let bpm = usec_per_beat / 6000;
                        commands.push(ParameterizedCommand::new(
                            None,
                            None,
                            Command::Tempo((bpm as f32 * tempo_factor) as u8),
                        ))
                    }
                }
                Message::MidiEvent { ref event, .. } => {
                    match *event {
                        MidiEvent::NoteOff { ch, note, .. } => {
                            if let Some(start) = note_start {
                                let duration =
                                    Track::get_duration(abs_time - start, ticks_per_beat, true);
                                if ch == 10 {
                                    let mut instr;
                                    if note == 0x3e || note == 0x40 {
                                        instr = SNARE;
                                    } else {
                                        instr = CYMBAL;
                                    }
                                    if last_ch11_instr != instr {
                                        commands.push(ParameterizedCommand::new(
                                            None,
                                            None,
                                            Command::SetInstrument(instr),
                                        ));
                                        last_ch11_instr = instr;
                                    }
                                }
                                commands.push(ParameterizedCommand::new(
                                    Some(if duration.overflow_count > 0 {
                                        0x7f
                                    } else {
                                        duration.length
                                    }),
                                    None,
                                    Command::Note(note + 0x68),
                                ));
                                for i in 0..duration.overflow_count {
                                    commands.push(ParameterizedCommand::new(
                                        Some(if i < duration.overflow_count - 1 {
                                            0x7f
                                        } else {
                                            duration.length
                                        }),
                                        None,
                                        Command::Tie,
                                    ));
                                }
                                last_note_end = start + duration.quantized_ticks;
                                note_start = None;
                            }
                        }
                        MidiEvent::NoteOn { .. } => {
                            last_note_end = Track::insert_rest(
                                &mut commands,
                                last_note_end,
                                abs_time,
                                ticks_per_beat,
                            );
                            if note_start.is_some() {
                                bail!("More than one voice needed on voice {}: notes start at {} and {}", voice, note_start.unwrap(), abs_time);
                            }
                            note_start = Some(abs_time)
                        }
                        MidiEvent::PolyphonicKeyPressure { .. } => {
                            // TODO
                        }
                        MidiEvent::ControlChange { control, data, .. } => {
                            match control {
                                7 => {
                                    // channel volume
                                    commands.push(ParameterizedCommand::new(
                                        None,
                                        None,
                                        Command::ChannelVolume(data * 2),
                                    ));
                                }
                                _ => {}
                            }
                            // TODO
                        }
                        MidiEvent::ProgramChange { program, .. } => {
                            last_note_end = Track::insert_rest(
                                &mut commands,
                                last_note_end,
                                abs_time,
                                ticks_per_beat,
                            );
                            commands.push(ParameterizedCommand::new(
                                None,
                                None,
                                Command::SetInstrument(INSTRUMENT_MAP[program as usize]),
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
        Ok(Track { commands })
    }

    pub fn write(
        &self,
        out: &mut Cursor<Vec<u8>>,
        call_loops: &mut Vec<CallLoopRef>,
    ) -> Result<(), Error> {
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
