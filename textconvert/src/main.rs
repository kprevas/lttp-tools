#[macro_use]
extern crate clap;
extern crate itertools;
#[macro_use]
extern crate maplit;
extern crate serde_json;
extern crate simple_error;
extern crate textwrap;

use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

const DEFAULT_ADDR: &str = "1C8000";
const DEFAULT_MODULE: &str = "text";
const DEFAULT_LABEL: &str = "data";

const LINE_WIDTH: usize = 14;

fn char_map() -> HashMap<char, Vec<u8>> {
    hashmap!(
            ' ' => vec![0xFF],
            '?' => vec![0xC6],
            '!' => vec![0xC7],
            ',' => vec![0xC8],
            '-' => vec![0xC9],
            '…' => vec![0xCC],
            '.' => vec![0xCD],
            '~' => vec![0xCE],
            '～' => vec![0xCE],
            '\'' => vec![0xD8],
            '’' => vec![0xD8],
            '@' => vec![0xFE, 0x6A], // link's name compressed
            '>' => vec![0xD2, 0xD3], // link face
            '%' => vec![0xDD], // Hylian Bird
            '^' => vec![0xDE], // Hylian Ankh
            '=' => vec![0xDF], // Hylian Wavy lines
            '↑' => vec![0xE0],
            '↓' => vec![0xE1],
            '→' => vec![0xE2],
            '←' => vec![0xE3],
            '≥' => vec![0xE4], // cursor
            '¼' => vec![0xE5, 0xE7], // ¼ heart
            '½' => vec![0xE6, 0xE7], // ½ heart
            '¾' => vec![0xE8, 0xE9], // ¾ heart
            '♥' => vec![0xEA, 0xEB], // full heart
            'ᚋ' => vec![0xFE, 0x6C, 0x00], // var 0
            'ᚌ' => vec![0xFE, 0x6C, 0x01], // var 1
            'ᚍ' => vec![0xFE, 0x6C, 0x02], // var 2
            'ᚎ' => vec![0xFE, 0x6C, 0x03], // var 3
            'あ' => vec![0x00],
            'い' => vec![0x01],
            'う' => vec![0x02],
            'え' => vec![0x03],
            'お' => vec![0x04],
            'や' => vec![0x05],
            'ゆ' => vec![0x06],
            'よ' => vec![0x07],
            'か' => vec![0x08],
            'き' => vec![0x09],
            'く' => vec![0x0A],
            'け' => vec![0x0B],
            'こ' => vec![0x0C],
            'わ' => vec![0x0D],
            'を' => vec![0x0E],
            'ん' => vec![0x0F],
            'さ' => vec![0x10],
            'し' => vec![0x11],
            'す' => vec![0x12],
            'せ' => vec![0x13],
            'そ' => vec![0x14],
            'が' => vec![0x15],
            'ぎ' => vec![0x16],
            'ぐ' => vec![0x17],
            'た' => vec![0x18],
            'ち' => vec![0x19],
            'つ' => vec![0x1A],
            'て' => vec![0x1B],
            'と' => vec![0x1C],
            'げ' => vec![0x1D],
            'ご' => vec![0x1E],
            'ざ' => vec![0x1F],
            'な' => vec![0x20],
            'に' => vec![0x21],
            'ぬ' => vec![0x22],
            'ね' => vec![0x23],
            'の' => vec![0x24],
            'じ' => vec![0x25],
            'ず' => vec![0x26],
            'ぜ' => vec![0x27],
            'は' => vec![0x28],
            'ひ' => vec![0x29],
            'ふ' => vec![0x2A],
            'へ' => vec![0x2B],
            'ほ' => vec![0x2C],
            'ぞ' => vec![0x2D],
            'だ' => vec![0x2E],
            'ぢ' => vec![0x2F],
            'ま' => vec![0x30],
            'み' => vec![0x31],
            'む' => vec![0x32],
            'め' => vec![0x33],
            'も' => vec![0x34],
            'づ' => vec![0x35],
            'で' => vec![0x36],
            'ど' => vec![0x37],
            'ら' => vec![0x38],
            'り' => vec![0x39],
            'る' => vec![0x3A],
            'れ' => vec![0x3B],
            'ろ' => vec![0x3C],
            'ば' => vec![0x3D],
            'び' => vec![0x3E],
            'ぶ' => vec![0x3F],
            'べ' => vec![0x40],
            'ぼ' => vec![0x41],
            'ぱ' => vec![0x42],
            'ぴ' => vec![0x43],
            'ぷ' => vec![0x44],
            'ぺ' => vec![0x45],
            'ぽ' => vec![0x46],
            'ゃ' => vec![0x47],
            'ゅ' => vec![0x48],
            'ょ' => vec![0x49],
            'っ' => vec![0x4A],
            'ぁ' => vec![0x4B],
            'ぃ' => vec![0x4C],
            'ぅ' => vec![0x4D],
            'ぇ' => vec![0x4E],
            'ぉ' => vec![0x4F],
            'ア' => vec![0x50],
            'イ' => vec![0x51],
            'ウ' => vec![0x52],
            'エ' => vec![0x53],
            'オ' => vec![0x54],
            'ヤ' => vec![0x55],
            'ユ' => vec![0x56],
            'ヨ' => vec![0x57],
            'カ' => vec![0x58],
            'キ' => vec![0x59],
            'ク' => vec![0x5A],
            'ケ' => vec![0x5B],
            'コ' => vec![0x5C],
            'ワ' => vec![0x5D],
            'ヲ' => vec![0x5E],
            'ン' => vec![0x5F],
            'サ' => vec![0x60],
            'シ' => vec![0x61],
            'ス' => vec![0x62],
            'セ' => vec![0x63],
            'ソ' => vec![0x64],
            'ガ' => vec![0x65],
            'ギ' => vec![0x66],
            'グ' => vec![0x67],
            'タ' => vec![0x68],
            'チ' => vec![0x69],
            'ツ' => vec![0x6A],
            'テ' => vec![0x6B],
            'ト' => vec![0x6C],
            'ゲ' => vec![0x6D],
            'ゴ' => vec![0x6E],
            'ザ' => vec![0x6F],
            'ナ' => vec![0x70],
            'ニ' => vec![0x71],
            'ヌ' => vec![0x72],
            'ネ' => vec![0x73],
            'ノ' => vec![0x74],
            'ジ' => vec![0x75],
            'ズ' => vec![0x76],
            'ゼ' => vec![0x77],
            'ハ' => vec![0x78],
            'ヒ' => vec![0x79],
            'フ' => vec![0x7A],
            'ヘ' => vec![0x7B],
            'ホ' => vec![0x7C],
            'ゾ' => vec![0x7D],
            'ダ' => vec![0x7E],
            'マ' => vec![0x80],
            'ミ' => vec![0x81],
            'ム' => vec![0x82],
            'メ' => vec![0x83],
            'モ' => vec![0x84],
            'ヅ' => vec![0x85],
            'デ' => vec![0x86],
            'ド' => vec![0x87],
            'ラ' => vec![0x88],
            'リ' => vec![0x89],
            'ル' => vec![0x8A],
            'レ' => vec![0x8B],
            'ロ' => vec![0x8C],
            'バ' => vec![0x8D],
            'ビ' => vec![0x8E],
            'ブ' => vec![0x8F],
            'ベ' => vec![0x90],
            'ボ' => vec![0x91],
            'パ' => vec![0x92],
            'ピ' => vec![0x93],
            'プ' => vec![0x94],
            'ペ' => vec![0x95],
            'ポ' => vec![0x96],
            'ャ' => vec![0x97],
            'ュ' => vec![0x98],
            'ョ' => vec![0x99],
            'ッ' => vec![0x9A],
            'ァ' => vec![0x9B],
            'ィ' => vec![0x9C],
            'ゥ' => vec![0x9D],
            'ェ' => vec![0x9E],
            'ォ' => vec![0x9F]
    )
}

