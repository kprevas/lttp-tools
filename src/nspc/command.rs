use byteorder::*;
use failure::Error;
use std::io::Cursor;

use super::CallLoopRef;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Command {
    Note(u8),
    Rest,
    Tie,
    SetInstrument(u8),
    Pan(u8),
    PanFade(u8, u8),
    Vibrato(u8, u8, u8),
    VibratoOff,
    MasterVolume(u8),
    MasterVolumeFade(u8, u8),
    Tempo(u8),
    TempoFade(u8, u8),
    GlobalTranspose(u8),
    ChannelTranspose(u8),
    Tremolo(u8, u8, u8),
    TremoloOff,
    ChannelVolume(u8),
    ChannelVolumeFade(u8),
    CallLoop(usize, u8),
    VibratoFade(u8),
    PitchEnvelopeTo(u8, u8, u8),
    PitchEnvelopeFrom(u8, u8, u8),
    PitchEnvelopeOff,
    Tuning(u8),
    EchoVolume(u8, u8, u8),
    EchoOff,
    EchoParams(u8, u8, u8),
    EchoVolumeFade(u8, u8, u8),
    PitchSlide(u8, u8, u8),
    PercussionPatchBase(u8),
}

impl Command {
    fn write(
        &self,
        out: &mut Cursor<Vec<u8>>,
        call_loops: &mut Vec<CallLoopRef>,
    ) -> Result<(), Error> {
        match *self {
            Command::Note(note) => {
                out.write_u8(note)?;
            }
            Command::Tie => {
                out.write_u8(0xc8)?;
            }
            Command::Rest => {
                out.write_u8(0xc9)?;
            }
            Command::SetInstrument(p1) => {
                out.write_u8(0xe0)?;
                out.write_u8(p1)?;
            }
            Command::Pan(p1) => {
                out.write_u8(0xe1)?;
                out.write_u8(p1)?;
            }
            Command::PanFade(p1, p2) => {
                out.write_u8(0xe2)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
            }
            Command::Vibrato(p1, p2, p3) => {
                out.write_u8(0xe3)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::VibratoOff => {
                out.write_u8(0xe4)?;
            }
            Command::MasterVolume(p1) => {
                out.write_u8(0xe5)?;
                out.write_u8(p1)?;
            }
            Command::MasterVolumeFade(p1, p2) => {
                out.write_u8(0xe6)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
            }
            Command::Tempo(p1) => {
                out.write_u8(0xe7)?;
                out.write_u8(p1)?;
            }
            Command::TempoFade(p1, p2) => {
                out.write_u8(0xe8)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
            }
            Command::GlobalTranspose(p1) => {
                out.write_u8(0xe9)?;
                out.write_u8(p1)?;
            }
            Command::ChannelTranspose(p1) => {
                out.write_u8(0xea)?;
                out.write_u8(p1)?;
            }
            Command::Tremolo(p1, p2, p3) => {
                out.write_u8(0xeb)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::TremoloOff => {
                out.write_u8(0xec)?;
            }
            Command::ChannelVolume(p1) => {
                out.write_u8(0xed)?;
                out.write_u8(p1)?;
            }
            Command::ChannelVolumeFade(p1) => {
                out.write_u8(0xee)?;
                out.write_u8(p1)?;
            }
            Command::CallLoop(p1, p2) => {
                out.write_u8(0xef)?;
                call_loops.push(CallLoopRef {
                    target_track: p1,
                    ref_pos: out.position(),
                });
                out.write_u8(0x00)?;
                out.write_u8(0xd0)?;
                out.write_u8(p2)?;
            }
            Command::VibratoFade(p1) => {
                out.write_u8(0xf0)?;
                out.write_u8(p1)?;
            }
            Command::PitchEnvelopeTo(p1, p2, p3) => {
                out.write_u8(0xf1)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::PitchEnvelopeFrom(p1, p2, p3) => {
                out.write_u8(0xf2)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::PitchEnvelopeOff => {
                out.write_u8(0xf3)?;
            }
            Command::Tuning(p1) => {
                out.write_u8(0xf4)?;
                out.write_u8(p1)?;
            }
            Command::EchoVolume(p1, p2, p3) => {
                out.write_u8(0xf5)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::EchoOff => {
                out.write_u8(0xf6)?;
            }
            Command::EchoParams(p1, p2, p3) => {
                out.write_u8(0xf7)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::EchoVolumeFade(p1, p2, p3) => {
                out.write_u8(0xf8)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::PitchSlide(p1, p2, p3) => {
                out.write_u8(0xf9)?;
                out.write_u8(p1)?;
                out.write_u8(p2)?;
                out.write_u8(p3)?;
            }
            Command::PercussionPatchBase(p1) => {
                out.write_u8(0xfa)?;
                out.write_u8(p1)?;
            }
        };
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ParameterizedCommand {
    duration: Option<u8>,
    velocity: Option<u8>,
    command: Command,
}

impl ParameterizedCommand {
    pub fn new(
        duration: Option<u8>,
        velocity: Option<u8>,
        command: Command,
    ) -> ParameterizedCommand {
        ParameterizedCommand {
            duration,
            velocity,
            command,
        }
    }

    pub fn write(
        &self,
        out: &mut Cursor<Vec<u8>>,
        prev_duration: u8,
        prev_velocity: Option<u8>,
        call_loops: &mut Vec<CallLoopRef>,
    ) -> Result<(u8, Option<u8>), Error> {
        let mut duration_out = prev_duration;
        let mut velocity_out = prev_velocity;
        match self.duration {
            Some(duration) => {
                if duration != prev_duration {
                    if duration > 0 {
                        out.write_u8(duration)?;
                    }
                    match self.velocity {
                        Some(velocity) => {
                            if prev_velocity.is_none() || prev_velocity.unwrap() != velocity {
                                if velocity > 0 {
                                    out.write_u8(velocity)?;
                                }
                                velocity_out = None;
                            }
                        }
                        _ => {
                            if prev_velocity.is_none() {
                                out.write_u8(0x7d)?;
                                velocity_out = Some(0x7d);
                            }
                        }
                    }
                    duration_out = duration;
                }
            }
            _ => {}
        }
        self.command.write(out, call_loops)?;
        Ok((duration_out, velocity_out))
    }

    pub fn call_loop_eligible(&self) -> bool {
        match self.command {
            Command::CallLoop(_, _) => false,
            _ => true,
        }
    }
}
