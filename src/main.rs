#[macro_use]
extern crate clap;
extern crate prefix_tree;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate itertools;
extern crate png;
extern crate termion;

use itertools::Itertools;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::str::FromStr;

const CMD_COPY: u8 = 0;
const CMD_BYTE_REPEAT: u8 = 1;
const CMD_WORD_REPEAT: u8 = 2;
const CMD_BYTE_INCREMENT: u8 = 3;
const CMD_COPY_EXISTING: u8 = 4;
const MAX_CMD_LEN: usize = 1024;

fn snes_bytes_to_pc(bank: u8, high: u8, low: u8) -> usize {
    let snes_addr = ((bank as u32) << 16) + ((high as u32) << 8) + (low as u32);
    ((snes_addr & 0x7FFF) + ((snes_addr / 2) & 0xFF8000)) as usize
}

fn pc_to_snes_bytes(pc_addr: usize) -> [u8; 3] {
    let mut bytes = [
        (pc_addr >> 16) as u8,
        ((pc_addr & 0xFF00) >> 8) as u8,
        (pc_addr & 0xFF) as u8,
    ];
    bytes[0] *= 2;
    if bytes[1] >= 0x80 {
        bytes[0] += 1;
    } else {
        bytes[1] += 0x80;
    }
    bytes
}

fn get_gfx_address(
    i: usize,
    romdata: &Vec<u8>,
    bank_table_addr: usize,
    hi_table_addr: usize,
    lo_table_addr: usize,
) -> usize {
    snes_bytes_to_pc(
        romdata[snes_bytes_to_pc(0, romdata[bank_table_addr + 1], romdata[bank_table_addr]) + i]
            as u8,
        romdata[snes_bytes_to_pc(0, romdata[hi_table_addr + 1], romdata[hi_table_addr]) + i] as u8,
        romdata[snes_bytes_to_pc(0, romdata[lo_table_addr + 1], romdata[lo_table_addr]) + i] as u8,
    )
}

fn put_gfx_address(
    i: usize,
    romdata: &mut Vec<u8>,
    bank_table_addr: usize,
    hi_table_addr: usize,
    lo_table_addr: usize,
    addr: usize,
) {
    let bytes = pc_to_snes_bytes(addr);
    let bank_addr = snes_bytes_to_pc(0, romdata[bank_table_addr + 1], romdata[bank_table_addr]);
    let hi_addr = snes_bytes_to_pc(0, romdata[hi_table_addr + 1], romdata[hi_table_addr]);
    let lo_addr = snes_bytes_to_pc(0, romdata[lo_table_addr + 1], romdata[lo_table_addr]);
    romdata[bank_addr + i] = bytes[0];
    romdata[hi_addr + i] = bytes[1];
    romdata[lo_addr + i] = bytes[2];
}