fn directives<'a>() -> HashMap<&'a str, Vec<u8>> {
    hashmap!(
            "SPEED0" => vec![0xFC, 0x00],
            "SPEED2" => vec![0xFC, 0x02],
            "SPEED6" => vec![0xFC, 0x06],
            "PAUSE1" => vec![0xFE, 0x78, 0x01],
            "PAUSE3" => vec![0xFE, 0x78, 0x03],
            "PAUSE5" => vec![0xFE, 0x78, 0x05],
            "PAUSE7" => vec![0xFE, 0x78, 0x07],
            "PAUSE9" => vec![0xFE, 0x78, 0x09],
            "INPUT" => vec![0xFA],
            "CHOICE" => vec![0xFE, 0x68],
            "ITEMSELECT" => vec![0xFE, 0x69],
            "CHOICE2" => vec![0xFE, 0x71],
            "CHOICE3" => vec![0xFE, 0x72],
            "C:GREEN" => vec![0xFE, 0x77, 0x07],
            "C:YELLOW" => vec![0xFE, 0x77, 0x02],
            "HARP" => vec![0xFE, 0x79, 0x2D],
            "MENU" => vec![0xFE, 0x6D, 0x00],
            "BOTTOM" => vec![0xFE, 0x6D, 0x01],
            "NOBORDER" => vec![0xFE, 0x6B, 0x02],
            "CHANGEPIC" => vec![0xFE, 0x67, 0xFE, 0x67],
            "CHANGEMUSIC" => vec![0xFE, 0x67],
            "INTRO" => vec![0xFE, 0x6E, 0x00, 0xFE, 0x77, 0x07, 0xFC, 0x03, 0xFE, 0x6B, 0x02, 0xFE, 0x67],
            "NOTEXT" => vec![0xFB, 0xFE, 0x6E, 0x00, 0xFE, 0x6B, 0x04],
            "IBOX" => vec![0xFE, 0x6B, 0x02, 0xFE, 0x77, 0x07, 0xFC, 0x03, 0xF7]
    )
}

