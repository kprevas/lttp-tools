use byteorder::*;
use failure::Error;
use std::io::Cursor;

use super::CallLoopRef;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sustain_not_note() {
        let mut first = ParameterizedCommand::new(Some(1), None, None, Command::Rest);
        let second = ParameterizedCommand::new(Some(1), None, None, Command::Rest);
        assert!(!first.set_sustain(&second));
        assert!(first.sustain.is_none());
        assert_eq!(1, first.duration.unwrap());
    }

    #[test]
    fn test_sustain_not_followed_by_rest() {
        let mut first = ParameterizedCommand::new(Some(1), None, None, Command::Note(0));
        let second = ParameterizedCommand::new(Some(1), None, None, Command::Tie);
        assert!(!first.set_sustain(&second));
        assert!(first.sustain.is_none());
        assert_eq!(1, first.duration.unwrap());
    }

    #[test]
    fn test_sustain_half() {
        let mut first = ParameterizedCommand::new(Some(1), None, None, Command::Note(0));
        let second = ParameterizedCommand::new(Some(1), None, None, Command::Rest);
        assert!(first.set_sustain(&second));
        assert_eq!(4, first.sustain.unwrap());
        assert_eq!(2, first.duration.unwrap());
    }

    #[test]
    fn test_sustain_eighth() {
        let mut first = ParameterizedCommand::new(Some(1), None, None, Command::Note(0));
        let second = ParameterizedCommand::new(Some(7), None, None, Command::Rest);
        assert!(first.set_sustain(&second));
        assert_eq!(1, first.sustain.unwrap());
        assert_eq!(8, first.duration.unwrap());
    }

    #[test]
    fn test_sustain_five_eighths() {
        let mut first = ParameterizedCommand::new(Some(5), None, None, Command::Note(0));
        let second = ParameterizedCommand::new(Some(3), None, None, Command::Rest);
        assert!(first.set_sustain(&second));
        assert_eq!(5, first.sustain.unwrap());
        assert_eq!(8, first.duration.unwrap());
    }

    #[test]
    fn test_sustain_too_short() {
        let mut first = ParameterizedCommand::new(Some(1), None, None, Command::Note(0));
        let second = ParameterizedCommand::new(Some(12), None, None, Command::Rest);
        assert!(!first.set_sustain(&second));
        assert!(first.sustain.is_none());
        assert_eq!(1, first.duration.unwrap());
    }

    #[test]
    fn test_sustain_too_long() {
        let mut first = ParameterizedCommand::new(Some(12), None, None, Command::Note(0));
        let second = ParameterizedCommand::new(Some(1), None, None, Command::Rest);
        assert!(!first.set_sustain(&second));
        assert!(first.sustain.is_none());
        assert_eq!(12, first.duration.unwrap());
    }
}

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
    sustain: Option<u8>,
    command: Command,
}

impl ParameterizedCommand {
    pub fn new(
        duration: Option<u8>,
        velocity: Option<u8>,
        sustain: Option<u8>,
        command: Command,
    ) -> ParameterizedCommand {
        ParameterizedCommand {
            duration,
            velocity,
            sustain,
            command,
        }
    }

    pub fn write(
        &self,
        out: &mut Cursor<Vec<u8>>,
        prev_duration: u8,
        prev_velocity_sustain: Option<u8>,
        call_loops: &mut Vec<CallLoopRef>,
    ) -> Result<(u8, Option<u8>), Error> {
        let mut duration_out = prev_duration;
        let mut velocity_sustain_out = prev_velocity_sustain;
        let mut duration_to_write = None;
        if let Some(duration) = self.duration {
            if duration != prev_duration && duration > 0 {
                duration_out = duration;
                duration_to_write = Some(duration);
            }
        }
        let mut velocity_sustain = None;
        if self.velocity.is_some() || self.sustain.is_some() {
            let mut velocity_sustain_value = prev_velocity_sustain.unwrap_or(0x7d);
            let mut nonzero = false;
            if let Some(velocity) = self.velocity {
                if velocity > 0 {
                    velocity_sustain_value = (velocity_sustain_value & 0x70) | velocity;
                    nonzero = true;
                }
            }
            if let Some(sustain) = self.sustain {
                if sustain > 0 {
                    velocity_sustain_value = (velocity_sustain_value & 0x0F) | (sustain << 4);
                    nonzero = true;
                }
            }
            if nonzero {
                velocity_sustain = Some(velocity_sustain_value);
            }
        }
        let mut velocity_sustain_to_write = None;
        if let Some(velocity_sustain) = velocity_sustain {
            if prev_velocity_sustain.is_none() || prev_velocity_sustain.unwrap() != velocity_sustain {
                velocity_sustain_out = Some(velocity_sustain);
                velocity_sustain_to_write = Some(velocity_sustain);
            }
        } else if prev_velocity_sustain.is_none() {
            if let Command::Note( .. ) = self.command {
                velocity_sustain_out = Some(0x7d);
                velocity_sustain_to_write = Some(0x7d);
            }
        }
        if let Some(duration) = duration_to_write {
            out.write_u8(duration)?;
        } else if velocity_sustain_to_write.is_some() {
            out.write_u8(prev_duration)?;
        }
        if let Some(velocity_sustain) = velocity_sustain_to_write {
            out.write_u8(velocity_sustain)?;
        }
        self.command.write(out, call_loops)?;
        Ok((duration_out, velocity_sustain_out))
    }

    pub fn call_loop_eligible(&self) -> bool {
        match self.command {
            Command::CallLoop(_, _) => false,
            _ => true,
        }
    }

    pub fn set_sustain(&mut self, next_command: &ParameterizedCommand) -> bool {
        if let Command::Note(..) = self.command {
            if let Command::Rest = next_command.command {
                let note_duration = self.duration.unwrap() as u16;
                let total_duration = note_duration + (next_command.duration.unwrap() as u16);
                if (note_duration * 8) % total_duration == 0 {
                    let sustain = ((note_duration * 8) / total_duration) as u8;
                    self.sustain = Some(sustain.min(7));
                    self.duration = Some(total_duration as u8);
                    return true;
                }
            }
        }
        false
    }
}