fn decompress_sheet(
    romdata: &Vec<u8>,
    start: usize,
    max_len: usize,
    swap_copy_cmd: bool,
) -> Vec<u8> {
    info!("decompress sheet at {:6X}", start);
    let mut out = vec![];
    let max_offset = start + max_len;
    let mut offset = start;
    let mut header = romdata[offset];
    while header != 0xFF {
        let mut cmd = header >> 5;
        let mut length = (header & 0x1F) as usize;
        if cmd == 7 {
            cmd = (header >> 2) & 7;
            length = (((header & 3) as usize) << 8) + romdata[offset + 1] as usize;
            offset += 1;
        }

        length += 1;
        assert!(offset < max_offset);
        match cmd {
            CMD_COPY => {
                debug!("copy {:?}", &romdata[offset + 1..offset + length + 1]);
                out.extend_from_slice(&romdata[offset + 1..offset + length + 1]);
                offset += length + 1;
            }
            CMD_BYTE_REPEAT => {
                debug!("byte repeat {:2X} {}", romdata[offset + 1], length);
                out.extend_from_slice(
                    std::iter::repeat(romdata[offset + 1])
                        .take(length)
                        .to_owned()
                        .collect::<Vec<u8>>()
                        .as_slice(),
                );
                offset += 2;
            }
            CMD_WORD_REPEAT => {
                debug!(
                    "word repeat {:2X}{:2X} {}",
                    romdata[offset + 1],
                    romdata[offset + 2],
                    length
                );
                out.extend_from_slice(
                    &romdata[offset + 1..offset + 3]
                        .iter()
                        .cloned()
                        .cycle()
                        .take(length)
                        .collect::<Vec<u8>>()
                        .as_slice(),
                );
                offset += 3;
            }
            CMD_BYTE_INCREMENT => {
                debug!("byte increment {:2X} {}", romdata[offset + 1], length);
                for i in 0..length {
                    out.push(romdata[offset + 1] + i as u8);
                }
                offset += 2;
            }
            CMD_COPY_EXISTING => {
                let target = if swap_copy_cmd {
                    ((romdata[offset + 2] as usize) << 8) | romdata[offset + 1] as usize
                } else {
                    ((romdata[offset + 1] as usize) << 8) | romdata[offset + 2] as usize
                } as usize;
                assert!(target + length <= out.len());
                debug!(
                    "copy existing {:4X} {} {:?}",
                    target,
                    length,
                    &out[target..target + length]
                );
                let copy = out[target..target + length].to_vec();
                out.extend_from_slice(copy.as_slice());
                offset += 3;
            }
            _ => panic!("bad command {}", cmd),
        }
        header = romdata[offset]
    }
    out
}

fn create_compressed_command(cmd: u8, length: usize, args: &[u8]) -> Vec<u8> {
    let mut out = vec![];
    if length <= 32 {
        out.push((cmd << 5) + length as u8 - 1);
    } else {
        out.push((7 << 5) | (cmd << 2) | ((length - 1) >> 8) as u8);
        out.push(((length - 1) & 0xFF) as u8);
    }
    out.extend_from_slice(args);
    out
}

fn maybe_write_direct_copy(out: &mut Vec<u8>, direct_copy: &mut Vec<u8>) {
    if direct_copy.len() > 0 {
        debug!("copy {:?}", direct_copy);
        out.extend_from_slice(
            create_compressed_command(CMD_COPY, direct_copy.len(), &direct_copy).as_slice(),
        );
        direct_copy.clear();
    }
}

