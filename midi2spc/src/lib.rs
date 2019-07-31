extern crate byteorder;
extern crate clap;
extern crate ghakuf;
extern crate itertools;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use clap::ArgMatches;
use std::error::Error;
use std::num::ParseIntError;
use std::path::Path;

pub mod manifest;
pub mod midi;
pub mod nspc;
pub mod rom;

pub fn run(matches: clap::ArgMatches) -> Result<(), Box<Error>> {
    let optimize = !matches.is_present("skip_optimization");
    let verbose = matches.is_present("verbose");
    if let Some(matches) = matches.subcommand_matches("build_rom") {
        let manifest_path = matches.value_of("MANIFEST").unwrap();
        let rom_path = matches.value_of("ROM").unwrap();
        let bank_addrs = read_bank_addrs(&matches)?;
        build_rom(manifest_path, rom_path, bank_addrs, optimize, verbose)?;
    } else if let Some(matches) = matches.subcommand_matches("all_overworld") {
        let input_path = matches.value_of("INPUT").unwrap();
        let rom_path = matches.value_of("ROM").unwrap();
        let bank_addrs = read_bank_addrs(matches)?;
        write_all_overworld(input_path, rom_path, bank_addrs, optimize, verbose)?;
    } else if let Some(matches) = matches.subcommand_matches("file_select") {
        let input_path = matches.value_of("INPUT").unwrap();
        let rom_path = matches.value_of("ROM").unwrap();
        let bank_addrs = read_bank_addrs(matches)?;
        write_file_select(input_path, rom_path, bank_addrs, optimize, verbose)?;
    } else if let Some(matches) = matches.subcommand_matches("dump_midi") {
        let input_path = matches.value_of("INPUT");
        let mut midi = midi::MidiHandler::new();
        midi.read(Path::new(input_path.unwrap()), verbose)
            .unwrap_or_else(|err| {
                println!("Error reading MIDI: {:?}", err);
            });
        println!("{:#?}", midi);
    } else if let Some(matches) = matches.subcommand_matches("midi2json") {
        let input_path = matches.value_of("INPUT");
        let output_path = matches.value_of("OUTPUT");
        let mut midi = midi::MidiHandler::new();
        midi.read(Path::new(input_path.unwrap()), verbose)?;
        let song = nspc::Song::from_midi(&midi, manifest::DEFAULT_TEMPO_ADJUST, optimize, verbose)?;
        song.write_to_json(Path::new(output_path.unwrap()));
    } else if let Some(matches) = matches.subcommand_matches("gen_fake_rom") {
        let input_path = matches.value_of("INPUT");
        let output_path = matches.value_of("OUTPUT");
        rom::gen_fake_rom(
            Path::new(input_path.unwrap()),
            Path::new(output_path.unwrap()),
            read_bank_addrs(matches)?,
        )?;
    }
    Ok(())
}

pub fn build_rom(
    manifest_path: &str,
    rom_path: &str,
    bank_addrs: [u32; 3],
    optimize: bool,
    verbose: bool,
) -> Result<(), Box<Error>> {
    let manifest = manifest::Manifest::new(Path::new(manifest_path))?;
    rom::write(
        &manifest,
        Path::new(rom_path),
        bank_addrs,
        converter(optimize, verbose).as_ref(),
        verbose,
    )?;
    Ok(())
}

pub fn write_all_overworld(
    input_path: &str,
    rom_path: &str,
    bank_addrs: [u32; 3],
    optimize: bool,
    verbose: bool,
) -> Result<(), Box<Error>> {
    rom::write_all_overworld(
        Path::new(input_path),
        Path::new(rom_path),
        bank_addrs,
        converter(optimize, verbose).as_ref(),
        verbose,
    )?;
    Ok(())
}

pub fn write_file_select(
    input_path: &str,
    rom_path: &str,
    bank_addrs: [u32; 3],
    optimize: bool,
    verbose: bool,
) -> Result<(), Box<Error>> {
    rom::write_file_select(
        Path::new(input_path),
        Path::new(rom_path),
        bank_addrs,
        converter(optimize, verbose).as_ref(),
        verbose,
    )?;
    Ok(())
}

fn converter(optimize: bool, verbose: bool) -> Box<Fn(&Path, f32) -> Result<nspc::Song, Box<Error>>> {
    let converter = move |path: &Path, tempo_factor| {
        if path.extension().map_or(false, |ext| ext.eq("mid")) {
            song_from_midi(path, tempo_factor, optimize, verbose)
        } else {
            Ok(nspc::Song::from_json(path))
        }
    };
    Box::new(converter)
}

fn read_bank_addrs(matches: &ArgMatches) -> Result<[u32; 3], Box<Error>> {
    match matches.values_of("bank_addrs") {
        None => Ok(rom::DEFAULT_BANK_BASE_ADDRS),
        Some(values) => {
            let parsed = values.map(|value_str| u32::from_str_radix(value_str, 16));
            let vec: Result<Vec<u32>, ParseIntError> = parsed.collect();
            match vec {
                Ok(vec) => Ok([vec[0], vec[1], vec[2]]),
                Err(err) => Err(Box::new(err)),
            }
        }
    }
}

fn song_from_midi(
    path: &Path,
    tempo_factor: f32,
    optimize: bool,
    verbose: bool,
) -> Result<nspc::Song, Box<Error>> {
    let mut midi = midi::MidiHandler::new();
    midi.read(path, verbose)?;
    nspc::Song::from_midi(&midi, tempo_factor, optimize, verbose)
}
