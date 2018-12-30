extern crate itertools;

use ghakuf::messages::*;
use ghakuf::reader::*;
use self::itertools::Itertools;
use std::path::Path;
use std::cmp::max;
use std::collections::HashSet;
use std::collections::HashMap;

fn permute(assignments: &Vec<Vec<usize>>, idx: usize, used: &mut HashSet<usize>) -> Vec<Vec<usize>> {
    let mut permutations = Vec::new();
    if idx < assignments.len() {
        for assignment in &assignments[idx] {
            if !used.contains(&assignment) {
                if idx < assignments.len() - 1 {
                    used.insert(*assignment);
                    let mut rest = permute(assignments, idx + 1, used);
                    for mut permutation in rest {
                        permutation.insert(0, *assignment);
                        permutations.push(permutation);
                    }
                    used.remove(&assignment);
                } else {
                    permutations.push(vec!(*assignment))
                }
            }
        }
    }
    permutations
}

fn delta_time(message: &Message) -> u32 {
    match *message {
        Message::MetaEvent { delta_time, .. } => delta_time,
        Message::MidiEvent { delta_time, .. } => delta_time,
        Message::SysExEvent { delta_time, .. } => delta_time,
        _ => 0,
    }
}

fn channel(event: &MidiEvent) -> usize {
    match *event {
        MidiEvent::NoteOff { ch, .. } => ch as usize,
        MidiEvent::NoteOn { ch, .. } => ch as usize,
        MidiEvent::PolyphonicKeyPressure { ch, .. } => ch as usize,
        MidiEvent::ControlChange { ch, .. } => ch as usize,
        MidiEvent::ProgramChange { ch, .. } => ch as usize,
        MidiEvent::ChannelPressure { ch, .. } => ch as usize,
        MidiEvent::PitchBendChange { ch, .. } => ch as usize,
        MidiEvent::Unknown { ch, .. } => ch as usize,
    }
}

fn priority(message: &Message) -> u8 {
    match *message {
        Message::MidiEvent { delta_time, ref event } => match *event {
            MidiEvent::NoteOff { .. } => 0,
            MidiEvent::NoteOn { .. } => 2,
            MidiEvent::PolyphonicKeyPressure { .. } => 1,
            MidiEvent::ControlChange { .. } => 1,
            MidiEvent::ProgramChange { .. } => 1,
            MidiEvent::ChannelPressure { .. } => 1,
            MidiEvent::PitchBendChange { .. } => 1,
            MidiEvent::Unknown { .. } => 1,
        },
        _ => 0,
    }
}

#[derive(Debug, Copy, Clone)]
struct VoiceInterval {
    start: u32,
    end: u32,
    voices: usize,
}

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
struct MidiChannel {
    messages: Vec<(Message, u32)>,
    intervals: Vec<VoiceInterval>,
    max_voices: usize,
    voices: [usize; 8],
}

impl MidiChannel {
    fn new() -> MidiChannel {
        MidiChannel {
            messages: Vec::new(),
            intervals: Vec::new(),
            max_voices: 0,
            voices: [0; 8],
        }
    }
}

#[derive(Debug)]
pub struct MidiHandler {
    tracks: Vec<MidiTrack>,
    channels: [MidiChannel; 16],
    pub ticks_per_beat: u16,
}

impl MidiHandler {
    pub fn new() -> MidiHandler {
        MidiHandler {
            tracks: Vec::new(),
            channels: [
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
                MidiChannel::new(),
            ],
            ticks_per_beat: 0,
        }
    }