fn compress_sheet(sheet_data: &Vec<u8>, swap_copy_cmd: bool) -> Vec<u8> {
    info!("compress sheet");
    let mut out = vec![];

    let mut prefix_tree = prefix_tree::Tree::new();
    for start in 0..sheet_data.len() - 4 {
        for end in start + 4..std::cmp::min(sheet_data.len(), start + 100) {
            prefix_tree.insert(&sheet_data[start..end], start);
        }
    }

    let mut offset = 0;
    let mut direct_copy = vec![];
    while offset < sheet_data.len() {
        let mut bytes_used = [1, 0, 0, 0, 0];
        let cmd_size = [0, 2, 3, 2, 3];

        // try byte repeat
        let next_byte = sheet_data[offset];
        let mut byte_repeat_offset = offset;
        while byte_repeat_offset < sheet_data.len()
            && byte_repeat_offset < offset + MAX_CMD_LEN
            && sheet_data[byte_repeat_offset] == next_byte
        {
            bytes_used[CMD_BYTE_REPEAT as usize] += 1;
            byte_repeat_offset += 1;
        }

        // try word repeat
        if offset + 1 < sheet_data.len() {
            let next_word_lo = sheet_data[offset + 1];
            let mut word_repeat_offset = offset;
            while word_repeat_offset + 1 < sheet_data.len()
                && word_repeat_offset + 1 < offset + MAX_CMD_LEN
                && sheet_data[word_repeat_offset] == next_byte
                && sheet_data[word_repeat_offset + 1] == next_word_lo
            {
                bytes_used[CMD_WORD_REPEAT as usize] += 2;
                word_repeat_offset += 2;
            }
            if word_repeat_offset < sheet_data.len()
                && word_repeat_offset < offset + MAX_CMD_LEN
                && sheet_data[word_repeat_offset] == next_byte
            {
                bytes_used[CMD_WORD_REPEAT as usize] += 1;
            }
        }

        // try byte increment
        let mut byte_increment_offset = offset;
        while byte_increment_offset < sheet_data.len()
            && byte_increment_offset < offset + MAX_CMD_LEN
            && 0xFF - (byte_increment_offset - offset) as u8 > next_byte
            && sheet_data[byte_increment_offset]
                == next_byte + (byte_increment_offset - offset) as u8
        {
            bytes_used[CMD_BYTE_INCREMENT as usize] += 1;
            byte_increment_offset += 1;
        }

        // try copy existing
        let mut copy_existing_end = offset + 4;
        while copy_existing_end <= sheet_data.len()
            && copy_existing_end <= offset + MAX_CMD_LEN
            && copy_existing_end - offset < offset
            && prefix_tree
                .find(&sheet_data[offset..copy_existing_end])
                .map_or(std::usize::MAX, |node| node.value.unwrap())
                <= offset - (copy_existing_end - offset)
        {
            bytes_used[CMD_COPY_EXISTING as usize] = copy_existing_end - offset;
            copy_existing_end += 1;
        }

        let cmd = bytes_used
            .iter()
            .zip(&cmd_size)
            .map(|(bytes, cmd)| if cmd > bytes { 0 } else { bytes - cmd })
            .enumerate()
            .max_by_key(|enumerated| enumerated.1)
            .unwrap()
            .0;
        match cmd as u8 {
            CMD_COPY => {
                direct_copy.push(sheet_data[offset]);
                if direct_copy.len() == MAX_CMD_LEN {
                    debug!("copy {:?}", direct_copy);
                    out.extend_from_slice(
                        create_compressed_command(CMD_COPY, direct_copy.len(), &direct_copy)
                            .as_slice(),
                    );
                    direct_copy.clear();
                }
            }
            CMD_BYTE_REPEAT => {
                maybe_write_direct_copy(&mut out, &mut direct_copy);
                debug!(
                    "byte repeat {:2X} {}",
                    sheet_data[offset], bytes_used[CMD_BYTE_REPEAT as usize]
                );
                out.extend_from_slice(
                    create_compressed_command(
                        CMD_BYTE_REPEAT,
                        bytes_used[CMD_BYTE_REPEAT as usize],
                        &[sheet_data[offset]],
                    )
                    .as_slice(),
                );
            }
            CMD_WORD_REPEAT => {
                maybe_write_direct_copy(&mut out, &mut direct_copy);
                debug!(
                    "word repeat {:2X}{:2X} {}",
                    sheet_data[offset],
                    sheet_data[offset + 1],
                    bytes_used[CMD_WORD_REPEAT as usize]
                );
                out.extend_from_slice(
                    create_compressed_command(
                        CMD_WORD_REPEAT,
                        bytes_used[CMD_WORD_REPEAT as usize],
                        &[sheet_data[offset], sheet_data[offset + 1]],
                    )
                    .as_slice(),
                );
            }
            CMD_BYTE_INCREMENT => {
                maybe_write_direct_copy(&mut out, &mut direct_copy);
                debug!(
                    "byte increment {:2X} {}",
                    sheet_data[offset], bytes_used[CMD_BYTE_INCREMENT as usize]
                );
                out.extend_from_slice(
                    create_compressed_command(
                        CMD_BYTE_INCREMENT,
                        bytes_used[CMD_BYTE_INCREMENT as usize],
                        &[sheet_data[offset]],
                    )
                    .as_slice(),
                );
            }
            CMD_COPY_EXISTING => {
                maybe_write_direct_copy(&mut out, &mut direct_copy);
                let copy_source = prefix_tree
                    .find(
                        &sheet_data
                            [offset..offset + bytes_used[CMD_COPY_EXISTING as usize] as usize],
                    )
                    .unwrap()
                    .value
                    .unwrap();
                let copy_source_bytes = if swap_copy_cmd {
                    [(copy_source & 0xFF) as u8, (copy_source >> 8) as u8]
                } else {
                    [(copy_source >> 8) as u8, (copy_source & 0xFF) as u8]
                };
                debug!(
                    "copy existing {:4X} {}",
                    copy_source, bytes_used[CMD_COPY_EXISTING as usize]
                );
                out.extend_from_slice(
                    create_compressed_command(
                        CMD_COPY_EXISTING,
                        bytes_used[CMD_COPY_EXISTING as usize],
                        &copy_source_bytes,
                    )
                    .as_slice(),
                );
            }
            _ => {}
        }
        offset += bytes_used[cmd] as usize;
    }
    if !direct_copy.is_empty() {
        out.extend_from_slice(
            create_compressed_command(CMD_COPY, direct_copy.len(), &direct_copy).as_slice(),
        );
    }
    out.push(0xFF);
    out
}

