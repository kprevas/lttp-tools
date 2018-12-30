use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Cursor;
use std::path::Path;

fn snes_to_pc_addr(snes_addr: u32) -> usize {
    ((snes_addr & 0x7FFF) + ((snes_addr / 2) & 0xFF8000)) as usize
}

fn snes_bytes_to_pc_addr(bank: u8, high: u8, low: u8) -> usize {
    snes_to_pc_addr(((bank as u32) << 16) + ((high as u32) << 8) + (low as u32))
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
        let aram_addr = ((romdata[start_addr + 3] as usize) << 8) + (romdata[start_addr + 2] as usize);
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

struct SongBank {
}

impl SongBank {
    fn read(romdata: &Vec<u8>, addr_bank: u32, addr_high: u32, addr_low: u32) -> SongBank {
        let bank = romdata[snes_to_pc_addr(addr_bank)];
        let high= romdata[snes_to_pc_addr(addr_high)];
        let low= romdata[snes_to_pc_addr(addr_low)];

        let mut addr = snes_bytes_to_pc_addr(bank, high, low);

        let mut aram = [0u8;0xffff];

        let mut last_chunk_length = 0xffffusize;
        while last_chunk_length != 0 {
            let chunk = Chunk::load(romdata, addr);
            let aram_chunk = &mut aram[chunk.aram_addr..(chunk.aram_addr + chunk.length)];
            aram_chunk.copy_from_slice(chunk.data.as_slice());
            last_chunk_length = chunk.length;
            addr = chunk.offset_addr + chunk.length;
        }

        let mut lowest_song_data = 0xffffusize;
        let mut song_index = 0xd000usize;

        // TODO read songs

        SongBank {}
    }
}

pub struct Rom {
    base: SongBank,
}

impl Rom {
    pub fn load(path: &Path) -> Rom {
        let mut file = File::open(path).unwrap();
        let mut romdata = Vec::new();
        file.read_to_end(&mut romdata);

        let base = SongBank::read(&romdata, 0x90a, 0x906, 0x902);

        Rom {
            base
        }
    }

    pub fn write_all_base_songs_as(song: &super::nspc::Song, path: &Path) {
        let mut file = OpenOptions::new().read(true).write(true).open(path).unwrap();
        let mut romdata = Vec::new();
        file.read_to_end(&mut romdata);

        // find chunk going to ARAM D000
        let bank = romdata[snes_to_pc_addr(0x90a)];
        let high= romdata[snes_to_pc_addr(0x906)];
        let low= romdata[snes_to_pc_addr(0x902)];

        let mut addr = snes_bytes_to_pc_addr(bank, high, low);

        let mut last_chunk_length = 0xffffusize;
        while last_chunk_length != 0 {
            let chunk = Chunk::load(&romdata, addr);
            if chunk.aram_addr == 0xd000 {
                addr = chunk.offset_addr;
                break;
            }
            last_chunk_length = chunk.length;
            addr = chunk.offset_addr + chunk.length;
        }

        // point all 15 songs at D036
        for i in 0..15 {
            romdata[addr + i * 2 + 1] = 0xd0;
            romdata[addr + i * 2] = 0x36;
        }

        // song data: single part, looping
        romdata[addr + 0x37] = 0xd0;
        romdata[addr + 0x36] = 0x3e;
        romdata[addr + 0x39] = 0x00;
        romdata[addr + 0x38] = 0xff;
        romdata[addr + 0x3b] = 0xd0;
        romdata[addr + 0x3a] = 0x36;
        romdata[addr + 0x3d] = 0x00;
        romdata[addr + 0x3c] = 0x00;

        let track_table_size = 16usize;
        let mut track_data_addr = 0xd03e + track_table_size;
        let track_data_rom_addr = addr + 0x3e + track_table_size;

        let mut track_addrs = Vec::<usize>::new();
        {
            let mut write = Cursor::new(romdata);
            write.seek(SeekFrom::Start(track_data_rom_addr as u64));

            for i in 0..song.get_num_tracks() {
                let start = write.position();
                song.write_track(&mut write, i);
                let size_written = (write.position() - start) as usize;
                if size_written > 0 {
                    track_addrs.push(track_data_addr);
                    track_data_addr += size_written;
                } else {
                    track_addrs.push(0);
                }
            }
            romdata = write.into_inner();
        }

        let mut track_table_addr = addr + 0x3e;
        for i in 0..track_table_size {
            romdata[track_table_addr + i] = 0;
        }
        song.get_part_tracks(0).iter().for_each(|&track_idx| {
            romdata[track_table_addr] = (track_addrs[track_idx] & 0xff) as u8;
            romdata[track_table_addr + 1] = ((track_addrs[track_idx] & 0xff00) >> 8) as u8;
            track_table_addr += 2;
        });

        file.seek(SeekFrom::Start(0));
        file.write(&romdata);
    }
}