use crate::{Format, Pixel};

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