fn bpp3_sheet_to_pixels(sheet: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; 128]; 32];
    for tile_y in 0..4 {
        for tile_x in 0..16 {
            for px_y in 0..8 {
                let line = [
                    sheet[px_y * 2 + tile_x * 24 + tile_y * 384],
                    sheet[px_y * 2 + tile_x * 24 + tile_y * 384 + 1],
                    sheet[px_y + tile_x * 24 + tile_y * 384 + 16],
                ];
                for px_x in 0..4 {
                    let mut px_hi = 0;
                    let mut px_lo = 0;
                    if line[0] & (1 << (px_x * 2)) > 0 {
                        px_hi += 1;
                    }
                    if line[1] & (1 << (px_x * 2)) > 0 {
                        px_hi += 2;
                    }
                    if line[2] & (1 << (px_x * 2)) > 0 {
                        px_hi += 4;
                    }
                    if line[0] & (1 << (px_x * 2 + 1)) > 0 {
                        px_lo += 1;
                    }
                    if line[1] & (1 << (px_x * 2 + 1)) > 0 {
                        px_lo += 2;
                    }
                    if line[2] & (1 << (px_x * 2 + 1)) > 0 {
                        px_lo += 4;
                    }
                    out[tile_y * 8 + px_y][tile_x * 8 + (7 - px_x * 2)] = px_hi;
                    out[tile_y * 8 + px_y][tile_x * 8 + (7 - (px_x * 2 + 1))] = px_lo;
                }
            }
        }
    }
    out
}

fn pixels_to_bpp3_sheet(px_data: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = vec![0; 0x600];
    for tile_y in 0..4 {
        for tile_x in 0..16 {
            for px_y in 0..8 {
                let mut line = [0u8, 0u8, 0u8];
                for px_x in 0..4 {
                    let px_hi = px_data[tile_y * 8 + px_y][tile_x * 8 + (7 - px_x * 2)];
                    let px_lo = px_data[tile_y * 8 + px_y][tile_x * 8 + (7 - (px_x * 2 + 1))];
                    if px_hi & 1 > 0 {
                        line[0] |= 1 << (px_x * 2);
                    }
                    if px_hi & 2 > 0 {
                        line[1] |= 1 << (px_x * 2);
                    }
                    if px_hi & 4 > 0 {
                        line[2] |= 1 << (px_x * 2);
                    }
                    if px_lo & 1 > 0 {
                        line[0] |= 1 << (px_x * 2 + 1);
                    }
                    if px_lo & 2 > 0 {
                        line[1] |= 1 << (px_x * 2 + 1);
                    }
                    if px_lo & 4 > 0 {
                        line[2] |= 1 << (px_x * 2 + 1);
                    }
                }
                out[px_y * 2 + tile_x * 24 + tile_y * 384] = line[0];
                out[px_y * 2 + tile_x * 24 + tile_y * 384 + 1] = line[1];
                out[px_y + tile_x * 24 + tile_y * 384 + 16] = line[2];
            }
        }
    }
    out
}

