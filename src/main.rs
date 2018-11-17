extern crate midi2spc;
#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(myapp =>
        (@subcommand insert_title_song =>
            (about: "convert a MIDI file and insert it as the title music")
            (@arg INPUT: "the input file to use")
            (@arg ROM: "the ROM file to use")
        )
        (@subcommand dump_midi =>
            (about: "read a MIDI file and dump it to stdout")
            (@arg INPUT: "the input file to use")
        )
        (@subcommand dump_nspc =>
            (about: "convert a MIDI file and dump the NSPC commands")
            (@arg INPUT: "the input file to use")
        )
    ).get_matches();
    midi2spc::run(matches);
}
