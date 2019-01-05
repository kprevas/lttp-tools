extern crate byteorder;
extern crate clap;
extern crate ghakuf;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::path::{Path};

pub mod midi;
pub mod nspc;
pub mod rom;
pub mod manifest;

pub fn run(matches: clap::ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("build_rom") {
        let manifest_path = matches.value_of("MANIFEST");
        let rom_path = matches.value_of("ROM");
        let manifest = manifest::Manifest::new(Path::new(manifest_path.unwrap()));
        println!("{:#?}", manifest);
    } else if let Some(matches) = matches.subcommand_matches("all_overworld_midi") {
        let input_path = matches.value_of("INPUT");
        let rom_path = matches.value_of("ROM");
        let mut midi = midi::MidiHandler::new();
        midi.read(Path::new(input_path.unwrap()));
        let song = nspc::Song::from_midi(&midi);
        rom::Rom::write_all_base_songs_as(&song, Path::new(rom_path.unwrap()));
    } else if let Some(matches) = matches.subcommand_matches("all_overworld_json") {
        let input_path = matches.value_of("INPUT");
        let rom_path = matches.value_of("ROM");
        let song = nspc::Song::from_json(Path::new(input_path.unwrap()));
        rom::Rom::write_all_base_songs_as(&song, Path::new(rom_path.unwrap()));
    } else if let Some(matches) = matches.subcommand_matches("dump_midi") {
        let input_path = matches.value_of("INPUT");
        let mut midi = midi::MidiHandler::new();
        midi.read(Path::new(input_path.unwrap()));
        println!("{:#?}", midi);
    } else if let Some(matches) = matches.subcommand_matches("midi2json") {
        let input_path = matches.value_of("INPUT");
        let output_path = matches.value_of("OUTPUT");
        let mut midi = midi::MidiHandler::new();
        midi.read(Path::new(input_path.unwrap()));
        let song = nspc::Song::from_midi(&midi);
        song.write_to_json(Path::new(output_path.unwrap()));
    }
}