fn dump_sheet(sheet: usize, sheet_data: &Vec<Vec<u8>>, sheet_addr: usize) {
    println!("sheet {} at {:06X}:", sheet, sheet_addr);
    for line in sheet_data {
        for px in line {
            match px {
                0 => print!("{}█", termion::color::Fg(termion::color::Black)),
                1 => print!("{}█", termion::color::Fg(termion::color::Red)),
                2 => print!("{}█", termion::color::Fg(termion::color::Blue)),
                3 => print!("{}█", termion::color::Fg(termion::color::Green)),
                4 => print!("{}█", termion::color::Fg(termion::color::Yellow)),
                5 => print!("{}█", termion::color::Fg(termion::color::Magenta)),
                6 => print!("{}█", termion::color::Fg(termion::color::Cyan)),
                7 => print!("{}█", termion::color::Fg(termion::color::White)),
                _ => {}
            };
        }
        println!();
    }
}

fn dump_sheets(
    bank_table_addr: usize,
    hi_table_addr: usize,
    lo_table_addr: usize,
    romdata: &mut Vec<u8>,
    sheet_arg: Option<usize>,
) -> () {
    for sheet in 0..113 {
        if sheet_arg.is_none() || sheet_arg.unwrap() == sheet {
            let sheet_addr = get_gfx_address(
                sheet,
                &romdata,
                bank_table_addr,
                hi_table_addr,
                lo_table_addr,
            );
            let decompressed_sheet = decompress_sheet(&romdata, sheet_addr, 0x800, true);
            let sheet_data = bpp3_sheet_to_pixels(&decompressed_sheet);

            dump_sheet(sheet, &sheet_data, sheet_addr);
        }
    }
}

fn patch_tile(
    bank_table_addr: usize,
    hi_table_addr: usize,
    lo_table_addr: usize,
    mut romdata: &mut Vec<u8>,
    output_rom_path: &str,
    png_path: &str,
    sheet: usize,
    x: usize,
    y: usize,
    verify: bool,
    sheet_start: usize,
) {
    let sheet_addr = get_gfx_address(
        sheet,
        &romdata,
        bank_table_addr,
        hi_table_addr,
        lo_table_addr,
    );
    let decompressed_sheet = decompress_sheet(&romdata, sheet_addr, 0x600, true);
    let mut sheet_data = bpp3_sheet_to_pixels(&decompressed_sheet);
    let png_file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(png_path)
        .unwrap();
    let decoder = png::Decoder::new(png_file);
    let (_, mut reader) = decoder.read_info().unwrap();
    let first_row = reader.next_row().unwrap().unwrap();
    let palette: HashMap<(u8, u8, u8), usize> = first_row
        .iter()
        .tuples::<(_, _, _)>()
        .take(8)
        .enumerate()
        .map(|(a, (r, g, b))| ((*r, *g, *b), a))
        .collect();
    let mut png_y = 0;
    let mut row = reader.next_row().unwrap();
    while row.is_some() {
        let mut png_x = 0;
        for px in row
            .unwrap()
            .iter()
            .tuples::<(_, _, _)>()
            .map(|(r, g, b)| (*r, *g, *b))
        {
            sheet_data[y + png_y][x + png_x] = *palette.get(&px).unwrap() as u8;
            png_x += 1;
        }
        row = reader.next_row().unwrap();
        png_y += 1;
    }
    let out_sheet = pixels_to_bpp3_sheet(&sheet_data);
    let compressed_sheet = compress_sheet(&out_sheet, true);
    if verify {
        assert_eq!(
            sheet_data,
            bpp3_sheet_to_pixels(&decompress_sheet(&compressed_sheet, 0, 0x600, true))
        );
    };
    romdata.splice(
        sheet_start..sheet_start + compressed_sheet.len(),
        compressed_sheet.iter().cloned(),
    );
    put_gfx_address(
        sheet,
        &mut romdata,
        bank_table_addr,
        hi_table_addr,
        lo_table_addr,
        sheet_start,
    );
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(output_rom_path)
        .unwrap();
    file.write(&romdata).unwrap();
}

