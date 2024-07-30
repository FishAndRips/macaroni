use crate::{Format, Pixel};

// 128x128 bmp of the Ringhopper penguin
const RINGHOPPER_PENGY: &'static [u8] = include_bytes!("ringhopper.bmp");

fn open_ringhopper_image() -> [Pixel; 128*128] {
    let data = RINGHOPPER_PENGY[138..].chunks(4);
    let mut output = [Pixel::default(); 128*128];

    for (data, pixel) in data.zip(output.iter_mut()) {
        pixel.alpha = data[3];
        pixel.red = data[2];
        pixel.green = data[1];
        pixel.blue = data[0];
    }

    output
}

#[test]
pub fn roundtrip_32bit() {
    // 32-bit should output to the same thing...

    let ringhopper = open_ringhopper_image();

    let mut output = [0u8; 128*128*4];
    Format::A8R8G8B8.encode_pixels(&ringhopper, &mut output, 128, 128);
    assert_eq!(RINGHOPPER_PENGY[138..], output);

    let mut output = [0u8; 128*128*4];
    Format::X8R8G8B8.encode_pixels(&ringhopper, &mut output, 128, 128);
    assert_eq!(RINGHOPPER_PENGY[138..], output);
}

#[test]
pub fn roundtrip_uncompressed() {
    // Test encoding to X, decoding X, and re-encoding X. It should be the same.

    macro_rules! roundtrip_fmt{
        ($fmt:expr) => {{
            let ringhopper = open_ringhopper_image();
            let mut output = [0u8; 128 * 128 * $fmt.block_size_bytes()];
            $fmt.encode_pixels(&ringhopper, &mut output, 128, 128);
            let mut ringhopper16 = [Pixel::default(); 128 * 128];
            $fmt.decode_pixels(&output, &mut ringhopper16, 128, 128);
            let mut output_again = [0u8; 128 * 128 * $fmt.block_size_bytes()];
            $fmt.encode_pixels(&ringhopper16, &mut output_again, 128, 128);
            assert_eq!(output, output_again);
        }}
    }

    roundtrip_fmt!(Format::R5G6B5);
    roundtrip_fmt!(Format::A4R4G4B4);
    roundtrip_fmt!(Format::A1R5G5B5);
    roundtrip_fmt!(Format::A8Y8);
    roundtrip_fmt!(Format::AY8);
    roundtrip_fmt!(Format::A8);
    roundtrip_fmt!(Format::Y8);
    roundtrip_fmt!(Format::A8R8G8B8);
    roundtrip_fmt!(Format::X8R8G8B8);
}

