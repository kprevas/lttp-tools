extern crate bimap;
#[macro_use]
extern crate clap;
extern crate itertools;
extern crate regex;
extern crate serde_json;
extern crate simple_error;
extern crate textwrap;

use bimap::BiMap;
use clap::ArgMatches;
use itertools::Itertools;
use regex::Regex;
use serde_json::Value;
use simple_error::SimpleError;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{stdout, BufWriter, Read, Write};

const DEFAULT_ADDR: &str = "1C8000";
const DEFAULT_ROM_ADDR: usize = 0xE0000;
const DEFAULT_MODULE: &str = "text";
const DEFAULT_LABEL: &str = "data";

const LINE_WIDTH: usize = 14;

fn parse_hex_arg(arg_matches: &ArgMatches, arg: &str, default: usize) -> Result<usize, Box<Error>> {
    Ok(arg_matches.value_of(arg).map_or(Ok(default), |arg| {
        usize::from_str_radix(arg.trim_start_matches("0x"), 16)
    })?)
}

fn char_map_length(first: u8, next: u8) -> usize {
    match first {
        0xFE => match next {
            0x6C => 3,
            _ => 2,
        },
        0xD2 => 2,
        0xE5 => 2,
        0xE6 => 2,
        0xE8 => 2,
        0xEA => 2,
        _ => 1,
    }
}

