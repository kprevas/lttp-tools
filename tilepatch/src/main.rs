use clap::clap_app;
use clap::ArgMatches;
use itertools::Itertools;
use log::{debug, info};
use simple_error::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

const CMD_COPY: u8 = 0;
const CMD_BYTE_REPEAT: u8 = 1;
const CMD_WORD_REPEAT: u8 = 2;
const CMD_BYTE_INCREMENT: u8 = 3;
const CMD_COPY_EXISTING: u8 = 4;
const MAX_CMD_LEN: usize = 1024;
const MAX_COPY_EXISTING_LEN: usize = 100;

const DEFAULT_EXPANDED_TILES_START: usize = 0x110000;
const DEFAULT_EXPANDED_TILES_SIZE: usize = 0x1000;

const BANK_TABLE_SNES_ADDR: usize = 0xCF80;
const HI_TABLE_SNES_ADDR: usize = 0xD05F;
const LO_TABLE_SNES_ADDR: usize = 0xD13E;

const BPP4_SHEET_LEN: usize = 0x7000;
const BPP3_SHEET_LEN: usize = 0x600;
const BPP2_SHEET_LEN: usize = 0x800;

const _SHEETS_BG_TILES: Range<usize> = 0..113;
const SHEETS_HUD: Range<usize> = 113..115;
const SHEETS_UNCOMPRESSED_3BPP_SPRITES: Range<usize> = 115..127;
const _SHEETS_COMPRESSED_3BPP_SPRITES: Range<usize> = 127..218;
const SHEETS_FONTS: Range<usize> = 218..223;
const SHEETS_MAX: usize = 223;

const DEFAULT_MODULE_PREFIX: &str = "gfxTile";
const DEFAULT_LINK_MODULE: &str = "gfxLink";
const DEFAULT_LABEL: &str = "gfxData";

fn snes_bytes_to_pc(bank: u8, high: u8, low: u8) -> usize {
    let snes_addr = ((bank as u32) << 16) + ((high as u32) << 8) + (low as u32);
    snes_to_pc(snes_addr)
}

