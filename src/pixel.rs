macro_rules! convert_channel {
    ($from_bits:expr, $to_bits:expr, $channel:expr) => {
        if $from_bits == 0 || $to_bits == 0 {
            u8::MAX
        }
        else {
            const FROM: u16 = make_ones($from_bits) as u16;
            const TO: u16 = make_ones($to_bits) as u16;

            ((($channel as u16) * TO + (FROM / 2)) / FROM) as u8
        }
    }
}

macro_rules! convert_pixel {
    ($pixel:expr, $a1:expr, $r1:expr, $g1:expr, $b1:expr, $a2:expr, $r2:expr, $g2:expr, $b2:expr) => {{
        let alpha = convert_channel!($a1, $a2, $pixel.alpha);
        let red = convert_channel!($r1, $r2, $pixel.red);
        let green = convert_channel!($g1, $g2, $pixel.green);
        let blue = convert_channel!($b1, $b2, $pixel.blue);

        Pixel { alpha, red, green, blue }
    }};
}

macro_rules! split_pixel {
    ($pixel:expr, $a:expr, $r:expr, $g:expr, $b:expr) => {{
        const A_MASK: u8 = make_ones($a as usize);
        const R_MASK: u8 = make_ones($r as usize);
        const G_MASK: u8 = make_ones($g as usize);
        const B_MASK: u8 = make_ones($b as usize);

        Pixel {
            alpha: ((($pixel as u32) >> ($r + $g + $b)) as u8) & A_MASK,
            red: (($pixel >> ($g + $b)) as u8) & R_MASK,
            green: (($pixel >> ($b)) as u8) & G_MASK,
            blue: ($pixel as u8) & B_MASK
        }
    }};
}

macro_rules! blorp_pixel {
    ($pixel:expr, $a:expr, $r:expr, $g:expr, $b:expr, $pixel_type:ty) => {{
        ($pixel.alpha as $pixel_type) << (($r + $g + $b) as $pixel_type)
        | ($pixel.red as $pixel_type) << (($g + $b) as $pixel_type)
        | ($pixel.green as $pixel_type) << (($b) as $pixel_type)
        | ($pixel.blue as $pixel_type)
    }};
}

/// Defines a color with 8-bit alpha, red, green, and blue.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Pixel {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}
impl Pixel {
    /// Load a pixel from 8-bit A8.
    ///
    /// Red, green, and blue channels will be set to 255.
    pub const fn from_a8(a8: [u8; 1]) -> Self {
        Self {
            alpha: a8[0],
            red: 255,
            green: 255,
            blue: 255
        }
    }

    /// Encode the pixel as 8-bit A8.
    pub const fn as_a8(self) -> [u8; 1] {
        [self.alpha]
    }

    /// Load a pixel from 8-bit AY8.
    pub const fn from_ay8(ay8: [u8; 1]) -> Self {
        Self {
            alpha: ay8[0],
            red: ay8[0],
            green: ay8[0],
            blue: ay8[0]
        }
    }

    /// Encode the pixel as 8-bit AY8.
    ///
    /// This functionally does the same thing as [`as_a8()`](Pixel::as_a8).
    pub const fn as_ay8(self) -> [u8; 1] {
        [self.alpha]
    }

    /// Load a pixel from 8-bit Y8.
    ///
    /// The pixel will have 255 alpha.
    pub const fn from_y8(y8: [u8; 1]) -> Self {
        let [y8] = y8;
        Self {
            alpha: 255,
            red: y8,
            green: y8,
            blue: y8
        }
    }

    /// Encode the pixel as 8-bit Y8.
    ///
    /// Rec. 601 Luma will be used to convert non-monochrome colors to monochrome.
    ///
    /// `Y = 0.299 * self.red + 0.587 * self.green + 0.114 * self.blue`
    pub const fn as_y8(self) -> [u8; 1] {
        if self.red == self.blue && self.blue == self.green {
            return [self.blue]
        }

        let red = (299 * self.red as u32 + 255/2) / 255;
        let green = (587 * self.green as u32 + 255/2) / 255;
        let blue = (114 * self.blue as u32 + 255/2) / 255;

        let sum = red + green + blue;

        [(sum * 255 / 1000) as u8]
    }

