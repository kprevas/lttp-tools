extern crate pbr;

use self::pbr::*;
use simple_error::SimpleError;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::path::Path;
use std::thread;

use crate::manifest::*;
use crate::nspc::{CallLoopRef, Song};

pub const DEFAULT_BANK_BASE_ADDRS: [u32; 3] = [0x914, 0x926, 0x932];
const BANK_FIRST_SONG_ADDRS: [usize; 3] = [0xD036, 0xD046, 0xD046];
const ARAM_BASE: usize = 0xd000;

fn snes_to_pc_addr(snes_addr: u32) -> usize {
    ((snes_addr & 0x7FFF) + ((snes_addr / 2) & 0xFF8000)) as usize
}

fn snes_bytes_to_pc_addr(bank: u8, high: u8, low: u8) -> usize {
    snes_to_pc_addr(((bank as u32) << 16) + ((high as u32) << 8) + (low as u32))
}

fn addr_to_bytes(addr: usize) -> (u8, u8) {
    ((addr >> 8 & 0xFF) as u8, (addr & 0xFF) as u8)
}

struct RomCallLoopRef {
    pub target_track: usize,
    pub chunk_base: usize,
    pub ref_pos: u64,
}

struct Chunk {
    offset_addr: usize,
    length: usize,
    aram_addr: usize,
}

impl Chunk {
    fn load(romdata: &Vec<u8>, start_addr: usize) -> Chunk {
        let length = ((romdata[start_addr + 1] as usize) << 8) + (romdata[start_addr] as usize);
        let aram_addr =
            ((romdata[start_addr + 3] as usize) << 8) + (romdata[start_addr + 2] as usize);
        let offset_addr = start_addr + 4;
        Chunk {
            offset_addr,
            length,
            aram_addr,
        }
    }

    fn write_header(&self, romdata: &mut Vec<u8>, start_addr: usize) {
        romdata[start_addr] = (self.length & 0xff) as u8;
        romdata[start_addr + 1] = ((self.length >> 8) & 0xff) as u8;
        romdata[start_addr + 2] = (self.aram_addr & 0xff) as u8;
        romdata[start_addr + 3] = ((self.aram_addr >> 8) & 0xff) as u8;
    }
}

pub fn write(
    manifest: &Manifest,
    path: &Path,
    bank_base_addrs: [u32; 3],
    converter: &Fn(&Path, f32) -> Result<Song, Box<Error>>,
    verbose: bool,
) -> Result<(), Box<Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut romdata = Vec::new();
    file.read_to_end(&mut romdata)?;

    let num_songs = manifest
        .banks
        .iter()
        .fold(0, |acc, bank| acc + bank.songs.len());
    let mut mb = MultiBar::new();
    let mut songs_pb = mb.create_bar(num_songs as u64);
    let mut bank_pbs = manifest
        .banks
        .iter()
        .map(|bank| {
            let mut pb = mb.create_bar(1);
            pb.message(&format!("{} ", bank.name));
            pb.format("[==-]");
            pb.show_speed = false;
            pb.show_time_left = false;
            pb.show_counter = false;
            pb.set(0);
            pb
        })
        .collect::<Vec<ProgressBar<Pipe>>>();

    let mb_thread = thread::spawn(move || {
        mb.listen();
    });

    manifest.banks.iter().enumerate().fold(
        Ok(0),
        |first_song_result: Result<usize, Box<Error>>, (i, bank)| {
            first_song_result.and_then(|first_song| {
                songs_pb.message(&format!("Writing {} songs ", bank.name));
                write_bank(
                    bank,
                    &mut romdata,
                    bank_base_addrs[i],
                    BANK_FIRST_SONG_ADDRS[i],
                    first_song,
                    converter,
                    &mut songs_pb,
                    &mut bank_pbs[i],
                    verbose,
                )?;
                bank_pbs[i].finish_print(&format!("{} bank complete.", bank.name));
                Ok(first_song + bank.songs.len())
            })
        },
    )?;

    file.seek(SeekFrom::Start(0))?;
    file.write(&romdata)?;
    songs_pb.finish_print("All songs written.");
    mb_thread.join().unwrap();
    Ok(())
}