fn char_map() -> BiMap<char, Vec<u8>> {
    let mut map = BiMap::new();
    map.insert(' ', vec![0xFF]);
    map.insert('?', vec![0xC6]);
    map.insert('!', vec![0xC7]);
    map.insert(',', vec![0xC8]);
    map.insert('-', vec![0xC9]);
    map.insert('…', vec![0xCC]);
    map.insert('.', vec![0xCD]);
    map.insert('~', vec![0xCE]);
    map.insert('\'', vec![0xD8]);
    map.insert('@', vec![0xFE, 0x6A]); // link's name compressed
    map.insert('>', vec![0xD2, 0xD3]); // link face
    map.insert('%', vec![0xDD]); // Hylian Bird
    map.insert('^', vec![0xDE]); // Hylian Ankh
    map.insert('=', vec![0xDF]); // Hylian Wavy lines
    map.insert('↑', vec![0xE0]);
    map.insert('↓', vec![0xE1]);
    map.insert('→', vec![0xE2]);
    map.insert('←', vec![0xE3]);
    map.insert('≥', vec![0xE4]); // cursor
    map.insert('¼', vec![0xE5, 0xE7]); // ¼ heart
    map.insert('½', vec![0xE6, 0xE7]); // ½ heart
    map.insert('¾', vec![0xE8, 0xE9]); // ¾ heart
    map.insert('♥', vec![0xEA, 0xEB]); // full heart
    map.insert('ᚋ', vec![0xFE, 0x6C, 0x00]); // var 0
    map.insert('ᚌ', vec![0xFE, 0x6C, 0x01]); // var 1
    map.insert('ᚍ', vec![0xFE, 0x6C, 0x02]); // var 2
    map.insert('ᚎ', vec![0xFE, 0x6C, 0x03]); // var 3
    map.insert('あ', vec![0x00]);
    map.insert('い', vec![0x01]);
    map.insert('う', vec![0x02]);
    map.insert('え', vec![0x03]);
    map.insert('お', vec![0x04]);
    map.insert('や', vec![0x05]);
    map.insert('ゆ', vec![0x06]);
    map.insert('よ', vec![0x07]);
    map.insert('か', vec![0x08]);
    map.insert('き', vec![0x09]);
    map.insert('く', vec![0x0A]);
    map.insert('け', vec![0x0B]);
    map.insert('こ', vec![0x0C]);
    map.insert('わ', vec![0x0D]);
    map.insert('を', vec![0x0E]);
    map.insert('ん', vec![0x0F]);
    map.insert('さ', vec![0x10]);
    map.insert('し', vec![0x11]);
    map.insert('す', vec![0x12]);
    map.insert('せ', vec![0x13]);
    map.insert('そ', vec![0x14]);
    map.insert('が', vec![0x15]);
    map.insert('ぎ', vec![0x16]);
    map.insert('ぐ', vec![0x17]);
    map.insert('た', vec![0x18]);
    map.insert('ち', vec![0x19]);
    map.insert('つ', vec![0x1A]);
    map.insert('て', vec![0x1B]);
    map.insert('と', vec![0x1C]);
    map.insert('げ', vec![0x1D]);
    map.insert('ご', vec![0x1E]);
    map.insert('ざ', vec![0x1F]);
    map.insert('な', vec![0x20]);
    map.insert('に', vec![0x21]);
    map.insert('ぬ', vec![0x22]);
    map.insert('ね', vec![0x23]);
    map.insert('の', vec![0x24]);
    map.insert('じ', vec![0x25]);
    map.insert('ず', vec![0x26]);
    map.insert('ぜ', vec![0x27]);
    map.insert('は', vec![0x28]);
    map.insert('ひ', vec![0x29]);
    map.insert('ふ', vec![0x2A]);
    map.insert('へ', vec![0x2B]);
    map.insert('ほ', vec![0x2C]);
    map.insert('ぞ', vec![0x2D]);
    map.insert('だ', vec![0x2E]);
    map.insert('ぢ', vec![0x2F]);
    map.insert('ま', vec![0x30]);
    map.insert('み', vec![0x31]);
    map.insert('む', vec![0x32]);
    map.insert('め', vec![0x33]);
    map.insert('も', vec![0x34]);
    map.insert('づ', vec![0x35]);
    map.insert('で', vec![0x36]);
    map.insert('ど', vec![0x37]);
    map.insert('ら', vec![0x38]);
    map.insert('り', vec![0x39]);
    map.insert('る', vec![0x3A]);
    map.insert('れ', vec![0x3B]);
    map.insert('ろ', vec![0x3C]);
    map.insert('ば', vec![0x3D]);
    map.insert('び', vec![0x3E]);
    map.insert('ぶ', vec![0x3F]);
    map.insert('べ', vec![0x40]);
    map.insert('ぼ', vec![0x41]);
    map.insert('ぱ', vec![0x42]);
    map.insert('ぴ', vec![0x43]);
    map.insert('ぷ', vec![0x44]);
    map.insert('ぺ', vec![0x45]);
    map.insert('ぽ', vec![0x46]);
    map.insert('ゃ', vec![0x47]);
    map.insert('ゅ', vec![0x48]);
    map.insert('ょ', vec![0x49]);
    map.insert('っ', vec![0x4A]);
    map.insert('ぁ', vec![0x4B]);
    map.insert('ぃ', vec![0x4C]);
    map.insert('ぅ', vec![0x4D]);
    map.insert('ぇ', vec![0x4E]);
    map.insert('ぉ', vec![0x4F]);
    map.insert('ア', vec![0x50]);
    map.insert('イ', vec![0x51]);
    map.insert('ウ', vec![0x52]);
    map.insert('エ', vec![0x53]);
    map.insert('オ', vec![0x54]);
    map.insert('ヤ', vec![0x55]);
    map.insert('ユ', vec![0x56]);
    map.insert('ヨ', vec![0x57]);
    map.insert('カ', vec![0x58]);
    map.insert('キ', vec![0x59]);
    map.insert('ク', vec![0x5A]);
    map.insert('ケ', vec![0x5B]);
    map.insert('コ', vec![0x5C]);
    map.insert('ワ', vec![0x5D]);
    map.insert('ヲ', vec![0x5E]);
    map.insert('ン', vec![0x5F]);
    map.insert('サ', vec![0x60]);
    map.insert('シ', vec![0x61]);
    map.insert('ス', vec![0x62]);
    map.insert('セ', vec![0x63]);
    map.insert('ソ', vec![0x64]);
    map.insert('ガ', vec![0x65]);
    map.insert('ギ', vec![0x66]);
    map.insert('グ', vec![0x67]);
    map.insert('タ', vec![0x68]);
    map.insert('チ', vec![0x69]);
    map.insert('ツ', vec![0x6A]);
    map.insert('テ', vec![0x6B]);
    map.insert('ト', vec![0x6C]);
    map.insert('ゲ', vec![0x6D]);
    map.insert('ゴ', vec![0x6E]);
    map.insert('ザ', vec![0x6F]);
    map.insert('ナ', vec![0x70]);
    map.insert('ニ', vec![0x71]);
    map.insert('ヌ', vec![0x72]);
    map.insert('ネ', vec![0x73]);
    map.insert('ノ', vec![0x74]);
    map.insert('ジ', vec![0x75]);
    map.insert('ズ', vec![0x76]);
    map.insert('ゼ', vec![0x77]);
    map.insert('ハ', vec![0x78]);
    map.insert('ヒ', vec![0x79]);
    map.insert('フ', vec![0x7A]);
    map.insert('ヘ', vec![0x7B]);
    map.insert('ホ', vec![0x7C]);
    map.insert('ゾ', vec![0x7D]);
    map.insert('ダ', vec![0x7E]);
    map.insert('マ', vec![0x80]);
    map.insert('ミ', vec![0x81]);
    map.insert('ム', vec![0x82]);
    map.insert('メ', vec![0x83]);
    map.insert('モ', vec![0x84]);
    map.insert('ヅ', vec![0x85]);
    map.insert('デ', vec![0x86]);
    map.insert('ド', vec![0x87]);
    map.insert('ラ', vec![0x88]);
    map.insert('リ', vec![0x89]);
    map.insert('ル', vec![0x8A]);
    map.insert('レ', vec![0x8B]);
    map.insert('ロ', vec![0x8C]);
    map.insert('バ', vec![0x8D]);
    map.insert('ビ', vec![0x8E]);
    map.insert('ブ', vec![0x8F]);
    map.insert('ベ', vec![0x90]);
    map.insert('ボ', vec![0x91]);
    map.insert('パ', vec![0x92]);
    map.insert('ピ', vec![0x93]);
    map.insert('プ', vec![0x94]);
    map.insert('ペ', vec![0x95]);
    map.insert('ポ', vec![0x96]);
    map.insert('ャ', vec![0x97]);
    map.insert('ュ', vec![0x98]);
    map.insert('ョ', vec![0x99]);
    map.insert('ッ', vec![0x9A]);
    map.insert('ァ', vec![0x9B]);
    map.insert('ィ', vec![0x9C]);
    map.insert('ゥ', vec![0x9D]);
    map.insert('ェ', vec![0x9E]);
    map.insert('ォ', vec![0x9F]);
    map
}