fn main() -> Result<(), Box<Error>> {
    let matches = clap_app!(lttp_tileconvert =>
        (@arg infile: +required "input JSON file")
        (@arg outfile: +required "output ASM file")
        (@arg asm_module: --asm_module +takes_value "module name to use for the ASM file")
        (@arg asm_label: --asm_label +takes_value "label prefix to use for the data in the ASM file")
        (@arg asm_addr: --asm_addr +takes_value "hex address in SNES address space where text table should go")
    ).get_matches();

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
        let pause = entry_obj.get("pause").map_or(true, |v| v.as_bool().unwrap_or(true));
        let lines = entry_obj.get("lines").and_then(|value| value.as_array());
        if lines.is_none() || lines.unwrap().is_empty() {
            return Err(Box::from(SimpleError::new(format!(
                "Failed to parse {}",
                infile
            ))));
        }
        let lines = lines.unwrap();
        let mut line_num = 0;
        let line_count;
        let last_line = lines.last().unwrap();
        if let Some(last_line) = last_line.as_str() {
            if last_line.starts_with("{") {
                line_count = lines.len() - 1;
            } else {
                line_count = lines.len();
            }
        } else {
            return Err(Box::from(SimpleError::new(format!(
                "Failed to parse {}",
                infile
            ))));
        }
        let mut bytes = vec![0xFBu8];
        for line in lines {
            if let Some(line) = line.as_str() {
                for wrapped_line in textwrap::wrap(line, LINE_WIDTH) {
                    let mut chars = wrapped_line.chars();
                    let mut next_char = chars.next();
                    while let Some(c) = next_char {
                        if c == '{' {
                            let directive = chars.by_ref().take_while(|c| *c != '}').collect::<String>();
                            if let Some(directive_bytes) = directives.get(&directive.as_str()) {
                                bytes.extend(directive_bytes.iter());
                            } else {
                                return Err(Box::from(SimpleError::new(format!(
                                    "Text {} contained illegal directive {}",
                                    wrapped_line, directive))));
                            }
                        } else if let Some(digit) = c.to_digit(10) {
                            bytes.push(0xA0 + digit as u8);
                        } else if c.is_ascii_alphabetic() {
                            bytes.push(0xAA + (c.to_ascii_uppercase() as u8) - ('A' as u8));
                        } else if let Some(char_bytes) = char_map.get(&c) {
                            bytes.extend(char_bytes.iter());
                        } else {
                            return Err(Box::from(SimpleError::new(format!(
                                "Text {} contained illegal character {}",
                                wrapped_line, c))));
                        }
                        next_char = chars.next();
                    }
                    if line_num == 1 {
                        bytes.push(0xF8);
                    } else if line_num != 0 {
                        if line_num >=3 && line_num < line_count {
                            bytes.push(0xF6);
                        } else {
                            bytes.push(0xF9);
                        }
                    }
                    line_num += 1;
                    if pause && line_num % 3 == 0 && line_num < line_count {
                        bytes.push(0xFA);
                    }
                }
            } else {
                return Err(Box::from(SimpleError::new(format!(
                    "Failed to parse {}",
                    infile
                ))));
            }
        }
        writeln!(
            &mut writer,
            "        .db {}",
            bytes.iter().map(|byte| format!("${:02X}", byte)).collect::<Vec<String>>().join(", ")
        )?;
    }

    Ok(())
}