fn main() {
    env_logger::init();

    let matches = clap_app!(lttp_tilepatch =>
        (@arg in_ROM: +required "input ROM file")
        (@arg banktable: "Bank table address")
        (@arg hitable: "High table address")
        (@arg lotable: "Low table address")
        (@subcommand patch =>
            (about: "Patch a single PNG into a tile sheet")
            (@arg out_ROM: -o --out +required +takes_value "output ROM file")
            (@arg in_png: -p --png +required +takes_value "input PNG file")
            (@arg sheet: -s --sheet +required +takes_value "target tile sheet (0-222)")
            (@arg x: -x +required +takes_value "target X coordinate (0-127)")
            (@arg y: -y +required +takes_value "target Y coordinate (0-31)")
            (@arg verify: -v "verify data compression")
            (@arg expanded_tiles_start: --exp_start "ROM address to start new tilesheets")
            (@arg expanded_tiles_size: --exp_size "space to reserve for each tilesheet")
        )
        (@subcommand dump =>
            (about: "Dump all tile sheets to the console")
            (@arg sheet: -s --sheet +takes_value "target tile sheet (0-222)")
        )
    )
    .get_matches();

    let input_rom_path = matches.value_of("in_ROM").unwrap();
    let bank_table_addr = usize::from_str_radix(
        matches
            .value_of("banktable")
            .unwrap_or("0x6790")
            .trim_left_matches("0x"),
        16,
    )
    .unwrap();
    let hi_table_addr = usize::from_str_radix(
        matches
            .value_of("hitable")
            .unwrap_or("0x6795")
            .trim_left_matches("0x"),
        16,
    )
    .unwrap();
    let lo_table_addr = usize::from_str_radix(
        matches
            .value_of("lotable")
            .unwrap_or("0x679A")
            .trim_left_matches("0x"),
        16,
    )
    .unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(input_rom_path)
        .unwrap();
    let mut romdata = Vec::new();
    file.read_to_end(&mut romdata).unwrap();

    if let Some(patch) = matches.subcommand_matches("patch") {
        let sheet = usize::from_str(patch.value_of("sheet").unwrap()).unwrap();
        let exp_start = patch
            .value_of("expanded_tiles_start")
            .map_or(0x110000, |arg| {
                usize::from_str_radix(arg.trim_left_matches("0x"), 16).unwrap()
            });
        let exp_size = patch.value_of("expanded_tiles_size").map_or(0x1000, |arg| {
            usize::from_str_radix(arg.trim_left_matches("0x"), 16).unwrap()
        });
        let sheet_start = exp_start + (exp_size * sheet);

        patch_tile(
            bank_table_addr,
            hi_table_addr,
            lo_table_addr,
            &mut romdata,
            patch.value_of("out_ROM").unwrap(),
            patch.value_of("in_png").unwrap(),
            sheet,
            usize::from_str(patch.value_of("x").unwrap()).unwrap(),
            usize::from_str(patch.value_of("y").unwrap()).unwrap(),
            patch.is_present("verify"),
            sheet_start,
        );
    }
    if let Some(dump) = matches.subcommand_matches("dump") {
        dump_sheets(
            bank_table_addr,
            hi_table_addr,
            lo_table_addr,
            &mut romdata,
            dump.value_of("sheet")
                .map(|arg| usize::from_str(arg).unwrap()),
        )
    }
}