fn directive_length(first: u8, next: &[u8]) -> usize {
    match first {
        0xFA => 1,
        0xF7 => 1,
        0xFE => match next[0] {
            0x67 => {
                if next[1] == 0xFE && next[2] == 0x67 {
                    4
                } else {
                    2
                }
            }
            0x68 => 2,
            0x69 => 2,
            0x71 => 2,
            0x72 => 2,
            _ => 3,
        },
        _ => 2,
    }
}

fn directives<'a>() -> BiMap<&'a str, Vec<u8>> {
    let mut map = BiMap::new();
    map.insert("SPEED0", vec![0xFC, 0x00]);
    map.insert("SPEED2", vec![0xFC, 0x02]);
    map.insert("SPEED3", vec![0xFC, 0x03]);
    map.insert("SPEED6", vec![0xFC, 0x06]);
    map.insert("SCROLLSPEED0", vec![0xFE, 0x6E, 0x00]);
    map.insert("PAUSE1", vec![0xFE, 0x78, 0x01]);
    map.insert("PAUSE3", vec![0xFE, 0x78, 0x03]);
    map.insert("PAUSE5", vec![0xFE, 0x78, 0x05]);
    map.insert("PAUSE7", vec![0xFE, 0x78, 0x07]);
    map.insert("PAUSE9", vec![0xFE, 0x78, 0x09]);
    map.insert("CHOICE", vec![0xFE, 0x68]);
    map.insert("ITEMSELECT", vec![0xFE, 0x69]);
    map.insert("CHOICE2", vec![0xFE, 0x71]);
    map.insert("CHOICE3", vec![0xFE, 0x72]);
    map.insert("C:GREEN", vec![0xFE, 0x77, 0x07]);
    map.insert("C:YELLOW", vec![0xFE, 0x77, 0x02]);
    map.insert("HARP", vec![0xFE, 0x79, 0x2D]);
    map.insert("MENU", vec![0xFE, 0x6D, 0x00]);
    map.insert("BOTTOM", vec![0xFE, 0x6D, 0x01]);
    map.insert("NOBORDER", vec![0xFE, 0x6B, 0x02]);
    map.insert("NOWINDOW", vec![0xFE, 0x6B, 0x04]);
    map.insert("CHANGEPIC", vec![0xFE, 0x67, 0xFE, 0x67]);
    map.insert("CHANGEMUSIC", vec![0xFE, 0x67]);
    map.insert(
        "INTRO",
        vec![
            0xFE, 0x6E, 0x00, 0xFE, 0x77, 0x07, 0xFC, 0x03, 0xFE, 0x6B, 0x02, 0xFE, 0x67,
        ],
    );
    map.insert("NOTEXT", vec![0xFE, 0x6E, 0x00, 0xFE, 0x6B, 0x04]);
    map.insert(
        "IBOX",
        vec![0xFE, 0x6B, 0x02, 0xFE, 0x77, 0x07, 0xFC, 0x03, 0xF7],
    );
    map.insert("IBOX_ENDBYTE", vec![0xF7]);
    map
}

