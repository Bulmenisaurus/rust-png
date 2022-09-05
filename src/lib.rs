// https://en.wikipedia.org/wiki/Portable_Network_Graphics
// https://www.w3.org/TR/PNG/
// https://darka.github.io/posts/generating-png-in-python/

use crc::{Crc, CRC_32_ISO_HDLC};
use deflate::deflate_bytes_zlib;

use std::str;

fn encode_image_chunk(chunk_type: &str, chunk_data: &mut Vec<u8>) -> Vec<u8> {
    // chunk data length (4 bytes)
    let mut img_data: Vec<u8> = (chunk_data.len() as i32).to_be_bytes().to_vec();

    // checksum over the chunk type + chunk data

    let mut crc_checksum: Vec<u8> =
        crc_32(&([chunk_type.as_bytes(), &chunk_data[..]]).concat()[..])
            .to_be_bytes()
            .to_vec();

    // chunk type (4 bytes)
    img_data.append(&mut chunk_type.as_bytes().to_vec());
    // chunk data (length bytes)
    img_data.append(chunk_data);
    // crc checksum (4 bytes)
    img_data.append(&mut crc_checksum);

    img_data
}

fn encode_image_metadata(width: i32, height: i32) -> Vec<u8> {
    // 300 -> [0, 0, 1, 44]
    let hex_width = &width.to_be_bytes()[..];
    let hex_height = &height.to_be_bytes()[..];

    let mut image_data = [hex_width, hex_height].concat();

    // each color is 1 byte (0 - 255, 0 - 255, 0 - 255)
    let bit_depth = 8u8;

    // true color + alpha (rgba)
    let color_type = 6u8;

    // deflate/inflate compression
    let compression_method = 0u8;
    // adaptive filtering with five basic filter types
    let filter_method = 0u8;
    // no interlacing
    let interlacing_method = 0u8;

    image_data.append(&mut vec![
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlacing_method,
    ]);

    image_data
}

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub alpha: u8,
}

fn encode_image_pixels(pixels: Vec<RGBA>, width: i32) -> Vec<u8> {
    // [
    //  [RGBA(0, 0, 0, 1), RGBA(255, 255, 255, 1)],
    //  [white, red]
    //]
    let mut scanlines: Vec<u8> = Vec::new();

    for row in pixels.chunks(width as usize) {
        scanlines.push(0u8);

        scanlines.append(
            &mut row
                .iter()
                .map(|pixel| vec![pixel.red, pixel.blue, pixel.green, pixel.alpha])
                .flatten()
                .collect(),
        );
    }

    deflate_bytes_zlib(&scanlines)
}

fn crc_32(data: &[u8]) -> u32 {
    let crc_encode: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

    crc_encode.checksum(data)
}

// TODO: allow a `file` parameter
pub fn create_image(data: Vec<RGBA>, width: i32, height: i32) -> Vec<u8> {
    // https://en.wikipedia.org/wiki/Portable_Network_Graphics#File_header
    let mut image_data = vec![0x89u8, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    // The IHDR contains lots of metadata such as dimensions, bit depth, color type, and more
    image_data.append(&mut encode_image_chunk(
        "IHDR",
        &mut encode_image_metadata(width, height),
    ));

    // The IDAT chunk contains the actual image data which is the output stream of the compression algorithm.
    image_data.append(&mut encode_image_chunk(
        "IDAT",
        &mut encode_image_pixels(data, width),
    ));

    // IEND marks the image end; the data field of the IEND chunk has 0 bytes/is empty
    image_data.append(&mut encode_image_chunk("IEND", &mut vec![]));

    image_data
}