fn write_bank(
    bank: &Bank,
    romdata: &mut Vec<u8>,
    base_addr: u32,
    first_song_addr: usize,
    first_song: usize,
    converter: &Fn(&Path, f32) -> Result<Song, Box<Error>>,
    songs_pb: &mut ProgressBar<Pipe>,
    bank_pb: &mut ProgressBar<Pipe>,
    verbose: bool,
) -> Result<(), Box<Error>> {
    // find chunk going to ARAM D000
    let bank_addr = romdata[snes_to_pc_addr(base_addr + 8)];
    let high_addr = romdata[snes_to_pc_addr(base_addr + 4)];
    let low_addr = romdata[snes_to_pc_addr(base_addr)];
    let mut addr = snes_bytes_to_pc_addr(bank_addr, high_addr, low_addr);
    let mut last_chunk_length = 0xffffusize;
    let mut base_chunk_addr = 0;
    let mut base_chunk_len = 0;
    let mut overflow_chunk_addr = 0;
    let mut overflow_chunk_len = 0;
    let mut aram_overflow_base = 0;
    while last_chunk_length != 0 {
        let chunk = Chunk::load(&romdata, addr);
        last_chunk_length = chunk.length;
        if chunk.aram_addr == ARAM_BASE {
            base_chunk_addr = chunk.offset_addr;
            base_chunk_len = chunk.length;
            addr = chunk.offset_addr + chunk.length;
            let chunk = Chunk::load(&romdata, addr);
            last_chunk_length = chunk.length;
            overflow_chunk_addr = chunk.offset_addr;
            overflow_chunk_len = chunk.length;
            aram_overflow_base = chunk.aram_addr;
        }
        addr = chunk.offset_addr + chunk.length;
    }
    if base_chunk_addr == 0 {
        return Err(Box::from(SimpleError::new(format!(
            "Couldn't find base chunk for {} bank",
            bank.name
        ))));
    }
    if overflow_chunk_addr == 0 {
        return Err(Box::from(SimpleError::new(format!(
            "Couldn't find overflow chunk for {} bank",
            bank.name
        ))));
    }
    let mut rom_addr = base_chunk_addr;
    let mut chunk_length = base_chunk_len;
    let mut aram_base_addr = ARAM_BASE;

    if verbose {
        println!(
            "Writing {} bank to 0x{:X} starting at song {}.  Available chunk length is 0x{:X}",
            bank.name, rom_addr, first_song, chunk_length
        );
    }

    bank_pb.total = (base_chunk_len + overflow_chunk_len) as u64;
    bank_pb.show_counter = true;

    let mut song_table_addr = rom_addr + first_song * 2;
    let mut song_offset = first_song_addr - aram_base_addr;

    for song_def in &bank.songs {
        bank_pb.message(&match &song_def.input {
            Some(path) => format!("{} ", path.file_name().unwrap().to_str().unwrap()),
            None => "[empty song] ".to_string(),
        });
        bank_pb.set(
            (song_offset
                + if rom_addr == base_chunk_addr {
                    0
                } else {
                    base_chunk_len
                }) as u64,
        );

        let song_data = match &song_def.input {
            Some(path) => converter(&path, song_def.tempo_factor)?,
            None => Song::empty()?,
        };

        // check if non-track data fits in chunk
        if song_offset + (if song_def.loops { 8 } else { 4 }) + 16 > chunk_length {
            if aram_base_addr == aram_overflow_base {
                return Err(Box::from(SimpleError::new(format!(
                    "{} bank does not fit in available chunks",
                    bank.name
                ))));
            }
            if verbose {
                println!(
                    "Switching to overflow chunk - data size before switch 0x{:X}",
                    chunk_length - song_offset
                );
            }
            song_offset = 0;
            rom_addr = overflow_chunk_addr;
            chunk_length = overflow_chunk_len;
            aram_base_addr = aram_overflow_base;
        };

        // write song address to song table
        if verbose {
            println!(
                "Writing song address 0x{:X} (0x{:X}) to song table at 0x{:X}",
                rom_addr + song_offset,
                aram_base_addr + song_offset,
                song_table_addr
            );
        }
        let song_addr_bytes = addr_to_bytes(aram_base_addr + song_offset);
        romdata[song_table_addr + 1] = song_addr_bytes.0;
        romdata[song_table_addr] = song_addr_bytes.1;
        song_table_addr += 2;

        // song data
        let part_data_offset;
        if song_def.loops {
            // part address + loop command + loop target + terminator
            part_data_offset = song_offset + 8;
        } else {
            // part address + terminator
            part_data_offset = song_offset + 4;
        };
        let part_data_aram_addr = aram_base_addr + part_data_offset;
        let part_data_rom_addr = rom_addr + part_data_offset;
        let part_data_bytes = addr_to_bytes(part_data_aram_addr);
        romdata[rom_addr + song_offset + 1] = part_data_bytes.0;
        romdata[rom_addr + song_offset] = part_data_bytes.1;
        if song_def.loops {
            romdata[rom_addr + song_offset + 3] = 0x00;
            romdata[rom_addr + song_offset + 2] = 0xff;
            romdata[rom_addr + song_offset + 5] = song_addr_bytes.0;
            romdata[rom_addr + song_offset + 4] = song_addr_bytes.1;
            romdata[rom_addr + song_offset + 7] = 0x00;
            romdata[rom_addr + song_offset + 6] = 0x00;
        } else {
            romdata[rom_addr + song_offset + 3] = 0x00;
            romdata[rom_addr + song_offset + 2] = 0x00;
        };

        // track data
        let mut track_data_offset = part_data_offset + 16;
        let mut track_addrs = Vec::<usize>::new();

        let mut call_loops = Vec::<RomCallLoopRef>::new();

        for i in 0..song_data.get_num_tracks() {
            let track_data = Vec::<u8>::new();
            let mut track_call_loops = Vec::<CallLoopRef>::new();
            let mut pre_chunk_switch_track_call_loops = Vec::<CallLoopRef>::new();
            let track_start_offset = track_data_offset;
            let mut cursor = Cursor::new(track_data);
            song_data.write_track(&mut cursor, i, &mut track_call_loops)?;
            let track_data = cursor.into_inner();

            // check if track data fits in chunk
            if track_data_offset + track_data.len() > chunk_length {
                if aram_base_addr == aram_overflow_base {
                    return Err(Box::from(SimpleError::new(format!(
                        "Couldn't find base chunk for {} bank",
                        bank.name
                    ))));
                }
                if verbose {
                    println!(
                        "Switching to overflow chunk - data size before switch 0x{:X}",
                        chunk_length - song_offset
                    );
                }
                track_data_offset = 0;
                rom_addr = overflow_chunk_addr;
                chunk_length = overflow_chunk_len;
                aram_base_addr = aram_overflow_base;
                pre_chunk_switch_track_call_loops.extend_from_slice(track_call_loops.as_slice());
                track_call_loops.clear();
            };

            if verbose {
                println!(
                    "Writing track to 0x{:X} (0x{:X})",
                    rom_addr + track_data_offset,
                    aram_base_addr + track_data_offset
                );
            }
            if track_data.len() > 0 {
                romdata.splice(
                    (rom_addr + track_data_offset)
                        ..(rom_addr + track_data_offset + track_data.len()),
                    track_data.iter().cloned(),
                );
                track_addrs.push(aram_base_addr + track_data_offset);
                pre_chunk_switch_track_call_loops
                    .iter()
                    .for_each(|call_loop| {
                        call_loops.push(RomCallLoopRef {
                            target_track: call_loop.target_track,
                            chunk_base: base_chunk_addr + track_start_offset,
                            ref_pos: call_loop.ref_pos,
                        })
                    });
                track_call_loops.iter().for_each(|call_loop| {
                    call_loops.push(RomCallLoopRef {
                        target_track: call_loop.target_track,
                        chunk_base: rom_addr + track_data_offset,
                        ref_pos: call_loop.ref_pos,
                    })
                });
                track_data_offset += track_data.len();
            } else {
                track_addrs.push(0);
            };
        }

        // part data
        if verbose {
            println!(
                "Writing part data to 0x{:X} (0x{:X})",
                part_data_rom_addr, part_data_aram_addr
            );
        }
        let mut part_data_addr = part_data_rom_addr;
        for i in 0..16 {
            romdata[part_data_addr + i] = 0;
        }
        song_data.get_part_tracks(0).iter().for_each(|&track_idx| {
            if verbose {
                println!(
                    "Writing track address 0x{:X} to part data at 0x{:X}",
                    track_addrs[track_idx], part_data_addr
                );
            }
            let track_bytes = addr_to_bytes(track_addrs[track_idx]);
            romdata[part_data_addr + 1] = track_bytes.0;
            romdata[part_data_addr] = track_bytes.1;
            part_data_addr += 2;
        });

        call_loops.iter().for_each(|call_loop| {
            let call_loop_addr = call_loop.chunk_base + (call_loop.ref_pos as usize);
            if verbose {
                println!(
                    "Writing loop address 0x{:X} to CallLoop instruction at 0x{:X}",
                    track_addrs[call_loop.target_track], call_loop_addr
                );
            }
            let track_bytes = addr_to_bytes(track_addrs[call_loop.target_track]);
            romdata[call_loop_addr + 1] = track_bytes.0;
            romdata[call_loop_addr] = track_bytes.1;
        });

        if verbose {
            println!(
                "{} - total track size in current bank 0x{:X}",
                match &song_def.input {
                    Some(path) => path.file_name().unwrap().to_str().unwrap(),
                    None => "[empty song]",
                },
                if track_data_offset > song_offset {
                    track_data_offset - song_offset
                } else {
                    track_data_offset
                },
            );
        }
        song_offset = track_data_offset;
        songs_pb.inc();
    }
    for i in song_table_addr..(base_chunk_addr + first_song_addr - ARAM_BASE) {
        romdata[i] = 0x00;
    }
    Ok(())
}

