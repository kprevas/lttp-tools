extern crate midi2spc;
#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(midi2spc =>
        (@setting SubcommandRequiredElseHelp)
        (@subcommand all_overworld_midi =>
            (about: "convert a MIDI file and replace all overworld music with it")
            (@arg INPUT: "the input file to use")
            (@arg ROM: "the ROM file to use")
        )
        (@subcommand all_overworld_json =>
            (about: "read a song from JSON and replace all overworld music with it")
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
    midi2spc::run(matches);
}