#[test]
pub fn roundtrip_palettized() {
    // This palette was generated by GIMP for the 128x128 ringhopper.bmp image.
    let palette = [
        Pixel { blue: 0x4C, green: 0x0A, red: 0x1E, alpha: 0xFF },
        Pixel { blue: 0x20, green: 0x15, red: 0x13, alpha: 0xFF },
        Pixel { blue: 0x18, green: 0x16, red: 0x14, alpha: 0xFF },
        Pixel { blue: 0x1D, green: 0x16, red: 0x16, alpha: 0xFF },
        Pixel { blue: 0x23, green: 0x17, red: 0x15, alpha: 0xFF },
        Pixel { blue: 0x23, green: 0x16, red: 0x1A, alpha: 0xFF },
        Pixel { blue: 0x26, green: 0x1A, red: 0x17, alpha: 0xFF },
        Pixel { blue: 0x1C, green: 0x1B, red: 0x17, alpha: 0xFF },
        Pixel { blue: 0x2A, green: 0x19, red: 0x1A, alpha: 0xFF },
        Pixel { blue: 0x2B, green: 0x18, red: 0x1F, alpha: 0xFF },
        Pixel { blue: 0x23, green: 0x1A, red: 0x20, alpha: 0xFF },
        Pixel { blue: 0x20, green: 0x1E, red: 0x1B, alpha: 0xFF },
        Pixel { blue: 0x2E, green: 0x1D, red: 0x1E, alpha: 0xFF },
        Pixel { blue: 0x2F, green: 0x1C, red: 0x23, alpha: 0xFF },
        Pixel { blue: 0x4F, green: 0x1A, red: 0x22, alpha: 0xFF },
        Pixel { blue: 0x2B, green: 0x1E, red: 0x22, alpha: 0xFF },
        Pixel { blue: 0x1E, green: 0x21, red: 0x1F, alpha: 0xFF },
        Pixel { blue: 0x23, green: 0x21, red: 0x1E, alpha: 0xFF },
        Pixel { blue: 0x2D, green: 0x21, red: 0x1D, alpha: 0xFF },
        Pixel { blue: 0x2E, green: 0x1F, red: 0x28, alpha: 0xFF },
        Pixel { blue: 0x37, green: 0x21, red: 0x20, alpha: 0xFF },
        Pixel { blue: 0x2A, green: 0x22, red: 0x21, alpha: 0xFF },
        Pixel { blue: 0x38, green: 0x20, red: 0x25, alpha: 0xFF },
        Pixel { blue: 0x27, green: 0x25, red: 0x22, alpha: 0xFF },
        Pixel { blue: 0x23, green: 0x25, red: 0x24, alpha: 0xFF },
        Pixel { blue: 0x28, green: 0x23, red: 0x2C, alpha: 0xFF },
        Pixel { blue: 0x41, green: 0x25, red: 0x27, alpha: 0xFF },
        Pixel { blue: 0x42, green: 0x27, red: 0x22, alpha: 0xFF },
        Pixel { blue: 0x42, green: 0x24, red: 0x2D, alpha: 0xFF },
        Pixel { blue: 0x2B, green: 0x29, red: 0x26, alpha: 0xFF },
        Pixel { blue: 0x3F, green: 0x28, red: 0x27, alpha: 0xFF },
        Pixel { blue: 0x28, green: 0x2A, red: 0x29, alpha: 0xFF },
        Pixel { blue: 0x3F, green: 0x28, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x46, green: 0x29, red: 0x2B, alpha: 0xFF },
        Pixel { blue: 0x4B, green: 0x28, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x33, green: 0x2C, red: 0x28, alpha: 0xFF },
        Pixel { blue: 0x48, green: 0x2B, red: 0x26, alpha: 0xFF },
        Pixel { blue: 0x39, green: 0x29, red: 0x33, alpha: 0xFF },
        Pixel { blue: 0x2F, green: 0x2D, red: 0x29, alpha: 0xFF },
        Pixel { blue: 0x39, green: 0x2D, red: 0x28, alpha: 0xFF },
        Pixel { blue: 0x39, green: 0x2B, red: 0x2F, alpha: 0xFF },
        Pixel { blue: 0x4D, green: 0x2B, red: 0x2B, alpha: 0xFF },
        Pixel { blue: 0x40, green: 0x2E, red: 0x26, alpha: 0xFF },
        Pixel { blue: 0x2D, green: 0x2F, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x32, green: 0x30, red: 0x2D, alpha: 0xFF },
        Pixel { blue: 0x51, green: 0x2E, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x34, green: 0x2D, red: 0x3A, alpha: 0xFF },
        Pixel { blue: 0x51, green: 0x2D, red: 0x33, alpha: 0xFF },
        Pixel { blue: 0x2B, green: 0x2E, red: 0x3D, alpha: 0xFF },
        Pixel { blue: 0x52, green: 0x32, red: 0x2A, alpha: 0xFF },
        Pixel { blue: 0x4E, green: 0x30, red: 0x35, alpha: 0xFF },
        Pixel { blue: 0x3B, green: 0x34, red: 0x2F, alpha: 0xFF },
        Pixel { blue: 0x40, green: 0x32, red: 0x36, alpha: 0xFF },
        Pixel { blue: 0x41, green: 0x36, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x5C, green: 0x30, red: 0x3A, alpha: 0xFF },
        Pixel { blue: 0x47, green: 0x35, red: 0x31, alpha: 0xFF },
        Pixel { blue: 0x37, green: 0x36, red: 0x33, alpha: 0xFF },
        Pixel { blue: 0x51, green: 0x36, red: 0x2C, alpha: 0xFF },
        Pixel { blue: 0x5D, green: 0x33, red: 0x36, alpha: 0xFF },
        Pixel { blue: 0x5D, green: 0x35, red: 0x32, alpha: 0xFF },
        Pixel { blue: 0x44, green: 0x37, red: 0x35, alpha: 0xFF },
        Pixel { blue: 0x4F, green: 0x38, red: 0x31, alpha: 0xFF },
        Pixel { blue: 0x5F, green: 0x3B, red: 0x24, alpha: 0xFF },
        Pixel { blue: 0x40, green: 0x39, red: 0x34, alpha: 0xFF },
        Pixel { blue: 0x38, green: 0x34, red: 0x47, alpha: 0xFF },
        Pixel { blue: 0x5F, green: 0x39, red: 0x2D, alpha: 0xFF },
        Pixel { blue: 0x46, green: 0x3B, red: 0x33, alpha: 0xFF },
        Pixel { blue: 0x4C, green: 0x3A, red: 0x36, alpha: 0xFF },
        Pixel { blue: 0x4B, green: 0x3E, red: 0x3B, alpha: 0xFF },
        Pixel { blue: 0x6B, green: 0x3F, red: 0x31, alpha: 0xFF },
        Pixel { blue: 0x65, green: 0x41, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x6B, green: 0x3E, red: 0x37, alpha: 0xFF },
        Pixel { blue: 0x5B, green: 0x40, red: 0x35, alpha: 0xFF },
        Pixel { blue: 0x6D, green: 0x3B, red: 0x42, alpha: 0xFF },
        Pixel { blue: 0x65, green: 0x3F, red: 0x38, alpha: 0xFF },
        Pixel { blue: 0x57, green: 0x40, red: 0x39, alpha: 0xFF },
        Pixel { blue: 0x52, green: 0x40, red: 0x3C, alpha: 0xFF },
        Pixel { blue: 0x38, green: 0x3B, red: 0x54, alpha: 0xFF },
        Pixel { blue: 0x6F, green: 0x3E, red: 0x3E, alpha: 0xFF },
        Pixel { blue: 0x41, green: 0x3C, red: 0x52, alpha: 0xFF },
        Pixel { blue: 0x42, green: 0x43, red: 0x40, alpha: 0xFF },
        Pixel { blue: 0x70, green: 0x46, red: 0x2F, alpha: 0xFF },
        Pixel { blue: 0x75, green: 0x45, red: 0x37, alpha: 0xFF },
        Pixel { blue: 0x62, green: 0x46, red: 0x3C, alpha: 0xFF },
        Pixel { blue: 0x4E, green: 0x40, red: 0x56, alpha: 0xFF },
        Pixel { blue: 0x5D, green: 0x46, red: 0x3F, alpha: 0xFF },
        Pixel { blue: 0x79, green: 0x49, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x4E, green: 0x45, red: 0x49, alpha: 0xFF },
        Pixel { blue: 0x6E, green: 0x45, red: 0x43, alpha: 0xFF },
        Pixel { blue: 0x7D, green: 0x4D, red: 0x32, alpha: 0xFF },
        Pixel { blue: 0x70, green: 0x4A, red: 0x42, alpha: 0xFF },
        Pixel { blue: 0x68, green: 0x4B, red: 0x41, alpha: 0xFF },
        Pixel { blue: 0x61, green: 0x4B, red: 0x43, alpha: 0xFF },
        Pixel { blue: 0x5E, green: 0x48, red: 0x4E, alpha: 0xFF },
        Pixel { blue: 0x53, green: 0x4E, red: 0x41, alpha: 0xFF },
        Pixel { blue: 0x6F, green: 0x4D, red: 0x3F, alpha: 0xFF },
        Pixel { blue: 0x81, green: 0x52, red: 0x2D, alpha: 0xFF },
        Pixel { blue: 0x81, green: 0x4E, red: 0x3A, alpha: 0xFF },
        Pixel { blue: 0x84, green: 0x4C, red: 0x43, alpha: 0xFF },
        Pixel { blue: 0x82, green: 0x52, red: 0x37, alpha: 0xFF },
        Pixel { blue: 0x65, green: 0x53, red: 0x3C, alpha: 0xFF },
        Pixel { blue: 0x89, green: 0x53, red: 0x33, alpha: 0xFF },
        Pixel { blue: 0x8A, green: 0x56, red: 0x2B, alpha: 0xFF },
        Pixel { blue: 0x45, green: 0x4A, red: 0x68, alpha: 0xFF },
        Pixel { blue: 0x50, green: 0x51, red: 0x4F, alpha: 0xFF },
        Pixel { blue: 0x6E, green: 0x51, red: 0x47, alpha: 0xFF },
        Pixel { blue: 0x7E, green: 0x51, red: 0x43, alpha: 0xFF },
        Pixel { blue: 0x74, green: 0x52, red: 0x45, alpha: 0xFF },
        Pixel { blue: 0x7A, green: 0x52, red: 0x48, alpha: 0xFF },
        Pixel { blue: 0x71, green: 0x54, red: 0x4A, alpha: 0xFF },
        Pixel { blue: 0x43, green: 0x4A, red: 0x77, alpha: 0xFF },
        Pixel { blue: 0x8F, green: 0x5A, red: 0x31, alpha: 0xFF },
        Pixel { blue: 0x89, green: 0x5B, red: 0x31, alpha: 0xFF },
        Pixel { blue: 0x90, green: 0x5A, red: 0x3A, alpha: 0xFF },
        Pixel { blue: 0x94, green: 0x55, red: 0x4D, alpha: 0xFF },
        Pixel { blue: 0x93, green: 0x60, red: 0x2B, alpha: 0xFF },
        Pixel { blue: 0x93, green: 0x59, red: 0x47, alpha: 0xFF },
        Pixel { blue: 0x94, green: 0x5F, red: 0x36, alpha: 0xFF },
        Pixel { blue: 0x6C, green: 0x54, red: 0x67, alpha: 0xFF },
        Pixel { blue: 0x88, green: 0x5E, red: 0x40, alpha: 0xFF },
        Pixel { blue: 0x9C, green: 0x62, red: 0x31, alpha: 0xFF },
        Pixel { blue: 0x5D, green: 0x53, red: 0x75, alpha: 0xFF },
        Pixel { blue: 0x5B, green: 0x5D, red: 0x5C, alpha: 0xFF },
        Pixel { blue: 0x65, green: 0x5C, red: 0x5D, alpha: 0xFF },
        Pixel { blue: 0x80, green: 0x5D, red: 0x54, alpha: 0xFF },
        Pixel { blue: 0x6D, green: 0x5E, red: 0x5E, alpha: 0xFF },
        Pixel { blue: 0xA1, green: 0x66, red: 0x38, alpha: 0xFF },
        Pixel { blue: 0x2F, green: 0x54, red: 0x93, alpha: 0xFF },
        Pixel { blue: 0xA3, green: 0x69, red: 0x30, alpha: 0xFF },
        Pixel { blue: 0x71, green: 0x66, red: 0x4A, alpha: 0xFF },
        Pixel { blue: 0x46, green: 0x58, red: 0x85, alpha: 0xFF },
        Pixel { blue: 0x9D, green: 0x68, red: 0x3A, alpha: 0xFF },
        Pixel { blue: 0x53, green: 0x5A, red: 0x7C, alpha: 0xFF },
        Pixel { blue: 0xA6, green: 0x6E, red: 0x27, alpha: 0xFF },
        Pixel { blue: 0x43, green: 0x55, red: 0x94, alpha: 0xFF },
        Pixel { blue: 0x55, green: 0x57, red: 0x89, alpha: 0xFF },
        Pixel { blue: 0x98, green: 0x68, red: 0x43, alpha: 0xFF },
        Pixel { blue: 0x79, green: 0x63, red: 0x5C, alpha: 0xFF },
        Pixel { blue: 0xAD, green: 0x6E, red: 0x2D, alpha: 0xFF },
        Pixel { blue: 0xA4, green: 0x65, red: 0x50, alpha: 0xFF },
        Pixel { blue: 0x69, green: 0x6D, red: 0x4C, alpha: 0xFF },
        Pixel { blue: 0xA1, green: 0x69, red: 0x4B, alpha: 0xFF },
        Pixel { blue: 0x64, green: 0x66, red: 0x66, alpha: 0xFF },
        Pixel { blue: 0x85, green: 0x66, red: 0x60, alpha: 0xFF },
        Pixel { blue: 0x48, green: 0x5A, red: 0xA0, alpha: 0xFF },
        Pixel { blue: 0x2F, green: 0x5A, red: 0xA7, alpha: 0xFF },
        Pixel { blue: 0xB2, green: 0x73, red: 0x33, alpha: 0xFF },
        Pixel { blue: 0xB5, green: 0x77, red: 0x2B, alpha: 0xFF },
        Pixel { blue: 0x5B, green: 0x68, red: 0x7E, alpha: 0xFF },
        Pixel { blue: 0x55, green: 0x60, red: 0x9A, alpha: 0xFF },
        Pixel { blue: 0x4B, green: 0x60, red: 0xA0, alpha: 0xFF },
        Pixel { blue: 0xBE, green: 0x7D, red: 0x2E, alpha: 0xFF },
        Pixel { blue: 0x2A, green: 0x61, red: 0xB5, alpha: 0xFF },
        Pixel { blue: 0x59, green: 0x67, red: 0x95, alpha: 0xFF },
        Pixel { blue: 0xA4, green: 0x75, red: 0x55, alpha: 0xFF },
        Pixel { blue: 0xAE, green: 0x78, red: 0x4A, alpha: 0xFF },
        Pixel { blue: 0x52, green: 0x68, red: 0x98, alpha: 0xFF },
        Pixel { blue: 0x76, green: 0x7A, red: 0x59, alpha: 0xFF },
        Pixel { blue: 0xB2, green: 0x76, red: 0x56, alpha: 0xFF },
        Pixel { blue: 0xB7, green: 0x74, red: 0x5C, alpha: 0xFF },
        Pixel { blue: 0x50, green: 0x66, red: 0xA7, alpha: 0xFF },
        Pixel { blue: 0x72, green: 0x74, red: 0x71, alpha: 0xFF },
        Pixel { blue: 0x5F, green: 0x66, red: 0xA4, alpha: 0xFF },
        Pixel { blue: 0x6B, green: 0x67, red: 0xA1, alpha: 0xFF },
        Pixel { blue: 0xB8, green: 0x7D, red: 0x47, alpha: 0xFF },
        Pixel { blue: 0x81, green: 0x7C, red: 0x60, alpha: 0xFF },
        Pixel { blue: 0x87, green: 0x77, red: 0x74, alpha: 0xFF },
        Pixel { blue: 0x54, green: 0x6A, red: 0xAC, alpha: 0xFF },
        Pixel { blue: 0x9F, green: 0x79, red: 0x71, alpha: 0xFF },
        Pixel { blue: 0x94, green: 0x7B, red: 0x71, alpha: 0xFF },
        Pixel { blue: 0x57, green: 0x6D, red: 0xAF, alpha: 0xFF },
        Pixel { blue: 0x59, green: 0x6F, red: 0xB1, alpha: 0xFF },
        Pixel { blue: 0x69, green: 0x6F, red: 0xB2, alpha: 0xFF },
        Pixel { blue: 0x2E, green: 0x6B, red: 0xCF, alpha: 0xFF },
        Pixel { blue: 0x61, green: 0x72, red: 0xAB, alpha: 0xFF },
        Pixel { blue: 0x5A, green: 0x73, red: 0xAE, alpha: 0xFF },
        Pixel { blue: 0x55, green: 0x74, red: 0xB1, alpha: 0xFF },
        Pixel { blue: 0x7E, green: 0x81, red: 0x7F, alpha: 0xFF },
        Pixel { blue: 0xC1, green: 0x89, red: 0x5E, alpha: 0xFF },
        Pixel { blue: 0xC4, green: 0x88, red: 0x6B, alpha: 0xFF },
        Pixel { blue: 0xCA, green: 0x87, red: 0x6E, alpha: 0xFF },
        Pixel { blue: 0x5B, green: 0x7A, red: 0xB7, alpha: 0xFF },
        Pixel { blue: 0x84, green: 0x87, red: 0x85, alpha: 0xFF },
        Pixel { blue: 0x7B, green: 0x7C, red: 0xAE, alpha: 0xFF },
        Pixel { blue: 0x56, green: 0x7D, red: 0xBC, alpha: 0xFF },
        Pixel { blue: 0x64, green: 0x7D, red: 0xBA, alpha: 0xFF },
        Pixel { blue: 0x9B, green: 0x8A, red: 0x88, alpha: 0xFF },
        Pixel { blue: 0x75, green: 0x7A, red: 0xC8, alpha: 0xFF },
        Pixel { blue: 0x2E, green: 0x76, red: 0xE9, alpha: 0xFF },
        Pixel { blue: 0x74, green: 0x7F, red: 0xB9, alpha: 0xFF },
        Pixel { blue: 0x7F, green: 0x7B, red: 0xC4, alpha: 0xFF },
        Pixel { blue: 0x70, green: 0x7D, red: 0xC5, alpha: 0xFF },
        Pixel { blue: 0x5A, green: 0x83, red: 0xBB, alpha: 0xFF },
        Pixel { blue: 0x51, green: 0x82, red: 0xC3, alpha: 0xFF },
        Pixel { blue: 0x8F, green: 0x96, red: 0x75, alpha: 0xFF },
        Pixel { blue: 0x90, green: 0x87, red: 0xA9, alpha: 0xFF },
        Pixel { blue: 0x9E, green: 0x95, red: 0x79, alpha: 0xFF },
        Pixel { blue: 0x73, green: 0x7F, red: 0xCF, alpha: 0xFF },
        Pixel { blue: 0x5F, green: 0x87, red: 0xC0, alpha: 0xFF },
        Pixel { blue: 0x55, green: 0x88, red: 0xC3, alpha: 0xFF },
        Pixel { blue: 0x71, green: 0x83, red: 0xD5, alpha: 0xFF },
        Pixel { blue: 0x5D, green: 0x8A, red: 0xC4, alpha: 0xFF },
        Pixel { blue: 0xD6, green: 0x97, red: 0x7A, alpha: 0xFF },
        Pixel { blue: 0x7E, green: 0x84, red: 0xD6, alpha: 0xFF },
        Pixel { blue: 0xAA, green: 0x91, red: 0xA4, alpha: 0xFF },
        Pixel { blue: 0xCF, green: 0x9C, red: 0x78, alpha: 0xFF },
        Pixel { blue: 0x71, green: 0x8B, red: 0xC9, alpha: 0xFF },
        Pixel { blue: 0xBF, green: 0x96, red: 0x92, alpha: 0xFF },
        Pixel { blue: 0xB3, green: 0x98, red: 0x91, alpha: 0xFF },
        Pixel { blue: 0xAD, green: 0x98, red: 0x94, alpha: 0xFF },
        Pixel { blue: 0x5A, green: 0x90, red: 0xC5, alpha: 0xFF },
        Pixel { blue: 0x60, green: 0x90, red: 0xC4, alpha: 0xFF },
        Pixel { blue: 0x33, green: 0x82, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0xB9, green: 0x99, red: 0x91, alpha: 0xFF },
        Pixel { blue: 0x2C, green: 0x84, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0x43, green: 0x83, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0xCD, green: 0x98, red: 0x98, alpha: 0xFF },
        Pixel { blue: 0xBD, green: 0x9B, red: 0x9A, alpha: 0xFF },
        Pixel { blue: 0x91, green: 0x8A, red: 0xDF, alpha: 0xFF },
        Pixel { blue: 0xBF, green: 0x9E, red: 0x93, alpha: 0xFF },
        Pixel { blue: 0xAE, green: 0x9D, red: 0xA2, alpha: 0xFF },
        Pixel { blue: 0x8A, green: 0x8D, red: 0xE4, alpha: 0xFF },
        Pixel { blue: 0xC6, green: 0x9F, red: 0x9A, alpha: 0xFF },
        Pixel { blue: 0x59, green: 0x89, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0xD7, green: 0xA2, red: 0x91, alpha: 0xFF },
        Pixel { blue: 0xE3, green: 0xA4, red: 0x8A, alpha: 0xFF },
        Pixel { blue: 0xD1, green: 0xA3, red: 0xA0, alpha: 0xFF },
        Pixel { blue: 0x81, green: 0x94, red: 0xE7, alpha: 0xFF },
        Pixel { blue: 0x84, green: 0x9A, red: 0xD6, alpha: 0xFF },
        Pixel { blue: 0xA7, green: 0xAF, red: 0x8A, alpha: 0xFF },
        Pixel { blue: 0x88, green: 0x95, red: 0xE8, alpha: 0xFF },
        Pixel { blue: 0x72, green: 0x90, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0x89, green: 0x98, red: 0xE0, alpha: 0xFF },
        Pixel { blue: 0x95, green: 0x95, red: 0xF0, alpha: 0xFF },
        Pixel { blue: 0xB7, green: 0xB0, red: 0x90, alpha: 0xFF },
        Pixel { blue: 0xD8, green: 0xA6, red: 0xAE, alpha: 0xFF },
        Pixel { blue: 0xDE, green: 0xA7, red: 0xAA, alpha: 0xFF },
        Pixel { blue: 0xE2, green: 0xA8, red: 0xB4, alpha: 0xFF },
        Pixel { blue: 0x9E, green: 0x9C, red: 0xF9, alpha: 0xFF },
        Pixel { blue: 0x8D, green: 0xA1, red: 0xF6, alpha: 0xFF },
        Pixel { blue: 0x93, green: 0xA1, red: 0xF5, alpha: 0xFF },
        Pixel { blue: 0x92, green: 0xA9, red: 0xE2, alpha: 0xFF },
        Pixel { blue: 0xB3, green: 0xBF, red: 0x9B, alpha: 0xFF },
        Pixel { blue: 0xAD, green: 0xA6, red: 0xFC, alpha: 0xFF },
        Pixel { blue: 0x9C, green: 0xAA, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0xCA, green: 0xC5, red: 0xA6, alpha: 0xFF },
        Pixel { blue: 0xA0, green: 0xB1, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0xED, green: 0xCA, red: 0x9A, alpha: 0xFF },
        Pixel { blue: 0xA3, green: 0xB7, red: 0xED, alpha: 0xFF },
        Pixel { blue: 0xC7, green: 0xCB, red: 0xA9, alpha: 0xFF },
        Pixel { blue: 0xA5, green: 0xB7, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0xC5, green: 0xD0, red: 0xAD, alpha: 0xFF },
        Pixel { blue: 0xAE, green: 0xC2, red: 0xF4, alpha: 0xFF },
        Pixel { blue: 0xB4, green: 0xC0, red: 0xFF, alpha: 0xFF },
        Pixel { blue: 0xB8, green: 0xC6, red: 0xF7, alpha: 0xFF },
        Pixel { blue: 0xBA, green: 0xCB, red: 0xF4, alpha: 0xFF },
    ];

    let original = open_ringhopper_image();
    let mut output = [0u8; 128*128];
    Format::Palletized(palette).encode_pixels(&original, &mut output, 128, 128);

    let mut new_pixels_lossy = [Pixel::default(); 128*128];
    Format::Palletized(palette).decode_pixels(&output, &mut new_pixels_lossy, 128, 128);

    let mut output_again = [0u8; 128*128];
    Format::Palletized(palette).encode_pixels(&new_pixels_lossy, &mut output_again, 128, 128);
    assert_eq!(output, output_again);
}