pub fn write_all_overworld(
    song_path: &Path,
    rom_path: &Path,
    bank_base_addrs: [u32; 3],
    converter: &Fn(&Path, f32) -> Result<Song, Box<Error>>,
    verbose: bool,
) -> Result<(), Box<Error>> {
    write(
        &Manifest::single_song(song_path),
        rom_path,
        bank_base_addrs,
        converter,
        verbose,
    )?;
    Ok(())
}

pub fn write_file_select(
    song_path: &Path,
    rom_path: &Path,
    bank_base_addrs: [u32; 3],
    converter: &Fn(&Path, f32) -> Result<Song, Box<Error>>,
    verbose: bool,
) -> Result<(), Box<Error>> {
    write(
        &Manifest::file_select(song_path),
        rom_path,
        bank_base_addrs,
        converter,
        verbose,
    )?;
    Ok(())
}

pub fn gen_fake_rom(
    rom_path: &Path,
    output_path: &Path,
    bank_base_addrs: [u32; 3],
) -> Result<(), Box<Error>> {
    let mut rom_file = OpenOptions::new().read(true).open(rom_path)?;
    let mut romdata = Vec::new();
    rom_file.read_to_end(&mut romdata)?;
    let mut out_romdata = vec![0u8; romdata.len()];

    for &base_addr in bank_base_addrs.iter() {
        let bank_addr = romdata[snes_to_pc_addr(base_addr + 8)];
        let high_addr = romdata[snes_to_pc_addr(base_addr + 4)];
        let low_addr = romdata[snes_to_pc_addr(base_addr)];
        out_romdata[snes_to_pc_addr(base_addr + 8)] = bank_addr;
        out_romdata[snes_to_pc_addr(base_addr + 4)] = high_addr;
        out_romdata[snes_to_pc_addr(base_addr)] = low_addr;

        let mut addr = snes_bytes_to_pc_addr(bank_addr, high_addr, low_addr);
        let mut last_chunk_length = 0xffffusize;
        while last_chunk_length != 0 {
            let chunk = Chunk::load(&romdata, addr);
            chunk.write_header(&mut out_romdata, addr);
            last_chunk_length = chunk.length;
            addr = chunk.offset_addr + chunk.length;
        }
    }

    let mut out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_path)?;
    out_file.write(&out_romdata)?;
    Ok(())
}
