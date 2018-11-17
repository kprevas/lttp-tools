extern crate itertools;

use ghakuf::messages::*;
use ghakuf::reader::*;
use self::itertools::Itertools;
use std::path::Path;

#[derive(Debug)]
struct MidiTrack {
    messages: Vec<Message>,
}

impl MidiTrack {
    fn new() -> MidiTrack {
        MidiTrack {
            messages: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct MidiHandler {
    tracks: Vec<MidiTrack>,
    pub channel_map: [u8; 8],
    pub ticks_per_beat: u16,
}

impl MidiHandler {
    pub fn new() -> MidiHandler {
        MidiHandler {
            tracks: Vec::new(),
            channel_map: [0xff; 8],
            ticks_per_beat: 0,
        }
    }

    pub fn read(&mut self, path: &Path) {
        {
            let mut midi_reader = Reader::new(self, &path).unwrap();
            midi_reader.read();
        }

        let used_channels: Vec<u8> = self.tracks.iter().flat_map(|track| {
            track.messages.iter().filter_map(|message| {
                match *message {
                    Message::MidiEvent { delta_time, ref event } => {
                        match *event {
                            MidiEvent::NoteOff { ch, note, velocity } => Some(ch),
                            MidiEvent::NoteOn { ch, note, velocity } => Some(ch),
                            MidiEvent::PolyphonicKeyPressure { ch, note, velocity } => Some(ch),
                            MidiEvent::ControlChange { ch, control, data } => Some(ch),
                            MidiEvent::ProgramChange { ch, program } => Some(ch),
                            MidiEvent::ChannelPressure { ch, pressure } => Some(ch),
                            MidiEvent::PitchBendChange { ch, data } => Some(ch),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            })
        }).unique().collect();
        assert!(used_channels.len() <= 8, "Too many channels used ({})", used_channels.len());
        for (i, ch) in used_channels.iter().enumerate() {
            self.channel_map[i] = *ch;
        }
    }

    pub fn events_for_channel(&self, channel: u8) -> Box<Vec<(u32, Message)>> {
        let mut message_box = Box::new(Vec::new());
        {
            let mut messages = message_box.as_mut();
            for track in &self.tracks {
                let mut abs_time = 0u32;
                for message in &track.messages {
                    match *message {
                        Message::MetaEvent { delta_time, ref event, ref data } => {
                            abs_time += delta_time;
                            if let MetaEvent::SetTempo = *event {
                                if channel == self.channel_map[0] {
                                    messages.push((abs_time, message.clone()))
                                }
                            }
                        }
                        Message::MidiEvent { delta_time, ref event } => {
                            abs_time += delta_time;
                            match *event {
                                MidiEvent::NoteOff { ch, note, velocity } => {
                                    if ch == channel { messages.push((abs_time, message.clone())) }
                                }
                                MidiEvent::NoteOn { ch, note, velocity } => {
                                    if ch == channel { messages.push((abs_time, message.clone())) }
                                }
                                MidiEvent::PolyphonicKeyPressure { ch, note, velocity } => {
                                    if ch == channel { messages.push((abs_time, message.clone())) }
                                }
                                MidiEvent::ControlChange { ch, control, data } => {
                                    if ch == channel { messages.push((abs_time, message.clone())) }
                                }
                                MidiEvent::ProgramChange { ch, program } => {
                                    if ch == channel { messages.push((abs_time, message.clone())) }
                                }
                                MidiEvent::ChannelPressure { ch, pressure } => {
                                    if ch == channel { messages.push((abs_time, message.clone())) }
                                }
                                MidiEvent::PitchBendChange { ch, data } => {
                                    if ch == channel { messages.push((abs_time, message.clone())) }
                                }
                                _ => {}
                            }
                        }
                        Message::SysExEvent { delta_time, ref event, ref data } => {
                            abs_time += delta_time;
                            if channel == self.channel_map[0] { messages.push((abs_time, message.clone())) }
                        }
                        _ => {}
                    }
                }
            }
            messages.sort_by_key(|&(abs_time, _)| abs_time)
        }
        message_box
    }
}

impl Handler for MidiHandler {
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        if format != 1 {
            unimplemented!("MIDI format {}", format)
        }
        self.ticks_per_beat = time_base;
    }

    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        self.tracks.last_mut().unwrap().messages.push(Message::MetaEvent { delta_time, event: event.clone(), data: data.clone() });
    }

    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        self.tracks.last_mut().unwrap().messages.push(Message::MidiEvent { delta_time, event: event.clone() });
    }

    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        self.tracks.last_mut().unwrap().messages.push(Message::SysExEvent { delta_time, event: event.clone(), data: data.clone() });
    }

    fn track_change(&mut self) {
        self.tracks.push(MidiTrack::new());
    }
}