    /// Load a pixel from 16-bit A8Y8 (little endian).
    pub const fn from_a8y8(a8y8: [u8; 2]) -> Self {
        Self {
            alpha: a8y8[1],
            .. Self::from_y8([a8y8[0]])
        }
    }

    /// Encode the pixel as 16-bit A8Y8 (little endian).
    pub const fn as_a8y8(self) -> [u8; 2] {
        let [y8] = self.as_y8();
        let [a8] = self.as_a8();
        [y8, a8]
    }

    /// Load a pixel from 16-bit R5G6B5 (little endian).
    ///
    /// The pixel will have 255 alpha.
    pub const fn from_r5g6b5(r5g6b5: [u8; 2]) -> Self {
        let pixel = u16::from_le_bytes(r5g6b5);
        let color = Self {
            alpha: u8::MAX,
            .. split_pixel!(pixel,0,5,6,5)
        };
        convert_pixel!(color,0,5,6,5,8,8,8,8)
    }

    /// Encode the pixel as 16-bit R5G6B5 (little endian).
    pub const fn as_r5g6b5(self) -> [u8; 2] {
        let color = convert_pixel!(self,8,8,8,8,0,5,6,5);
        (blorp_pixel!(color,0,5,6,5,u32) as u16).to_le_bytes()
    }

    /// Load a pixel from 16-bit A1R5G5B5 (little endian).
    pub const fn from_a1r5g5b5(a1r5g5b5: [u8; 2]) -> Self {
        let pixel = u16::from_le_bytes(a1r5g5b5);
        let color = split_pixel!(pixel,1,5,5,5);
        convert_pixel!(color,1,5,5,5,8,8,8,8)
    }

    /// Encode the pixel as 16-bit A1R5G5B5 (little endian).
    pub const fn as_a1r5g5b5(self) -> [u8; 2] {
        let color = convert_pixel!(self,8,8,8,8,1,5,5,5);
        blorp_pixel!(color,1,5,5,5,u16).to_le_bytes()
    }

    /// Load a pixel from 16-bit A4R4G4B4 (little endian).
    pub const fn from_a4r4g4b4(a4r4g4b4: [u8; 2]) -> Self {
        let pixel = u16::from_le_bytes(a4r4g4b4);
        let color = split_pixel!(pixel,4,4,4,4);
        convert_pixel!(color,4,4,4,4,8,8,8,8)
    }

    /// Encode the pixel as 16-bit A4R4G4B4 (little endian).
    pub const fn as_a4r4g4b4(self) -> [u8; 2] {
        let color = convert_pixel!(self,8,8,8,8,4,4,4,4);
        blorp_pixel!(color,4,4,4,4,u16).to_le_bytes()
    }

    /// Load a pixel from 32-bit X8R8G8B8 (little endian).
    ///
    /// The pixel will have 255 alpha.
    pub const fn from_x8r8g8b8(pixel: [u8; 4]) -> Self {
        let [blue, green, red, _] = pixel;
        Self {
            alpha: u8::MAX,
            red, green, blue
        }
    }

    /// Encode the pixel as 32-bit X8R8G8B8 (little endian).
    ///
    /// The alpha value of the pixel will be ignored.
    pub const fn as_x8r8g8b8(self) -> [u8; 4] {
        [self.blue, self.green, self.red, 0xFF]
    }

    /// Load a pixel from 32-bit A8R8G8B8 (little endian).
    pub const fn from_a8r8g8b8(bytes: [u8; 4]) -> Self {
        let [blue, green, red, alpha] = bytes;
        Self { alpha, red, green, blue }
    }

    /// Encode the pixel as 32-bit A8R8G8B8 (little endian).
    pub const fn as_a8r8g8b8(self) -> [u8; 4] {
        [self.blue, self.green, self.red, self.alpha]
    }

}

const fn make_ones(size: usize) -> u8 {
    match size {
        0 => 0b00000000,
        1 => 0b00000001,
        2 => 0b00000011,
        3 => 0b00000111,
        4 => 0b00001111,
        5 => 0b00011111,
        6 => 0b00111111,
        7 => 0b01111111,
        8 => 0b11111111,
        _ => panic!()
    }
}

#[cfg(test)]
mod tests;
