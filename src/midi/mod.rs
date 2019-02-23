use failure::Error;
use ghakuf::messages::*;
use ghakuf::reader::*;
use itertools::*;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Fail)]
enum MidiError {
    #[fail(display = "{}: Couldn't fit notes into available channels", path)]
    CouldntFit { path: String },
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
        Message::MidiEvent { ref event, .. } => match *event {
            MidiEvent::NoteOff { .. } => 0,
            MidiEvent::NoteOn { velocity: 0, .. } => 0,
            MidiEvent::NoteOn { .. } => 2,
            MidiEvent::PolyphonicKeyPressure { .. } => 1,
            MidiEvent::ControlChange { .. } => 1,
            MidiEvent::ProgramChange { .. } => 1,
            MidiEvent::ChannelPressure { .. } => 1,
            MidiEvent::PitchBendChange { .. } => 1,
            MidiEvent::Unknown { .. } => 1,
        },
        _ => 1,
    }
}

#[derive(Debug, Copy, Clone)]
struct VoiceInterval {
    start: u32,
    end: u32,
    channel: usize,
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
    base_voice: usize,
}

impl MidiChannel {
    fn new() -> MidiChannel {
        MidiChannel {
            messages: Vec::new(),
            intervals: Vec::new(),
            base_voice: 0,
        }
    }
}

#[derive(Debug)]
struct MidiVoice {
    messages: Vec<(Message, u32)>,
}

