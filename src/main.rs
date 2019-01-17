extern crate failure;
extern crate midi2spc;
#[macro_use]
extern crate clap;

use failure::Error;

fn main() -> Result<(), Error> {
    let matches = clap_app!(midi2spc =>
        (@setting SubcommandRequiredElseHelp)
        (@arg skip_optimization: -s --skip_optimization  "skips optimization of CallLoops")
        (@arg verbose: -v --verbose "prints debugging output")
        (@subcommand build_rom =>
            (about: "build a ROM according to a manifest file")
            (@arg MANIFEST: +required "the manifest file to use")
            (@arg ROM: +required "the ROM file to use")
            (@arg bank_addrs: --bank_addrs #{3,3} +use_delimiter "song bank addresses in the ROM")
        )
        (@subcommand load_rom =>
            (about: "load in existing songs from a ROM")
            (@arg ROM: +required "the ROM file to use")
            (@arg bank_addrs: --bank_addrs #{3,3} +use_delimiter "song bank addresses in the ROM")
        )
        (@subcommand all_overworld =>
            (about: "convert a MIDI or JSON file and replace all music with it")
            (@arg INPUT: +required "the input file to use")
            (@arg ROM: +required "the ROM file to use")
            (@arg bank_addrs: --bank_addrs #{3,3} +use_delimiter "song bank addresses in the ROM")
        )
        (@subcommand dump_midi =>
            (about: "read a MIDI file and dump it to stdout")
            (@arg INPUT: +required "the input file to use")
        )
        (@subcommand midi2json =>
            (about: "convert a MIDI file to NSPC commands in JSON")
            (@arg INPUT: +required "the input file to use")
            (@arg OUTPUT: +required "the output file to use")
        )
    )
    .get_matches();
    midi2spc::run(matches)
}
