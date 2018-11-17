extern crate byteorder;
extern crate clap;
extern crate ghakuf;

use std::path::{Path};

pub mod midi;
pub mod nspc;
pub mod rom;

pub fn run(matches: clap::ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("insert_title_song") {
        let input_path = matches.value_of("INPUT");
        let rom_path = matches.value_of("ROM");
        let mut midi = midi::MidiHandler::new();
        midi.read(Path::new(input_path.unwrap()));
        let song = nspc::Song::from_midi(&midi);
        rom::Rom::write_all_base_songs_as(&song, Path::new(rom_path.unwrap()));
    } else if let Some(matches) = matches.subcommand_matches("dump_midi") {
        let input_path = matches.value_of("INPUT");
        let mut midi = midi::MidiHandler::new();
        midi.read(Path::new(input_path.unwrap()));
        println!("{:#?}", midi);
    }
}