fn snes_to_pc(snes_addr: u32) -> usize {
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

fn pc_to_snes(pc_addr: usize) -> u32 {
    let bytes = pc_to_snes_bytes(pc_addr);
    ((bytes[0] as u32) << 16) + ((bytes[1] as u32) << 8) + (bytes[2] as u32)
}

fn is_compressed(sheet: usize) -> bool {
    !SHEETS_UNCOMPRESSED_3BPP_SPRITES.contains(&sheet)
}

fn is_3bpp(sheet: usize) -> bool {
    !SHEETS_HUD.contains(&sheet) && !SHEETS_FONTS.contains(&sheet)
}

fn palette_size(sheet: usize) -> usize {
    if is_3bpp(sheet) {
        8
    } else {
        4
    }
}

fn get_gfx_address(
    i: usize,
    romdata: &Vec<u8>,
    bank_table_addr: u32,
    hi_table_addr: u32,
    lo_table_addr: u32,
) -> usize {
    snes_bytes_to_pc(
        romdata[snes_to_pc(bank_table_addr) + i] as u8,
        romdata[snes_to_pc(hi_table_addr) + i] as u8,
        romdata[snes_to_pc(lo_table_addr) + i] as u8,
    )
}

fn put_gfx_address(
    i: usize,
    romdata: &mut Vec<u8>,
    bank_table_addr: u32,
    hi_table_addr: u32,
    lo_table_addr: u32,
    addr: usize,
) {
    let bytes = pc_to_snes_bytes(addr);
    romdata[snes_to_pc(bank_table_addr) + i] = bytes[0];
    romdata[snes_to_pc(hi_table_addr) + i] = bytes[1];
    romdata[snes_to_pc(lo_table_addr) + i] = bytes[2];
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
        for end in start + 4..std::cmp::min(sheet_data.len(), start + MAX_COPY_EXISTING_LEN) {
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

fn bpp4_sheet_to_pixels(sheet: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; 128]; 448];
    for tile_y in 0..56 {
        for tile_x in 0..16 {
            for byte_pos in 0..32 {
                let byte = sheet[byte_pos + tile_x * 32 + tile_y * 512];
                for bit_pos in 0..8 {
                    if byte & (1 << (7 - bit_pos) as u8) > 0 {
                        let row = (byte_pos % 16) / 2;
                        let plane = (byte_pos % 2 + 2 * (byte_pos / 16)) as u8;
                        out[tile_y * 8 + row][tile_x * 8 + bit_pos] |= 1 << (plane);
                    }
                }
            }
        }
    }
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
                    if line[0] & (1 << (px_x as u8 * 2)) > 0 {
                        px_hi += 1;
                    }
                    if line[1] & (1 << (px_x as u8 * 2)) > 0 {
                        px_hi += 2;
                    }
                    if line[2] & (1 << (px_x as u8 * 2)) > 0 {
                        px_hi += 4;
                    }
                    if line[0] & (1 << (px_x as u8 * 2 + 1)) > 0 {
                        px_lo += 1;
                    }
                    if line[1] & (1 << (px_x as u8 * 2 + 1)) > 0 {
                        px_lo += 2;
                    }
                    if line[2] & (1 << (px_x as u8 * 2 + 1)) > 0 {
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

fn bpp2_sheet_to_pixels(sheet: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; 128]; 64];
    for tile_y in 0..8 {
        for tile_x in 0..16 {
            for px_y in 0..8 {
                let line = [
                    sheet[px_y * 2 + tile_x * 16 + tile_y * 256],
                    sheet[px_y * 2 + tile_x * 16 + tile_y * 256 + 1],
                ];
                for px_x in 0..4 {
                    let mut px_hi = 0;
                    let mut px_lo = 0;
                    if line[0] & (1 << (px_x as u8 * 2)) > 0 {
                        px_hi += 1;
                    }
                    if line[1] & (1 << (px_x as u8 * 2)) > 0 {
                        px_hi += 2;
                    }
                    if line[0] & (1 << (px_x as u8 * 2 + 1)) > 0 {
                        px_lo += 1;
                    }
                    if line[1] & (1 << (px_x as u8 * 2 + 1)) > 1 {
                        px_lo += 2;
                    }
                    out[tile_y * 8 + px_y][tile_x * 8 + (7 - px_x * 2)] = px_hi;
                    out[tile_y * 8 + px_y][tile_x * 8 + (7 - (px_x * 2 + 1))] = px_lo;
                }
            }
        }
    }
    out
}

fn pixels_to_bpp4_sheet(px_data: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = vec![0; BPP4_SHEET_LEN];
    for tile_y in 0..56 {
        for tile_x in 0..16 {
            for byte_pos in 0..32 {
                let row = (byte_pos % 16) / 2;
                let plane = (byte_pos % 2 + 2 * (byte_pos / 16)) as u8;
                for bit_pos in 0..8 {
                    if px_data[tile_y * 8 + row][tile_x * 8 + bit_pos] & (1 << plane) > 0 {
                        out[byte_pos + tile_x * 32 + tile_y * 512] |= 1 << (7 - bit_pos) as u8;
                    }
                }
            }
        }
    }
    out
}

fn pixels_to_bpp3_sheet(px_data: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = vec![0; BPP3_SHEET_LEN];
    for tile_y in 0..4 {
        for tile_x in 0..16 {
            for px_y in 0..8 {
                let mut line = [0u8, 0u8, 0u8];
                for px_x in 0..4 {
                    let px_hi = px_data[tile_y * 8 + px_y][tile_x * 8 + (7 - px_x * 2)];
                    let px_lo = px_data[tile_y * 8 + px_y][tile_x * 8 + (7 - (px_x * 2 + 1))];
                    if px_hi & 1 > 0 {
                        line[0] |= 1 << (px_x as u8 * 2);
                    }
                    if px_hi & 2 > 0 {
                        line[1] |= 1 << (px_x as u8 * 2);
                    }
                    if px_hi & 4 > 0 {
                        line[2] |= 1 << (px_x as u8 * 2);
                    }
                    if px_lo & 1 > 0 {
                        line[0] |= 1 << (px_x as u8 * 2 + 1);
                    }
                    if px_lo & 2 > 0 {
                        line[1] |= 1 << (px_x as u8 * 2 + 1);
                    }
                    if px_lo & 4 > 0 {
                        line[2] |= 1 << (px_x as u8 * 2 + 1);
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

fn pixels_to_bpp2_sheet(px_data: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = vec![0; BPP2_SHEET_LEN];
    for tile_y in 0..8 {
        for tile_x in 0..16 {
            for px_y in 0..8 {
                let mut line = [0u8, 0u8];
                for px_x in 0..4 {
                    let px_hi = px_data[tile_y * 8 + px_y][tile_x * 8 + (7 - px_x * 2)];
                    let px_lo = px_data[tile_y * 8 + px_y][tile_x * 8 + (7 - (px_x * 2 + 1))];
                    if px_hi & 1 > 0 {
                        line[0] |= 1 << (px_x as u8 * 2);
                    }
                    if px_hi & 2 > 0 {
                        line[1] |= 1 << (px_x as u8 * 2);
                    }
                    if px_lo & 1 > 0 {
                        line[0] |= 1 << (px_x as u8 * 2 + 1);
                    }
                    if px_lo & 2 > 0 {
                        line[1] |= 1 << (px_x as u8 * 2 + 1);
                    }
                    out[px_y * 2 + tile_x * 16 + tile_y * 256] = line[0];
                    out[px_y * 2 + tile_x * 16 + tile_y * 256 + 1] = line[1];
                }
            }
        }
    }
    out
}

fn load_sheet(
    bank_table_addr: u32,
    hi_table_addr: u32,
    lo_table_addr: u32,
    romdata: &Vec<u8>,
    sheet: usize,
) -> (usize, Vec<Vec<u8>>) {
    let sheet_addr = get_gfx_address(
        sheet,
        &romdata,
        bank_table_addr,
        hi_table_addr,
        lo_table_addr,
    );
    let sheet_len = if is_3bpp(sheet) {
        BPP3_SHEET_LEN
    } else {
        BPP2_SHEET_LEN
    };
    load_sheet_raw(
        romdata,
        sheet_addr,
        if is_3bpp(sheet) { 3 } else { 2 },
        is_compressed(sheet),
        sheet_len,
    )
}

fn load_sheet_raw(
    romdata: &Vec<u8>,
    sheet_addr: usize,
    bpp: usize,
    compressed: bool,
    sheet_len: usize,
) -> (usize, Vec<Vec<u8>>) {
    let decompressed_sheet;
    if compressed {
        decompressed_sheet = decompress_sheet(romdata, sheet_addr, sheet_len, true);
    } else {
        decompressed_sheet = romdata[sheet_addr..sheet_addr + sheet_len]
            .iter()
            .cloned()
            .collect();
    }
    let sheet_data = match bpp {
        2 => bpp2_sheet_to_pixels(&decompressed_sheet),
        3 => bpp3_sheet_to_pixels(&decompressed_sheet),
        4 => bpp4_sheet_to_pixels(&decompressed_sheet),
        _ => panic!(),
    };
    (sheet_addr, sheet_data)
}

fn compress_sheet_data(bpp: usize, compressed: bool, sheet_data: &mut Vec<Vec<u8>>) -> Vec<u8> {
    let out_sheet = match bpp {
        2 => pixels_to_bpp2_sheet(&sheet_data),
        3 => pixels_to_bpp3_sheet(&sheet_data),
        4 => pixels_to_bpp4_sheet(&sheet_data),
        _ => panic!(),
    };
    let compressed_sheet;
    if compressed {
        compressed_sheet = compress_sheet(&out_sheet, true);
    } else {
        compressed_sheet = out_sheet;
    }
    compressed_sheet
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
                8 => print!("{}█", termion::color::Fg(termion::color::LightBlack)),
                9 => print!("{}█", termion::color::Fg(termion::color::LightRed)),
                10 => print!("{}█", termion::color::Fg(termion::color::LightBlue)),
                11 => print!("{}█", termion::color::Fg(termion::color::LightGreen)),
                12 => print!("{}█", termion::color::Fg(termion::color::LightYellow)),
                13 => print!("{}█", termion::color::Fg(termion::color::LightMagenta)),
                14 => print!("{}█", termion::color::Fg(termion::color::LightCyan)),
                15 => print!("{}█", termion::color::Fg(termion::color::LightWhite)),
                _ => {}
            };
        }
        println!();
    }
}

fn dump_sheets(
    bank_table_addr: u32,
    hi_table_addr: u32,
    lo_table_addr: u32,
    romdata: &mut Vec<u8>,
    sheet_arg: Option<usize>,
) -> () {
    match sheet_arg {
        Some(sheet) => {
            let (sheet_addr, sheet_data) = load_sheet(
                bank_table_addr,
                hi_table_addr,
                lo_table_addr,
                &romdata,
                sheet,
            );
            dump_sheet(sheet, &sheet_data, sheet_addr);
        }
        None => {
            for sheet in 0..SHEETS_MAX {
                let (sheet_addr, sheet_data) = load_sheet(
                    bank_table_addr,
                    hi_table_addr,
                    lo_table_addr,
                    &romdata,
                    sheet,
                );
                dump_sheet(sheet, &sheet_data, sheet_addr);
            }
        }
    }
}

fn patch_tile(
    sheet_data: &mut Vec<Vec<u8>>,
    png_path: &Path,
    palette_size: usize,
    x: usize,
    y: usize,
) -> Result<(), Box<Error>> {
    let png_file = OpenOptions::new().read(true).write(false).open(png_path)?;
    let decoder = png::Decoder::new(png_file);
    let (_, mut reader) = decoder.read_info()?;
    if let Some(first_row) = reader.next_row()? {
        let palette: HashMap<(u8, u8, u8), usize> = first_row
            .iter()
            .tuples::<(_, _, _)>()
            .take(palette_size)
            .enumerate()
            .map(|(a, (r, g, b))| ((*r, *g, *b), a))
            .collect();
        let mut png_y = 0;
        let mut row = reader.next_row()?;
        while row.is_some() {
            let mut png_x = 0;
            for px in row
                .unwrap()
                .iter()
                .tuples::<(_, _, _)>()
                .map(|(r, g, b)| (*r, *g, *b))
            {
                if let Some(palette_idx) = palette.get(&px) {
                    sheet_data[y + png_y][x + png_x] = *palette_idx as u8;
                    png_x += 1;
                } else {
                    return Err(Box::from(SimpleError::new(format!(
                        "Color {:?} at {},{} not in palette row",
                        px, png_x, png_y
                    ))));
                }
            }
            row = reader.next_row()?;
            png_y += 1;
        }
        Ok(())
    } else {
        Err(Box::from(SimpleError::new("Empty PNG file")))
    }
}

fn patch_manifest(
    sheet_data: &mut Vec<Vec<u8>>,
    manifest_path: &str,
    palette_size: usize,
) -> Result<(), Box<Error>> {
    info!("patching from manifest {}", manifest_path);
    let manifest_path = Path::new(manifest_path);
    let manifest_file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(manifest_path)?;
    let parent = manifest_path.parent().unwrap();
    let reader = BufReader::new(&manifest_file);
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let parts: Vec<&str> = line.split(",").collect();
            let png_path = parent.to_path_buf().join(Path::new(parts[0]));
            info!("patching tile from {:?}", png_path);
            patch_tile(
                sheet_data,
                png_path.as_path(),
                palette_size,
                usize::from_str(parts[1])?,
                usize::from_str(parts[2])?,
            )?;
        }
    }
    Ok(())
}

fn write_rom(
    romdata: &mut Vec<u8>,
    sheet_data: &Vec<u8>,
    sheet_start: usize,
    output_rom_path: &str,
) -> Result<(), Box<Error>> {
    romdata.splice(
        sheet_start..sheet_start + sheet_data.len(),
        sheet_data.iter().cloned(),
    );
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(output_rom_path)?;
    file.write(&romdata)?;
    Ok(())
}

fn write_asm(
    sheet_data: &Vec<u8>,
    sheet_start: usize,
    output_asm_path: &str,
    asm_module: &str,
    asm_label: &str,
) -> Result<(), Box<Error>> {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_asm_path)?;
    let mut writer = BufWriter::new(&file);
    writeln!(&mut writer, "        .module {}", asm_module)?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "        .org ${:06X}", pc_to_snes(sheet_start))?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "{}:", asm_label)?;
    for line in &sheet_data.iter().chunks(8) {
        writeln!(
            &mut writer,
            "        .db {}",
            line.map(|byte| format!("${:02X}", byte)).join(", ")
        )?;
    }
    Ok(())
}

