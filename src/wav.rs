use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Default)]
pub struct Header {
    chunk_id: u32,
    chunk_size: u32,
    format: u32,
    subchunk1_id: u32,
    subchunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    subchunk2_id: u32,
    subchunk2_size: u32,
}

fn read_header_part_u32(data: &[u8], offset: &mut usize) -> u32 {
    let off_val = *offset;
    *offset += 4 as usize;
    return (data[off_val + 3] as u32) << 24 | (data[off_val + 2] as u32) << 16 | (data[off_val + 1] as u32) << 8 | data[off_val] as u32;
}

fn read_header_part_u16(data: &[u8], offset: &mut usize) -> u16 {
    let off_val = *offset;
    *offset += 2 as usize;
    return (data[off_val + 1] as u16) << 8 | data[off_val + 0] as u16;
}

pub fn read_header(path: &Path) -> Header {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut header = Header::default();
    let mut header_data = [0; 44];

    match file.read(&mut header_data) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => (),
    }
    let mut next_offset: usize = 0;
    header.chunk_id = read_header_part_u32(&header_data, &mut next_offset);
    header.chunk_size = read_header_part_u32(&header_data, &mut next_offset);
    header.format = read_header_part_u32(&header_data, &mut next_offset);
    header.subchunk1_id = read_header_part_u32(&header_data, &mut next_offset);
    header.subchunk1_size = read_header_part_u32(&header_data, &mut next_offset);
    header.audio_format = read_header_part_u16(&header_data, &mut next_offset);
    header.num_channels = read_header_part_u16(&header_data, &mut next_offset);
    header.sample_rate = read_header_part_u32(&header_data, &mut next_offset);
    header.byte_rate = read_header_part_u32(&header_data, &mut next_offset);
    header.block_align = read_header_part_u16(&header_data, &mut next_offset);
    header.bits_per_sample = read_header_part_u16(&header_data, &mut next_offset);
    header.subchunk2_id = read_header_part_u32(&header_data, &mut next_offset);
    header.subchunk2_size = read_header_part_u32(&header_data, &mut next_offset);

    println!("{}", header.sample_rate);

    return header;
}

pub fn read_data(path: &Path) {

}