fn wrap_line(line: &str) -> Vec<String> {
    let re = Regex::new("\\{[^}]*}").unwrap();
    let directives = re.find_iter(line).collect_vec();
    let zero_width_directives = re.replace_all(line, "\u{200B}");
    let wrapped = textwrap::wrap(&zero_width_directives, LINE_WIDTH);
    let mut wrapped_replaced = vec![];
    let mut directive_idx = 0;
    for line in wrapped {
        let mut replaced = line.to_string();
        let zero_width_re = Regex::new("\u{200B}").unwrap();
        let mut has_match = zero_width_re.is_match(&replaced);
        while has_match {
            let replaced_old = replaced.clone();
            let zero_width_match = zero_width_re.find(&replaced_old).unwrap();
            replaced.replace_range(
                zero_width_match.start()..zero_width_match.end(),
                &directives[directive_idx].as_str(),
            );
            directive_idx += 1;
            has_match = zero_width_re.is_match(&replaced);
        }
        wrapped_replaced.push(replaced);
    }
    wrapped_replaced
}

fn txt_to_asm(matches: &ArgMatches) -> Result<(), Box<Error>> {
    let infile = matches.value_of("infile").unwrap();
    let reader = File::open(infile)?;
    let json: Value = serde_json::from_reader(reader)?;
    let array = json.as_array();
    if array.is_none() {
        return Err(Box::from(SimpleError::new(format!(
            "Failed to parse {}",
            infile
        ))));
    }
    let array = array.unwrap();

    let mut writer = BufWriter::new(File::create(matches.value_of("outfile").unwrap())?);
    writeln!(
        &mut writer,
        "        .module {}",
        matches.value_of("asm_module").unwrap_or(DEFAULT_MODULE)
    )?;
    writeln!(&mut writer)?;
    writeln!(
        &mut writer,
        "        .org ${}",
        matches.value_of("asm_addr").unwrap_or(DEFAULT_ADDR)
    )?;
    writeln!(&mut writer)?;
    writeln!(
        &mut writer,
        "{}:",
        matches.value_of("asm_label").unwrap_or(DEFAULT_LABEL)
    )?;

    let char_map = char_map();
    let directives = directives();

    for entry in array {
        let entry_obj = entry.as_object();
        if entry_obj.is_none() {
            return Err(Box::from(SimpleError::new(format!(
                "Failed to parse {}",
                infile
            ))));
        }
        let entry_obj = entry_obj.unwrap();
        if let Some(label) = entry_obj.get("asmLabel").and_then(|value| value.as_str()) {
            writeln!(&mut writer, "{}:", label)?;
        }
        let pause = entry_obj
            .get("pause")
            .map_or(true, |v| v.as_bool().unwrap_or(true));
        let lines = entry_obj.get("lines").and_then(|value| value.as_array());
        if lines.is_none() || lines.unwrap().is_empty() {
            return Err(Box::from(SimpleError::new(format!(
                "Failed to parse {}",
                infile
            ))));
        }
        let lines = lines.unwrap();
        // Check values first so we can just unwrap() later.
        lines.iter().fold(Ok(()), |r: Result<(), Box<SimpleError>>, v| {
            if r.is_err() {
                r
            } else if v.as_str().is_none() {
                Err(Box::from(SimpleError::new(format!(
                    "Failed to parse {}",
                    infile
                ))))
            } else {
                Ok(())
            }
        })?;
        let lines = lines
            .iter()
            .flat_map(|line| wrap_line(line.as_str().unwrap()))
            .collect_vec();
        let mut line_num = 0;
        let line_count;
        if lines.last().unwrap_or(&"".to_string()).starts_with("{") {
            line_count = lines.len() - 1;
        } else {
            line_count = lines.len();
        }
        let mut bytes = vec![0xFBu8];
        for line in lines {
            if pause && line_num > 0 && line_num % 3 == 0 && line_num < line_count {
                bytes.push(0xFA);
            }
            if line_num == 1 {
                bytes.push(0xF8);
            } else if line_num != 0 {
                if line_num >= 3 && line_num < line_count {
                    bytes.push(0xF6);
                } else {
                    bytes.push(0xF9);
                }
            }
            let mut chars = line.chars();
            let mut next_char = chars.next();
            while let Some(c) = next_char {
                if c == '{' {
                    let directive = chars.by_ref().take_while(|c| *c != '}').collect::<String>();
                    if let Some(directive_bytes) = directives.get_by_left(&directive.as_str()) {
                        bytes.extend(directive_bytes.iter());
                    } else {
                        return Err(Box::from(SimpleError::new(format!(
                            "Text {} contained illegal directive {}",
                            line, directive
                        ))));
                    }
                } else if let Some(digit) = c.to_digit(10) {
                    bytes.push(0xA0 + digit as u8);
                } else if c.is_ascii_alphabetic() {
                    bytes.push(0xAA + ((c.to_ascii_uppercase() as u8) - ('A' as u8)));
                } else if let Some(char_bytes) = char_map.get_by_left(&c) {
                    bytes.extend(char_bytes.iter());
                } else {
                    return Err(Box::from(SimpleError::new(format!(
                        "Text {} contained illegal character {}",
                        line, c
                    ))));
                }
                next_char = chars.next();
            }
            line_num += 1;
        }
        writeln!(
            &mut writer,
            "        .db {}",
            bytes
                .iter()
                .map(|byte| format!("${:02X}", byte))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
    }
    writeln!(&mut writer, "        .db $FF, $FF")?;
    Ok(())
}

fn dump_rom(matches: &ArgMatches) -> Result<(), Box<Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(matches.value_of("romfile").unwrap())?;
    let mut romdata = Vec::new();
    file.read_to_end(&mut romdata)?;

    let writer: Box<Write> = match matches.value_of("outfile") {
        Some(outfile) => Box::new(File::create(outfile)?),
        _ => Box::new(stdout()),
    };
    let mut writer = BufWriter::new(writer);

    let char_map = char_map();
    let directives = directives();

    writeln!(&mut writer, "[")?;

    let mut i = parse_hex_arg(matches, "rom_addr", DEFAULT_ROM_ADDR)?;
    assert_eq!(0xFB, romdata[i]);
    i += 1;
    while romdata[i] != 0x80 && romdata[i] != 0xFF {
        writeln!(&mut writer, "  {{")?;
        writeln!(&mut writer, "    \"lines\": [")?;
        write!(&mut writer, "      \"")?;
        while romdata[i] != 0xFB {
            if romdata[i] >= 0xA0 && romdata[i] <= 0xA9 {
                write!(&mut writer, "{}", romdata[i] - 0xA0)?;
            } else if romdata[i] >= 0xAA && romdata[i] <= 0xC3 {
                write!(
                    &mut writer,
                    "{}",
                    (('A' as u8) + (romdata[i] - 0xAA)) as char
                )?;
            } else {
                let char_map_length = char_map_length(romdata[i], romdata[i + 1]);
                if let Some(c) = char_map.get_by_right(&romdata[i..i + char_map_length].to_vec()) {
                    write!(&mut writer, "{}", c)?;
                } else {
                    let directive_length = directive_length(romdata[i], &romdata[i + 1..i + 4]);
                    if let Some(directive) =
                        directives.get_by_right(&romdata[i..i + directive_length].to_vec())
                    {
                        write!(&mut writer, "{{{}}}", directive)?;
                        i += directive_length - 1;
                    } else {
                        if romdata[i] == 0xF6 || romdata[i] == 0xF8 || romdata[i] == 0xF9 {
                            write!(&mut writer, "\",\n      \"")?;
                        }
                    }
                }
            }
            i += 1;
        }
        writeln!(&mut writer, "\"")?;
        writeln!(&mut writer, "    ]")?;
        write!(&mut writer, "  }}")?;
        i += 1;
        if romdata[i] != 0x80 && romdata[i] != 0xFF {
            writeln!(&mut writer, ",")?;
        } else {
            writeln!(&mut writer)?;
        }
    }

    writeln!(&mut writer, "]")?;

    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    let matches = clap_app!(lttp_tileconvert =>
        (@subcommand txt_to_asm =>
            (@arg infile: +required "input JSON file")
            (@arg outfile: +required "output ASM file")
            (@arg asm_module: --asm_module +takes_value "module name to use for the ASM file")
            (@arg asm_label: --asm_label +takes_value "label prefix to use for the data in the ASM file")
            (@arg asm_addr: --asm_addr +takes_value "hex address in SNES address space where text table should go")
        )
        (@subcommand dump_rom =>
            (@arg romfile: +required "input ROM file")
            (@arg outfile: "output JSON file")
            (@arg rom_addr: --rom_addr +takes_value "hex address in ROM space where the text bank is located")
        )
    ).get_matches();

    if let Some(txt_to_asm_matches) = matches.subcommand_matches("txt_to_asm") {
        txt_to_asm(txt_to_asm_matches)?;
    } else if let Some(dump_rom_matches) = matches.subcommand_matches("dump_rom") {
        dump_rom(dump_rom_matches)?;
    }

    Ok(())
}