impl MidiVoice {
    fn new() -> MidiVoice {
        MidiVoice {
            messages: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct MidiHandler {
    tracks: Vec<MidiTrack>,
    channels: [MidiChannel; 16],
    voices: [MidiVoice; 8],
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
            voices: [
                MidiVoice::new(),
                MidiVoice::new(),
                MidiVoice::new(),
                MidiVoice::new(),
                MidiVoice::new(),
                MidiVoice::new(),
                MidiVoice::new(),
                MidiVoice::new(),
            ],
            ticks_per_beat: 0,
        }
    }

    pub fn read(&mut self, path: &Path, verbose: bool) -> Result<(), Error> {
        if verbose {
            println!("reading {:?}", path);
        }
        {
            let mut midi_reader =
                Reader::new(self, path).map_err(|err| format_err!("MIDI read error: {:?}", err))?;
            midi_reader
                .read()
                .map_err(|err| format_err!("MIDI read error: {:?}", err))?;
        }
        self.tracks_to_channels(verbose);
        for (i, mut channel) in &mut self.channels.iter_mut().enumerate() {
            let mut intervals = &mut channel.intervals;
            let mut last_interval_end = 0u32;
            let mut active_voices = 0usize;
            for message in &channel.messages {
                match *message {
                    (Message::MidiEvent { ref event, .. }, abs_time) => match *event {
                        MidiEvent::NoteOff { .. } => {
                            if abs_time > last_interval_end {
                                intervals.push(VoiceInterval {
                                    start: last_interval_end,
                                    end: abs_time,
                                    channel: i,
                                    voices: active_voices,
                                });
                            }
                            active_voices -= 1;
                            last_interval_end = abs_time;
                        }
                        MidiEvent::NoteOn { velocity, .. } => {
                            if abs_time > last_interval_end {
                                intervals.push(VoiceInterval {
                                    start: last_interval_end,
                                    end: abs_time,
                                    channel: i,
                                    voices: active_voices,
                                });
                            }
                            if velocity == 0 {
                                active_voices -= 1;
                            } else {
                                active_voices += 1;
                            }
                            last_interval_end = abs_time;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        let active_base_intervals = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        self.find_base_voices(0, &active_base_intervals, path, verbose)?;
        self.channels_to_voices(path, verbose)?;
        Ok(())
    }

    fn tracks_to_channels(&mut self, verbose: bool) {
        for (i, track) in self.tracks.iter().enumerate() {
            if verbose {
                println!("extracting events from midi track {}", i);
            }
            let mut abs_time = 0;
            for message in &track.messages {
                match *message {
                    Message::MetaEvent { delta_time, .. } => {
                        abs_time += delta_time;
                        self.channels[0].messages.push((message.clone(), abs_time));
                    }
                    Message::MidiEvent {
                        delta_time,
                        ref event,
                    } => {
                        abs_time += delta_time;
                        self.channels[channel(event)]
                            .messages
                            .push((message.clone(), abs_time));
                    }
                    Message::SysExEvent { delta_time, .. } => {
                        abs_time += delta_time;
                        self.channels[0].messages.push((message.clone(), abs_time));
                    }
                    _ => {}
                }
            }
        }
        for channel in &mut self.channels {
            channel
                .messages
                .sort_by_key(|&(ref event, abs_time)| (abs_time, priority(event)));
        }
    }

    fn find_base_voices(
        &mut self,
        channel_idx: usize,
        active_base_intervals: &Vec<Vec<VoiceInterval>>,
        path: &Path,
        verbose: bool,
    ) -> Result<(), Error> {
        if channel_idx == self.channels.len() {
            return Ok(());
        }
        if verbose {
            println!("find base voice for channel {}", channel_idx);
        }
        let mut valid_voices = vec![];
        let mut found_empty = false;
        for i in 0..8 {
            let empty = active_base_intervals[i].is_empty();
            if !found_empty || !empty {
                match MidiHandler::overlapping_interval(
                    &self.channels[channel_idx].intervals,
                    &active_base_intervals[i],
                ) {
                    Some(interval) => {
                        if verbose {
                            println!("voice {} overlaps with {:?}", i, interval);
                        }
                    }
                    None => {
                        valid_voices.push(i);
                        if empty {
                            found_empty = true;
                        }
                    }
                }
            }
        }
        if verbose {
            println!("non-overlapping voices: {:?}", valid_voices);
        }
        for voice in valid_voices {
            if verbose {
                println!("channel {}: trying {}", channel_idx, voice);
            }
            let mut active_base_intervals = active_base_intervals.clone();
            active_base_intervals[voice]
                .extend_from_slice(self.channels[channel_idx].intervals.as_slice());
            match self.find_base_voices(channel_idx + 1, &active_base_intervals, path, verbose) {
                Ok(_) => {
                    self.channels[channel_idx].base_voice = voice;
                    return Ok(());
                }
                Err(_) => (),
            }
        }
        Err(Error::from(MidiError::CouldntFit {
            path: path.to_str().unwrap().to_owned(),
        }))
    }

    fn overlapping_interval(
        channel: &Vec<VoiceInterval>,
        active_intervals: &Vec<VoiceInterval>,
    ) -> Option<VoiceInterval> {
        for interval in channel {
            for existing_interval in active_intervals {
                if interval.voices > 0
                    && existing_interval.voices > 0
                    && interval.end > existing_interval.start
                    && interval.start < existing_interval.end
                {
                    return Some(existing_interval.clone());
                }
            }
        }
        None
    }

    fn channels_to_voices(&mut self, path: &Path, verbose: bool) -> Result<(), Error> {
        let channels = &self.channels;
        let mut last_abs_time: Vec<u32> = Vec::new();
        let mut curr_event_idx: Vec<usize> = Vec::new();
        let mut last_ctrl_change_per_channel: Vec<Option<&Message>> = Vec::new();
        let mut last_prog_change_per_channel: Vec<Option<&Message>> = Vec::new();
        let mut last_pitch_bend_per_channel: Vec<Option<&Message>> = Vec::new();
        let mut last_ctrl_change_per_voice: Vec<Option<&Message>> = Vec::new();
        let mut last_prog_change_per_voice: Vec<Option<&Message>> = Vec::new();
        let mut last_pitch_bend_per_voice: Vec<Option<&Message>> = Vec::new();
        let mut last_channel_per_voice: Vec<Option<usize>> = Vec::new();
        let mut active_notes: Vec<HashMap<u8, usize>> = Vec::new();
        for _ in 0..self.channels.len() {
            last_abs_time.push(0);
            curr_event_idx.push(0);
            last_ctrl_change_per_channel.push(None);
            last_prog_change_per_channel.push(None);
            last_pitch_bend_per_channel.push(None);
            active_notes.push(HashMap::new());
        }
        for voice in 0..self.voices.len() {
            last_ctrl_change_per_voice.push(None);
            last_prog_change_per_voice.push(None);
            last_pitch_bend_per_voice.push(None);
            last_channel_per_voice.push(
                self.channels
                    .iter()
                    .find_position(|ch| ch.base_voice == voice)
                    .map(|(idx, _)| idx),
            );
        }
        let mut channels_done = channels
            .iter()
            .filter(|channel| channel.messages.is_empty())
            .count();
        while channels_done < channels.len() {
            let (next_channel, _) = channels
                .iter()
                .enumerate()
                .min_by_key(|&(i, channel)| {
                    if curr_event_idx[i] == channel.messages.len() {
                        (u32::max_value(), u8::max_value())
                    } else {
                        let event = &channel.messages[curr_event_idx[i]];
                        (event.1, priority(&event.0))
                    }
                })
                .unwrap();
            let next_event = &channels[next_channel].messages[curr_event_idx[next_channel]];
            curr_event_idx[next_channel] += 1;
            if curr_event_idx[next_channel] == channels[next_channel].messages.len() {
                channels_done += 1;
            }

            match *next_event {
                (Message::MetaEvent { ref event, .. }, _) => {
                    if let MetaEvent::SetTempo = *event {
                        self.voices[0].messages.push(next_event.clone());
                    }
                }
                (
                    Message::MidiEvent {
                        delta_time,
                        ref event,
                    },
                    abs_time,
                ) => match *event {
                    MidiEvent::NoteOff { ch, note, .. } => {
                        let ch = ch as usize;
                        if active_notes[ch].contains_key(&note) {
                            let voice = active_notes[ch].remove(&note).unwrap();
                            self.voices[voice].messages.push(next_event.clone());
                        }
                    }
                    MidiEvent::NoteOn { ch, note, velocity } => {
                        let ch = ch as usize;
                        if active_notes[ch].contains_key(&note) {
                            let note_voice = active_notes[ch].remove(&note).unwrap();
                            self.voices[note_voice].messages.push((
                                Message::MidiEvent {
                                    delta_time,
                                    event: MidiEvent::NoteOff {
                                        ch: ch as u8,
                                        note,
                                        velocity,
                                    },
                                },
                                abs_time,
                            ));
                        }
                        if velocity > 0 {
                            let mut next_voice = None;
                            let base_voice = self.channels[ch].base_voice;
                            if active_notes
                                .iter()
                                .flat_map(|active_note| active_note.values())
                                .any(|&voice| voice == base_voice)
                            {
                                if verbose {
                                    println!("channel {} base voice occupied at {} - base voices {:?} active notes {:?}",
                                             ch,
                                             abs_time,
                                             self.channels.iter().map(|channel| channel.base_voice).collect::<Vec<usize>>(),
                                             active_notes);
                                }
                                for possible_voice in 0..self.voices.len() {
                                    if !self
                                        .channels
                                        .iter()
                                        .map(|channel| channel.base_voice)
                                        .any(|voice| voice == possible_voice)
                                        && !active_notes
                                            .iter()
                                            .flat_map(|active_note| active_note.values())
                                            .any(|&voice| voice == possible_voice)
                                    {
                                        if verbose {
                                            println!("using {}", possible_voice);
                                        }
                                        next_voice = Some(possible_voice);
                                        break;
                                    }
                                }
                            } else {
                                next_voice = Some(base_voice);
                            }
                            let next_voice = next_voice.ok_or(MidiError::CouldntFit {
                                path: path.to_str().unwrap().to_owned(),
                            })?;

                            active_notes[ch].insert(note, next_voice);
                            let messages = &mut self.voices[next_voice].messages;
                            if ch != last_channel_per_voice[next_voice].unwrap_or(0xff) {
                                last_ctrl_change_per_channel[ch]
                                    .map(|event| messages.push((event.clone(), abs_time)));
                                last_prog_change_per_channel[ch]
                                    .map(|event| messages.push((event.clone(), abs_time)));
                                last_pitch_bend_per_channel[ch]
                                    .map(|event| messages.push((event.clone(), abs_time)));
                                last_channel_per_voice[next_voice] = Some(ch);
                            };
                            messages.push(next_event.clone());
                        }
                    }
                    MidiEvent::PolyphonicKeyPressure { ch, note, .. } => {
                        let ch = ch as usize;
                        match active_notes[ch].get(&note) {
                            Some(voice) => self.voices[*voice].messages.push(next_event.clone()),
                            None => (),
                        }
                    }
                    MidiEvent::ControlChange { ch, control, .. } => {
                        let ch = ch as usize;
                        let mut pushed_to_base = false;
                        for &voice in active_notes[ch].values() {
                            self.voices[voice].messages.push(next_event.clone());
                            if control == 7 {
                                last_ctrl_change_per_voice[voice] = Some(&next_event.0);
                            }
                            if voice == self.channels[ch].base_voice {
                                pushed_to_base = true;
                            }
                        }
                        if !pushed_to_base {
                            self.voices[self.channels[ch].base_voice]
                                .messages
                                .push(next_event.clone());
                        }
                        if control == 7 {
                            last_ctrl_change_per_channel[ch] = Some(&next_event.0);
                        }
                    }
                    MidiEvent::ProgramChange { ch, .. } => {
                        let ch = ch as usize;
                        let mut pushed_to_base = false;
                        for &voice in active_notes[ch].values() {
                            self.voices[voice].messages.push(next_event.clone());
                            last_prog_change_per_voice[voice] = Some(&next_event.0);
                            if voice == self.channels[ch].base_voice {
                                pushed_to_base = true;
                            }
                        }
                        if !pushed_to_base {
                            self.voices[self.channels[ch].base_voice]
                                .messages
                                .push(next_event.clone());
                        }
                        last_prog_change_per_channel[ch] = Some(&next_event.0);
                    }
                    MidiEvent::ChannelPressure { ch, .. } => {
                        let ch = ch as usize;
                        let mut pushed_to_base = false;
                        for &voice in active_notes[ch].values() {
                            self.voices[voice].messages.push(next_event.clone());
                            if voice == self.channels[ch].base_voice {
                                pushed_to_base = true;
                            }
                        }
                        if !pushed_to_base {
                            self.voices[self.channels[ch].base_voice]
                                .messages
                                .push(next_event.clone());
                        }
                    }
                    MidiEvent::PitchBendChange { ch, .. } => {
                        let ch = ch as usize;
                        let mut pushed_to_base = false;
                        for &voice in active_notes[ch].values() {
                            self.voices[voice].messages.push(next_event.clone());
                            last_pitch_bend_per_voice[voice] = Some(&next_event.0);
                            if voice == self.channels[ch].base_voice {
                                pushed_to_base = true;
                            }
                        }
                        if !pushed_to_base {
                            self.voices[self.channels[ch].base_voice]
                                .messages
                                .push(next_event.clone());
                        }
                        last_pitch_bend_per_channel[ch] = Some(&next_event.0);
                    }
                    _ => {}
                },
                (Message::SysExEvent { .. }, _) => {
                    self.voices[0].messages.push(next_event.clone());
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn events_for_voice(&self, voice: usize) -> &Vec<(Message, u32)> {
        &self.voices[voice].messages
    }
}

impl Handler for MidiHandler {
    fn header(&mut self, format: u16, _track: u16, time_base: u16) {
        if format != 1 {
            unimplemented!("MIDI format {}", format)
        }
        self.ticks_per_beat = time_base;
    }

    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        self.tracks
            .last_mut()
            .unwrap()
            .messages
            .push(Message::MetaEvent {
                delta_time,
                event: event.clone(),
                data: data.clone(),
            });
    }

    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        self.tracks
            .last_mut()
            .unwrap()
            .messages
            .push(Message::MidiEvent {
                delta_time,
                event: event.clone(),
            });
    }

    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        self.tracks
            .last_mut()
            .unwrap()
            .messages
            .push(Message::SysExEvent {
                delta_time,
                event: event.clone(),
                data: data.clone(),
            });
    }

    fn track_change(&mut self) {
        self.tracks.push(MidiTrack::new());
    }
}
