extern crate midi2spc;
extern crate failure;
#[macro_use]
extern crate clap;

use failure::Error;

fn main() -> Result<(), Error> {
    let matches = clap_app!(midi2spc =>
        (@setting SubcommandRequiredElseHelp)
        (@subcommand build_rom =>
            (about: "build a ROM according to a manifest file")
            (@arg MANIFEST: "the manifest file to use")
            (@arg ROM: "the ROM file to use")
        )
        (@subcommand load_rom =>
            (about: "load in existing songs from a ROM")
            (@arg ROM: "the ROM file to use")
        )
        (@subcommand all_overworld =>
            (about: "convert a MIDI or JSON file and replace all music with it")
            (@arg INPUT: "the input file to use")
            (@arg ROM: "the ROM file to use")
        )
        (@subcommand dump_midi =>
            (about: "read a MIDI file and dump it to stdout")
            (@arg INPUT: "the input file to use")
        )
        (@subcommand midi2json =>
            (about: "convert a MIDI file to NSPC commands in JSON")
            (@arg INPUT: "the input file to use")
            (@arg OUTPUT: "the output file to use")
        )
    ).get_matches();
    midi2spc::run(matches)
}
