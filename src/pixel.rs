macro_rules! convert_channel {
    ($from_bits:expr, $to_bits:expr, $channel:expr) => {
        if $from_bits == 0 || $to_bits == 0 {
            255
        }
        else {
            const FROM: u16 = make_ones($from_bits) as u16;
            const TO: u16 = make_ones($to_bits) as u16;

            let result = (((($channel as u16) & FROM) << 1) as u16) * (TO << 1) / (FROM << 1);
            let result_rounded_down = (result >> 1) as u8;

            // Round up?
            if (result & 1) != 0 {
                result_rounded_down + 1
            }
            else {
                result_rounded_down
            }
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pixel {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}
impl Pixel {
    pub const fn from_a8r8g8b8(color: u32) -> Self {
        Self {
            alpha: (color >> 24) as u8,
            red: (color >> 16) as u8,
            green: (color >> 8) as u8,
            blue: color as u8,
        }
    }
    pub const fn from_a8r8g8b8_bytes(bytes: [u8; 4]) -> Self {
        Self {
            alpha: bytes[0],
            red: bytes[1],
            green: bytes[2],
            blue: bytes[3],
        }
    }
    pub const fn from_b8g8r8a8(color: u32) -> Self {
        Self {
            alpha: color as u8,
            red: (color >> 8) as u8,
            green: (color >> 16) as u8,
            blue: (color >> 24) as u8,
        }
    }
    pub const fn from_b8g8r8a8_bytes(bytes: [u8; 4]) -> Self {
        Self {
            alpha: bytes[3],
            red: bytes[2],
            green: bytes[1],
            blue: bytes[0],
        }
    }
    pub const fn as_a8r8g8b8(self) -> u32 {
        (self.blue as u32)
            | ((self.green as u32) << 8)
            | ((self.red as u32) << 16)
            | ((self.alpha as u32) << 24)
    }
    pub const fn as_a8r8g8b8_bytes(self) -> [u8; 4] {
        [self.alpha, self.red, self.green, self.blue]
    }
    pub const fn as_b8g8r8a8(self) -> u32 {
        (self.alpha as u32)
            | ((self.red as u32) << 8)
            | ((self.green as u32) << 16)
            | ((self.blue as u32) << 24)
    }
    pub const fn as_b8g8r8a8_bytes(self) -> [u8; 4] {
        [self.blue, self.green, self.red, self.alpha]
    }

    pub const fn from_a8(a8: u8) -> Self {
        Self {
            alpha: a8,
            red: 255,
            green: 255,
            blue: 255
        }
    }
    pub const fn as_a8(self) -> u8 {
        self.alpha
    }

    pub const fn from_ay8(ay8: u8) -> Self {
        Self {
            alpha: ay8,
            red: ay8,
            green: ay8,
            blue: ay8
        }
    }
    pub const fn as_ay8(self) -> u8 {
        self.alpha
    }

    pub const fn from_a8y8(a8y8: u16) -> Self {
        Self {
            alpha: (a8y8 >> 8) as u8,
            .. Self::from_y8(a8y8 as u8)
        }
    }
    pub const fn as_a8y8(self) -> u16 {
        ((self.alpha as u16) << 8) | (self.as_y8() as u16)
    }

    pub const fn as_y8(self) -> u8 {
        self.blue
    }
    pub const fn from_y8(y8: u8) -> Self {
        Self {
            alpha: 255,
            red: y8,
            green: y8,
            blue: y8
        }
    }

    pub const fn from_r5g6b5(pixel: u16) -> Self {
        let color = Self {
            alpha: 255,
            .. split_pixel!(pixel,0,5,6,5)
        };
        convert_pixel!(color,0,5,6,5,8,8,8,8)
    }
    pub const fn as_r5g6b5(self) -> u16 {
        let color = convert_pixel!(self,8,8,8,8,0,5,6,5);
        blorp_pixel!(color,0,5,6,5,u32) as u16
    }

    pub const fn from_a1r5g5b5(pixel: u16) -> Self {
        let color = split_pixel!(pixel,1,5,5,5);
        convert_pixel!(color,1,5,5,5,8,8,8,8)
    }
    pub const fn as_a1r5g5b5(self) -> u16 {
        let color = convert_pixel!(self,8,8,8,8,1,5,5,5);
        blorp_pixel!(color,1,5,5,5,u16)
    }

    pub const fn from_a4r4g4b4(pixel: u16) -> Self {
        let color = split_pixel!(pixel,4,4,4,4);
        convert_pixel!(color,4,4,4,4,8,8,8,8)
    }
    pub const fn as_a4r4g4b4(self) -> u16 {
        let color = convert_pixel!(self,8,8,8,8,4,4,4,4);
        blorp_pixel!(color,4,4,4,4,u16)
    }

    pub const fn from_x8r8g8b8(pixel: u32) -> Self {
        Self {
            alpha: 255,
            .. split_pixel!(pixel, 8, 8, 8, 8)
        }
    }
    pub const fn as_x8r8g8b8(self) -> u32 {
        self.as_a8r8g8b8() | 0xFF000000
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