fn parse_hex_arg(arg_matches: &ArgMatches, arg: &str, default: usize) -> Result<usize, Box<Error>> {
    Ok(arg_matches.value_of(arg).map_or(Ok(default), |arg| {
        usize::from_str_radix(arg.trim_start_matches("0x"), 16)
    })?)
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();

    let matches = clap_app!(lttp_tilepatch =>
        (@arg in_ROM: +required "input ROM file")
        (@arg banktable: "Bank table address")
        (@arg hitable: "High table address")
        (@arg lotable: "Low table address")
        (@subcommand patch =>
            (about: "Patch PNGs into a tile sheet")
            (@arg sheet: -s --sheet +required +takes_value "target tile sheet (0-222)")
            (@arg in_manifest: -m --manifest +takes_value "CSV-formatted manifest file")
            (@arg in_png: -p --png +takes_value "input PNG file")
            (@arg x: -x +takes_value "target X coordinate")
            (@arg y: -y +takes_value "target Y coordinate")
            (@arg out_ROM: -o --out +takes_value "output ROM file")
            (@arg out_ASM: -a --asm_file +takes_value "name of ASM file to output containing the patched sheet")
            (@arg asm_module: --asm_module +takes_value "module name to use for the ASM file")
            (@arg asm_label: --asm_label +takes_value "label prefix to use for the data in the ASM file")
            (@arg expanded_tiles_start: --exp_start "ROM address to start new tilesheets")
            (@arg expanded_tiles_size: --exp_size "space to reserve for each tilesheet")
        )
        (@subcommand patch_link =>
            (about: "Patch PNGs into Link's sprite sheet")
            (@arg in_manifest: -m --manifest +takes_value "CSV-formatted manifest file")
            (@arg in_png: -p --png +takes_value "input PNG file")
            (@arg x: -x +takes_value "target X coordinate")
            (@arg y: -y +takes_value "target Y coordinate")
            (@arg out_ROM: -o --out +takes_value "output ROM file")
            (@arg out_ASM: -a --asm_file +takes_value "name of ASM file to output containing the patched sheet")
            (@arg asm_module: --asm_module +takes_value "module name to use for the ASM file")
            (@arg asm_label: --asm_label +takes_value "label prefix to use for the data in the ASM file")
            (@arg sheet_addr: --addr +takes_value "address of sprite sheet")
        )
        (@subcommand dump =>
            (about: "Dump all tile sheets to the console")
            (@arg sheet: -s --sheet +takes_value "target tile sheet (0-222)")
        )
        (@subcommand dump_raw =>
            (about: "Dumps tile sheet from a specific location")
            (@arg sheet_addr: -a +required +takes_value "address to look for tile data")
            (@arg sheet_len: -l +required +takes_value "length of sheet data")
            (@arg uncompressed: --uncompressed "don't try to decompress data")
            (@arg bpp: --bpp +required +takes_value "bits per pixel")
        )
    )
        .get_matches();

    let input_rom_path = matches.value_of("in_ROM").unwrap();
    let bank_table_addr = parse_hex_arg(&matches, "banktable", BANK_TABLE_SNES_ADDR)? as u32;
    let hi_table_addr = parse_hex_arg(&matches, "hitable", HI_TABLE_SNES_ADDR)? as u32;
    let lo_table_addr = parse_hex_arg(&matches, "lotable", LO_TABLE_SNES_ADDR)? as u32;

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(input_rom_path)?;
    let mut romdata = Vec::new();
    file.read_to_end(&mut romdata)?;

    if let Some(patch) = matches.subcommand_matches("patch") {
        let sheet = usize::from_str(patch.value_of("sheet").unwrap())?;
        let exp_start =
            parse_hex_arg(&patch, "expanded_tiles_start", DEFAULT_EXPANDED_TILES_START)?;
        let exp_size = parse_hex_arg(&patch, "expanded_tiles_size", DEFAULT_EXPANDED_TILES_SIZE)?;
        let sheet_start = exp_start + (exp_size * sheet);

        let (_, mut sheet_data) = load_sheet(
            bank_table_addr,
            hi_table_addr,
            lo_table_addr,
            &romdata,
            sheet,
        );
        if let Some(manifest_path) = patch.value_of("in_manifest") {
            patch_manifest(&mut sheet_data, manifest_path, palette_size(sheet))?;
        } else {
            patch_tile(
                &mut sheet_data,
                &Path::new(patch.value_of("in_png").unwrap()),
                palette_size(sheet),
                usize::from_str(patch.value_of("x").unwrap())?,
                usize::from_str(patch.value_of("y").unwrap())?,
            )?;
        }
        let compressed_sheet = compress_sheet_data(
            if is_3bpp(sheet) { 3 } else { 2 },
            is_compressed(sheet),
            &mut sheet_data,
        );
        if let Some(rom_path) = patch.value_of("out_ROM") {
            put_gfx_address(
                sheet,
                &mut romdata,
                bank_table_addr,
                hi_table_addr,
                lo_table_addr,
                sheet_start,
            );
            write_rom(&mut romdata, &compressed_sheet, sheet_start, rom_path)?;
        } else if let Some(asm_path) = patch.value_of("out_ASM") {
            write_asm(
                &compressed_sheet,
                sheet_start,
                asm_path,
                &format!(
                    "{}{}",
                    patch
                        .value_of("asm_module")
                        .unwrap_or(DEFAULT_MODULE_PREFIX),
                    sheet
                ),
                patch.value_of("asm_label").unwrap_or(DEFAULT_LABEL),
            )?;
        }
    }
    if let Some(patch_link) = matches.subcommand_matches("patch_link") {
        let sheet_start = parse_hex_arg(&patch_link, "sheet_addr", 0x080000)?;

        let (_, mut sheet_data) = load_sheet_raw(&romdata, sheet_start, 4, false, BPP4_SHEET_LEN);
        if let Some(manifest_path) = patch_link.value_of("in_manifest") {
            patch_manifest(&mut sheet_data, manifest_path, 16)?;
        } else {
            patch_tile(
                &mut sheet_data,
                &Path::new(patch_link.value_of("in_png").unwrap()),
                16,
                usize::from_str(patch_link.value_of("x").unwrap())?,
                usize::from_str(patch_link.value_of("y").unwrap())?,
            )?;
        }
        let compressed_sheet = compress_sheet_data(4, false, &mut sheet_data);
        if let Some(rom_path) = patch_link.value_of("out_ROM") {
            write_rom(&mut romdata, &compressed_sheet, sheet_start, rom_path)?;
        } else if let Some(asm_path) = patch_link.value_of("out_ASM") {
            write_asm(
                &compressed_sheet,
                sheet_start,
                asm_path,
                patch_link
                    .value_of("asm_module")
                    .unwrap_or(DEFAULT_LINK_MODULE),
                patch_link.value_of("asm_label").unwrap_or(DEFAULT_LABEL),
            )?;
        }
    }
    if let Some(dump) = matches.subcommand_matches("dump") {
        dump_sheets(
            bank_table_addr,
            hi_table_addr,
            lo_table_addr,
            &mut romdata,
            dump.value_of("sheet")
                .map(|arg| usize::from_str(arg))
                .map_or(Ok(None), |v| v.map(Some))?,
        )
    }
    if let Some(dump_raw) = matches.subcommand_matches("dump_raw") {
        let (sheet_addr, sheet_data) = load_sheet_raw(
            &romdata,
            parse_hex_arg(dump_raw, "sheet_addr", 0)?,
            usize::from_str(dump_raw.value_of("bpp").unwrap()).unwrap(),
            !dump_raw.is_present("uncompressed"),
            parse_hex_arg(dump_raw, "sheet_len", BPP3_SHEET_LEN)?,
        );
        dump_sheet(0, &sheet_data, sheet_addr);
    }
    Ok(())
}
