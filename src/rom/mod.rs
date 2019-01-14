use failure::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::path::Path;

use manifest::*;
use nspc::{CallLoopRef, Song};

const BANK_BASE_ADDRS: [u32; 3] = [0x914, 0x926, 0x932];
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

struct Chunk {
    offset_addr: usize,
    length: usize,
    aram_addr: usize,
    data: Vec<u8>,
}

impl Chunk {
    fn load(romdata: &Vec<u8>, start_addr: usize) -> Chunk {
        let length = ((romdata[start_addr + 1] as usize) << 8) + (romdata[start_addr] as usize);
        let aram_addr =
            ((romdata[start_addr + 3] as usize) << 8) + (romdata[start_addr + 2] as usize);
        let offset_addr = start_addr + 4;
        let data = Vec::from(&romdata[offset_addr..(offset_addr + length)]);
        Chunk {
            offset_addr,
            length,
            aram_addr,
            data,
        }
    }
}

struct SongBank {}

impl SongBank {
    fn read(romdata: &Vec<u8>, addr: u32) -> SongBank {
        let bank = romdata[snes_to_pc_addr(addr + 8)];
        let high = romdata[snes_to_pc_addr(addr + 4)];
        let low = romdata[snes_to_pc_addr(addr)];

        let mut addr = snes_bytes_to_pc_addr(bank, high, low);

        let mut aram = [0u8; 0xffff];

        let mut last_chunk_length = 0xffffusize;
        while last_chunk_length != 0 {
            let chunk = Chunk::load(romdata, addr);
            let aram_chunk = &mut aram[chunk.aram_addr..(chunk.aram_addr + chunk.length)];
            aram_chunk.copy_from_slice(chunk.data.as_slice());
            last_chunk_length = chunk.length;
            addr = chunk.offset_addr + chunk.length;
            println!(
                "Chunk found at {:X} target {:X} length {:X}",
                chunk.offset_addr, chunk.aram_addr, chunk.length
            );
        }

        // TODO read songs

        SongBank {}
    }
}

pub struct Rom {
    _base: SongBank,
}

impl Rom {
    pub fn load(path: &Path) -> Result<Rom, Error> {
        let mut file = File::open(path)?;
        let mut romdata = Vec::new();
        file.read_to_end(&mut romdata)?;

        let bank1 = SongBank::read(&romdata, BANK_BASE_ADDRS[0]);
        let _bank2 = SongBank::read(&romdata, BANK_BASE_ADDRS[1]);
        let _bank3 = SongBank::read(&romdata, BANK_BASE_ADDRS[2]);

        Ok(Rom { _base: bank1 })
    }

    pub fn write(
        manifest: &Manifest,
        path: &Path,
        converter: &Fn(&Path) -> Result<Song, Error>,
    ) -> Result<(), Error> {
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        let mut romdata = Vec::new();
        file.read_to_end(&mut romdata)?;

        manifest.banks.iter().enumerate().fold(
            Ok(0),
            |first_song_result: Result<usize, Error>, (i, bank)| {
                first_song_result.and_then(|first_song| {
                    Rom::write_bank(
                        bank,
                        &mut romdata,
                        BANK_BASE_ADDRS[i],
                        BANK_FIRST_SONG_ADDRS[i],
                        first_song,
                        converter,
                    )?;
                    Ok(first_song + bank.songs.len())
                })
            },
        )?;

        file.seek(SeekFrom::Start(0))?;
        file.write(&romdata)?;
        Ok(())
    }

