use super::Pixel;

#[test]
pub fn white_to_white() {
    let white = Pixel { alpha: 255, red: 255, green: 255, blue: 255 };

    assert_eq!(u32::MAX, white.as_x8r8g8b8());
    assert_eq!(u32::MAX, white.as_a8r8g8b8());
    assert_eq!(u16::MAX, white.as_r5g6b5());
    assert_eq!(u16::MAX, white.as_a1r5g5b5());
    assert_eq!(u16::MAX, white.as_a4r4g4b4());
    assert_eq!(u8::MAX, white.as_a8());
    assert_eq!(u8::MAX, white.as_y8());

    assert_eq!(white, Pixel::from_x8r8g8b8(u32::MAX));
    assert_eq!(white, Pixel::from_a8r8g8b8(u32::MAX));
    assert_eq!(white, Pixel::from_r5g6b5(u16::MAX));
    assert_eq!(white, Pixel::from_a1r5g5b5(u16::MAX));
    assert_eq!(white, Pixel::from_a4r4g4b4(u16::MAX));
    assert_eq!(white, Pixel::from_a8(u8::MAX));
    assert_eq!(white, Pixel::from_y8(u8::MAX));
}

#[test]
pub fn black_to_black() {
    let black_opaque = Pixel { alpha: 255, red: 0, green: 0, blue: 0 };
    let black_transparent = Pixel { alpha: 0, red: 0, green: 0, blue: 0 };
    let white_transparent = Pixel { alpha: 0, red: 255, green: 255, blue: 255 };

    assert_eq!(0xFF000000, black_opaque.as_x8r8g8b8());
    assert_eq!(0xFF000000, black_opaque.as_a8r8g8b8());
    assert_eq!(0, black_opaque.as_r5g6b5());
    assert_eq!(0b1000000000000000, black_opaque.as_a1r5g5b5());
    assert_eq!(0b1111000000000000, black_opaque.as_a4r4g4b4());
    assert_eq!(0, black_opaque.as_y8());
    assert_eq!(0, black_transparent.as_y8());
    assert_eq!(0, black_transparent.as_a8());
    assert_eq!(255, black_opaque.as_a8());

    assert_eq!(0, white_transparent.as_a8());

    assert_eq!(0xFF000000, black_transparent.as_x8r8g8b8());
    assert_eq!(0, black_transparent.as_a8r8g8b8());
    assert_eq!(0, black_transparent.as_r5g6b5());
    assert_eq!(0, black_transparent.as_a1r5g5b5());
    assert_eq!(0, black_transparent.as_a4r4g4b4());
    assert_eq!(0, black_transparent.as_y8());

    assert_eq!(black_opaque, Pixel::from_x8r8g8b8(0));
    assert_eq!(black_opaque, Pixel::from_x8r8g8b8(0xFF000000));
    assert_eq!(black_opaque, Pixel::from_a8r8g8b8(0xFF000000));
    assert_eq!(black_opaque, Pixel::from_r5g6b5(0));
    assert_eq!(black_opaque, Pixel::from_a1r5g5b5(0b1000000000000000));
    assert_eq!(black_opaque, Pixel::from_a4r4g4b4(0b1111000000000000));
    assert_eq!(black_opaque, Pixel::from_y8(0));

    assert_eq!(black_transparent, Pixel::from_a8r8g8b8(0));
    assert_eq!(black_transparent, Pixel::from_a1r5g5b5(0));
    assert_eq!(black_transparent, Pixel::from_a4r4g4b4(0));
    assert_eq!(black_transparent, Pixel::from_a1r5g5b5(0));
    assert_eq!(black_transparent, Pixel::from_a4r4g4b4(0));

    assert_eq!(white_transparent, Pixel::from_a8(0));
}

#[test]
pub fn round_trip() {
    for i in 0..=65535 {
        assert_eq!(i, Pixel::from_r5g6b5(i).as_r5g6b5());
        assert_eq!(i, Pixel::from_a1r5g5b5(i).as_a1r5g5b5());
        assert_eq!(i, Pixel::from_a4r4g4b4(i).as_a4r4g4b4());
    }
}