    pub fn read(&mut self, path: &Path) {
        {
            let mut midi_reader = Reader::new(self, &path).unwrap();
            midi_reader.read();
        }
        self.tracks_to_channels();
        for mut channel in &mut self.channels {
            let mut intervals = &mut channel.intervals;
            let mut last_interval_end = 0u32;
            let mut active_voices = 0usize;
            for message in &channel.messages {
                match *message {
                    (Message::MidiEvent { delta_time, ref event }, abs_time) => {
                        match *event {
                            MidiEvent::NoteOff { ch, note, velocity } => {
                                if abs_time > last_interval_end {
                                    intervals.push(VoiceInterval {
                                        start: last_interval_end,
                                        end: abs_time,
                                        voices: active_voices,
                                    });
                                }
                                active_voices -= 1;
                                last_interval_end = abs_time;
                            }
                            MidiEvent::NoteOn { ch, note, velocity } => {
                                if abs_time > last_interval_end {
                                    intervals.push(VoiceInterval {
                                        start: last_interval_end,
                                        end: abs_time,
                                        voices: active_voices,
                                    });
                                }
                                active_voices += 1;
                                channel.max_voices = max(channel.max_voices, active_voices);
                                last_interval_end = abs_time;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        let active_intervals = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
        if !self.solve_voices(0, &active_intervals) {
            panic!("Couldn't fit notes into available channels");
        }
    }

    fn tracks_to_channels(&mut self) {
        for track in &self.tracks {
            let mut abs_time = 0;
            for message in &track.messages {
                match *message {
                    Message::MetaEvent { delta_time, ref event, ref data } => {
                        abs_time += delta_time;
                        self.channels[0].messages.push((message.clone(), abs_time));
                    }
                    Message::MidiEvent { delta_time, ref event } => {
                        abs_time += delta_time;
                        self.channels[channel(event)].messages.push((message.clone(), abs_time));
                    }
                    Message::SysExEvent { delta_time, ref event, ref data } => {
                        abs_time += delta_time;
                        self.channels[0].messages.push((message.clone(), abs_time));
                    }
                    _ => {}
                }
            }
        }
        for channel in &mut self.channels {
            channel.messages.sort_by_key(|&(ref event, abs_time)| (abs_time, priority(event)));
        }
    }

    fn solve_voices(&mut self, track_idx: usize, active_intervals: &Vec<Vec<VoiceInterval>>) -> bool {
        if track_idx == self.channels.len() {
            return true;
        }
        let permutations;
        {
            let track = &self.channels[track_idx];
            let mut assignments: Vec<Vec<usize>> = Vec::new();
            for voice_idx in 0..track.max_voices {
                assignments.push(Vec::new());
                for channel in 0..8usize {
                    let mut active_idx = 0;
                    let ref active = active_intervals[channel];
                    let mut overlaps = false;
                    for interval in &track.intervals {
                        if interval.voices <= voice_idx {
                            continue;
                        }
                        while active_idx < active.len() && active[active_idx].end <= interval.start {
                            active_idx += 1;
                        }
                        if active_idx < active.len() && active[active_idx].start < interval.end {
                            overlaps = true;
                            break;
                        }
                    }
                    if !overlaps {
                        assignments[voice_idx].push(channel);
                    }
                }
            }
            permutations = permute(&assignments, 0, &mut HashSet::new());
            if track.max_voices > 0 && permutations.is_empty() {
                return false;
            }
        }
        for permutation in permutations {
            let mut new_active_intervals = active_intervals.clone();
            for voice_idx in 0..permutation.len() {
                for interval in &(&self.channels[track_idx]).intervals {
                    if interval.voices > voice_idx {
                        new_active_intervals.get_mut(permutation[voice_idx]).unwrap().push(*interval);
                    }
                }
            }
            new_active_intervals.iter_mut().for_each(|channel_intervals| {
                channel_intervals.sort_by_key(|interval| interval.start)
            });
            if self.solve_voices(track_idx + 1, &new_active_intervals) {
                for perm_idx in 0..permutation.len() {
                    self.channels[track_idx].voices[perm_idx] = permutation[perm_idx];
                }
                return true;
            }
        }
        if self.channels[track_idx].max_voices == 0 {
            return self.solve_voices(track_idx + 1, &active_intervals);
        } else {
            return false;
        }
    }

    pub fn max_voice(&self) -> u8 {
        *self.channels.iter().map(|channel| channel.voices.iter().max().unwrap()).max().unwrap() as u8
    }

    pub fn events_for_voice(&self, voice: usize) -> Box<Vec<(Message, u32)>> {
        let mut message_box = Box::new(Vec::new());
        {
            let channels = &self.channels;
            let mut messages = message_box.as_mut();
            let mut abs_time = 0u32;
            let mut last_abs_time: Vec<u32> = Vec::new();
            let mut curr_event_idx: Vec<usize> = Vec::new();
            let mut last_key_pressure: Vec<Option<&Message>> = Vec::new();
            let mut last_ctrl_change: Vec<Option<&Message>> = Vec::new();
            let mut last_prog_change: Vec<Option<&Message>> = Vec::new();
            let mut last_channel_pressure: Vec<Option<&Message>> = Vec::new();
            let mut last_pitch_bend: Vec<Option<&Message>> = Vec::new();
            let mut active_notes: Vec<HashMap<u8, usize>> = Vec::new();
            for i in 0..self.channels.len() {
                last_abs_time.push(0);
                curr_event_idx.push(0);
                last_key_pressure.push(None);
                last_ctrl_change.push(None);
                last_prog_change.push(None);
                last_channel_pressure.push(None);
                last_pitch_bend.push(None);
                active_notes.push(HashMap::new());
            }
            let mut last_channel = 0xffusize;
            let mut channels_done = channels.iter().filter(|channel| channel.messages.is_empty()).count();
            while channels_done < channels.len() {
                let (next_channel, abs_time) = channels.iter().enumerate().min_by_key(|&(i, channel)| {
                    if curr_event_idx[i] == channel.messages.len() {
                        (u32::max_value(), u8::max_value())
                    } else {
                        let event = &channel.messages[curr_event_idx[i]];
                        (event.1, priority(&event.0))
                    }
                }).unwrap();
                let next_event = &channels[next_channel].messages[curr_event_idx[next_channel]];
                curr_event_idx[next_channel] += 1;
                if curr_event_idx[next_channel] == channels[next_channel].messages.len() {
                    channels_done += 1;
                }

                match *next_event {
                    (Message::MetaEvent { delta_time, ref event, ref data }, abs_time) => {
                        if let MetaEvent::SetTempo = *event {
                            if voice == 0 {
                                messages.push(next_event.clone())
                            }
                        }
                    }
                    (Message::MidiEvent { delta_time, ref event }, abs_time) => {
                        match *event {
                            MidiEvent::NoteOff { ch, note, velocity } => {
                                let ch = ch as usize;
                                let note_voice = active_notes[ch].remove(&note).unwrap();
                                if note_voice == voice {
                                    messages.push(next_event.clone());
                                }
                            }
                            MidiEvent::NoteOn { ch, note, velocity } => {
                                let ch = ch as usize;
                                let next_voice = channels[next_channel].voices[active_notes[ch].len()];
                                active_notes[ch].insert(note, next_voice);
                                if next_voice == voice {
                                    if ch != last_channel {
                                        last_key_pressure[ch].map(|event| messages.push((event.clone(), abs_time)));
                                        last_ctrl_change[ch].map(|event| messages.push((event.clone(), abs_time)));
                                        last_prog_change[ch].map(|event| messages.push((event.clone(), abs_time)));
                                        last_channel_pressure[ch].map(|event| messages.push((event.clone(), abs_time)));
                                        last_pitch_bend[ch].map(|event| messages.push((event.clone(), abs_time)));
                                        last_channel = ch;
                                    }
                                    messages.push(next_event.clone());
                                }
                            }
                            MidiEvent::PolyphonicKeyPressure { ch, note, velocity } => {
                                let ch = ch as usize;
                                if last_channel == ch { messages.push(next_event.clone()) }
                                last_key_pressure[ch] = Some(&next_event.0);
                            }
                            MidiEvent::ControlChange { ch, control, data } => {
                                let ch = ch as usize;
                                if last_channel == ch { messages.push(next_event.clone()) }
                                last_ctrl_change[ch] = Some(&next_event.0);
                            }
                            MidiEvent::ProgramChange { ch, program } => {
                                let ch = ch as usize;
                                if last_channel == ch { messages.push(next_event.clone()) }
                                last_prog_change[ch] = Some(&next_event.0);
                            }
                            MidiEvent::ChannelPressure { ch, pressure } => {
                                let ch = ch as usize;
                                if last_channel == ch { messages.push(next_event.clone()) }
                                last_channel_pressure[ch] = Some(&next_event.0);
                            }
                            MidiEvent::PitchBendChange { ch, data } => {
                                let ch = ch as usize;
                                if last_channel == ch { messages.push(next_event.clone()) }
                                last_pitch_bend[ch] = Some(&next_event.0);
                            }
                            _ => {}
                        }
                    }
                    (Message::SysExEvent { delta_time, ref event, ref data }, abs_time) => {
                        if voice == 0 { messages.push(next_event.clone()) }
                    }
                    _ => {}
                }
            }
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