    fn write_bank(
        bank: &Bank,
        romdata: &mut Vec<u8>,
        base_addr: u32,
        first_song_addr: usize,
        first_song: usize,
        converter: &Fn(&Path) -> Result<Song, Error>,
    ) -> Result<(), Error> {
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
        ensure!(
            base_chunk_addr != 0,
            "Couldn't find base chunk for {} bank",
            bank.name
        );
        ensure!(
            overflow_chunk_addr != 0,
            "Couldn't find overflow chunk for {} bank",
            bank.name
        );
        let mut rom_addr = base_chunk_addr;
        let mut chunk_length = base_chunk_len;
        let mut aram_base_addr = ARAM_BASE;

        println!(
            "Writing {} bank to 0x{:X} starting at song {}.  Available chunk length is {:X}",
            bank.name, rom_addr, first_song, chunk_length
        );

        let mut song_table_addr = rom_addr + first_song * 2;
        let mut song_offset = first_song_addr - aram_base_addr;

        for song_def in &bank.songs {
            let song_data = converter(song_def.input.as_path())?;

            // check if non-track data fits in chunk
            if song_offset + (if song_def.loops { 8 } else { 4 }) + 16 > chunk_length {
                ensure!(
                    aram_base_addr != aram_overflow_base,
                    "{} bank does not fit in available chunks",
                    bank.name
                );
                println!("Switching to overflow chunk");
                song_offset = 0;
                rom_addr = overflow_chunk_addr;
                chunk_length = overflow_chunk_len;
                aram_base_addr = aram_overflow_base;
            };

            // write song address to song table
            println!(
                "Writing song address 0x{:X} (0x{:X}) to song table at 0x{:X}",
                rom_addr + song_offset,
                aram_base_addr + song_offset,
                song_table_addr
            );
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

            let mut call_loops = Vec::<CallLoopRef>::new();

            for i in 0..song_data.get_num_tracks() {
                let track_data = Vec::<u8>::new();
                let mut track_call_loops = Vec::<CallLoopRef>::new();
                let mut cursor = Cursor::new(track_data);
                song_data.write_track(&mut cursor, i, &mut track_call_loops)?;
                let track_data = cursor.into_inner();

                // check if track data fits in chunk
                if track_data_offset + track_data.len() > chunk_length {
                    ensure!(
                        aram_base_addr != aram_overflow_base,
                        "{} bank does not fit in available chunks",
                        bank.name
                    );
                    println!("Switching to overflow chunk");
                    track_data_offset = 0;
                    rom_addr = overflow_chunk_addr;
                    chunk_length = overflow_chunk_len;
                    aram_base_addr = aram_overflow_base;
                };

                println!(
                    "Writing track to 0x{:X} (0x{:X})",
                    rom_addr + track_data_offset,
                    aram_base_addr + track_data_offset
                );
                if track_data.len() > 0 {
                    romdata.splice(
                        (rom_addr + track_data_offset)
                            ..(rom_addr + track_data_offset + track_data.len()),
                        track_data.iter().cloned(),
                    );
                    track_addrs.push(aram_base_addr + track_data_offset);
                    track_call_loops.iter().for_each(|call_loop| {
                        call_loops.push(CallLoopRef {
                            target_track: call_loop.target_track,
                            ref_pos: track_data_offset as u64 + call_loop.ref_pos,
                        })
                    });
                    track_data_offset += track_data.len();
                } else {
                    track_addrs.push(0);
                };
            }

            // part data
            println!(
                "Writing part data to 0x{:X} (0x{:X})",
                part_data_rom_addr, part_data_aram_addr
            );
            let mut part_data_addr = part_data_rom_addr;
            for i in 0..16 {
                romdata[part_data_addr + i] = 0;
            }
            song_data.get_part_tracks(0).iter().for_each(|&track_idx| {
                println!(
                    "Writing track address 0x{:X} to part data at 0x{:X}",
                    track_addrs[track_idx], part_data_addr
                );
                let track_bytes = addr_to_bytes(track_addrs[track_idx]);
                romdata[part_data_addr + 1] = track_bytes.0;
                romdata[part_data_addr] = track_bytes.1;
                part_data_addr += 2;
            });

            call_loops.iter().for_each(|call_loop| {
                let call_loop_addr = rom_addr + (call_loop.ref_pos as usize);
                println!(
                    "Writing loop address 0x{:X} to CallLoop instruction at 0x{:X}",
                    track_addrs[call_loop.target_track], call_loop_addr
                );
                let track_bytes = addr_to_bytes(track_addrs[call_loop.target_track]);
                romdata[call_loop_addr + 1] = track_bytes.0;
                romdata[call_loop_addr] = track_bytes.1;
            });

            song_offset = track_data_offset;
        }
        for i in song_table_addr..(base_chunk_addr + first_song_addr - ARAM_BASE) {
            romdata[i] = 0x00;
        }
        Ok(())
    }

    pub fn write_all_songs_as(
        song_path: &Path,
        rom_path: &Path,
        converter: &Fn(&Path) -> Result<Song, Error>,
    ) -> Result<(), Error> {
        Rom::write(&Manifest::single_song(song_path), rom_path, converter)?;
        Ok(())
    }
}
