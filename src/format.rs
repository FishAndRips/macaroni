use crate::Pixel;

/// Defines formats supported by the compressor/decompressor.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Format {
    /// DXT block compression with optional 1-bit alpha
    DXT1,

    /// DXT block compression with 4-bit explicit alpha
    DXT3,

    /// DXT block compression with interpolated alpha
    DXT5,

    /// BC7 block compression with optional alpha
    BC7,

    /// 8-bit alpha (100% white)
    A8,

    /// 8-bit luminosity (100% opaque)
    Y8,

    /// 8-bit alpha-luminosity (alpha=luminosity)
    AY8,

    /// 8-bit alpha with 8-bit luminosity
    A8Y8,

    /// 5-bit red, 6-bit green, and 5-bit blue (100% opaque)
    R5G6B5,

    /// 1-bit alpha, 5-bit red, green, and blue
    A1R5G5B5,

    /// 4-bit alpha, red, green, and blue
    A4R4G4B4,

    /// 8-bit red, green, and blue (100% opaque)
    X8R8G8B8,

    /// 8-bit alpha, red, green, and blue
    A8R8G8B8,

    /// 8-bit palettized
    Palettized([Pixel; 256]),
}

impl Format {
    /// Size of each block in pixels (in width and height).
    ///
    /// For compressed textures utilizing block compression, the size will be rounded *up* to the
    /// nearest block. For example, with DXT1, a 16x17 texture will be compressed as 16x20, and
    /// the extra 3 pixels will be undetermined but valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use macaroni_tex::Format;
    ///
    /// assert_eq!(4, Format::BC7.block_size_pixels());
    /// assert_eq!(1, Format::A1R5G5B5.block_size_pixels());
    /// ```
    pub const fn block_size_pixels(self) -> usize {
        match self {
            Format::DXT1 => 4,
            Format::DXT3 => 4,
            Format::DXT5 => 4,
            Format::BC7 => 4,
            Format::A8 => 1,
            Format::Y8 => 1,
            Format::AY8 => 1,
            Format::A8Y8 => 1,
            Format::R5G6B5 => 1,
            Format::A1R5G5B5 => 1,
            Format::A4R4G4B4 => 1,
            Format::X8R8G8B8 => 1,
            Format::A8R8G8B8 => 1,
            Format::Palettized(_) => 1
        }
    }

    /// Number of bytes each block takes up.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use macaroni_tex::Format;
    ///
    /// assert_eq!(16, Format::BC7.block_size_bytes());
    /// assert_eq!(2, Format::A1R5G5B5.block_size_bytes());
    /// ```
    pub const fn block_size_bytes(self) -> usize {
        match self {
            Format::DXT1 => 8,
            Format::DXT3 => 16,
            Format::DXT5 => 16,
            Format::BC7 => 16,
            Format::A8 => 1,
            Format::Y8 => 1,
            Format::AY8 => 1,
            Format::A8Y8 => 2,
            Format::R5G6B5 => 2,
            Format::A1R5G5B5 => 2,
            Format::A4R4G4B4 => 2,
            Format::X8R8G8B8 => 4,
            Format::A8R8G8B8 => 4,
            Format::Palettized(_) => 1
        }
    }

    /// Get the number of bytes required to represent height*width with the format.
    ///
    /// # Panics
    ///
    /// This function will panic if the result exceeds [usize::MAX].
    pub const fn required_bytes(self, width: usize, height: usize) -> usize {
        let block_size_bytes = self.block_size_bytes();
        let block_size_pixels = self.block_size_pixels();
        let blocks_height = height.div_ceil(block_size_pixels);
        let blocks_width = width.div_ceil(block_size_pixels);

        let Some(block_count) = blocks_height.checked_mul(blocks_width) else {
            panic!("required_bytes(): blocks_height*block_width overflows usize")
        };

        let Some(bytes) = block_count.checked_mul(block_size_bytes) else {
            panic!("required_bytes(): total bytes overflows usize")
        };

        bytes
    }

    /// Convert pixels from 8-bit to the output format.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// * `to_bytes.len() != self.requires_bytes(width, height)`
    /// * `from_pixels.len() != width*height`
    pub fn encode_pixels(
        self,
        from_pixels: &[Pixel],
        to_bytes: &mut [u8],
        width: usize,
        height: usize
    ) {
        assert_eq!(self.required_bytes(width, height), to_bytes.len());
        assert_eq!(width*height, from_pixels.len());

        match self {
            // Compressed formats
            Format::DXT1 => todo!(),
            Format::DXT3 => todo!(),
            Format::DXT5 => todo!(),
            Format::BC7 => todo!(),

            // Palettized
            Format::Palettized(_) => todo!(),

            // Simple conversion (uncompressed)
            _ => {
                let converter = match self {
                    Format::A8 => |pixel: Pixel, to: &mut [u8]| {
                        to[0] = pixel.as_a8();
                    },
                    Format::Y8 => |pixel: Pixel, to: &mut [u8]| {
                        to[0] = pixel.as_y8();
                    },
                    Format::AY8 => |pixel: Pixel, to: &mut [u8]| {
                        to[0] = pixel.as_ay8();
                    },
                    Format::A8Y8 => |pixel: Pixel, to: &mut [u8]| {
                        let bytes = pixel.as_a8y8().to_le_bytes();
                        to.copy_from_slice(&bytes);
                    },
                    Format::R5G6B5 => |pixel: Pixel, to: &mut [u8]| {
                        let bytes = pixel.as_r5g6b5().to_le_bytes();
                        to.copy_from_slice(&bytes);
                    },
                    Format::A1R5G5B5 => |pixel: Pixel, to: &mut [u8]| {
                        let bytes = pixel.as_a1r5g5b5().to_le_bytes();
                        to.copy_from_slice(&bytes);
                    },
                    Format::A4R4G4B4 => |pixel: Pixel, to: &mut [u8]| {
                        let bytes = pixel.as_a4r4g4b4().to_le_bytes();
                        to.copy_from_slice(&bytes);
                    },
                    Format::X8R8G8B8 => |pixel: Pixel, to: &mut [u8]| {
                        let bytes = pixel.as_x8r8g8b8().to_le_bytes();
                        to.copy_from_slice(&bytes);
                    },
                    Format::A8R8G8B8 => |pixel: Pixel, to: &mut [u8]| {
                        let bytes = pixel.as_a8r8g8b8().to_le_bytes();
                        to.copy_from_slice(&bytes);
                    },
                    n => unreachable!("can't convert {:?}", n)
                };

                let bytes_per_block = self.block_size_bytes();
                for (chunk, pixel) in to_bytes.chunks_exact_mut(bytes_per_block).zip(from_pixels.iter()) {
                    converter(*pixel, chunk);
                }
            }
        }
    }

    /// Convert pixels from 8-bit to the output format.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// * `to_bytes.len() != self.requires_bytes(width, height)`
    /// * `pixels.len() != width*height`
    pub fn decode_pixels(
        self,
        from_bytes: &[u8],
        to_pixels: &mut [Pixel],
        width: usize,
        height: usize
    ) {
        assert_eq!(self.required_bytes(width, height), from_bytes.len());
        assert_eq!(width*height, to_pixels.len());

        match self {
            // Compressed formats
            Format::DXT1 => todo!(),
            Format::DXT3 => todo!(),
            Format::DXT5 => todo!(),
            Format::BC7 => todo!(),

            // Palettized
            Format::Palettized(_) => todo!(),

            // Simple conversion (uncompressed)
            _ => {
                let converter = match self {
                    Format::A8 => |from: &[u8]| -> Pixel {
                        Pixel::from_a8(from[0])
                    },
                    Format::Y8 => |from: &[u8]| -> Pixel {
                        Pixel::from_y8(from[0])
                    },
                    Format::AY8 => |from: &[u8]| -> Pixel {
                        Pixel::from_ay8(from[0])
                    },
                    Format::A8Y8 => |from: &[u8]| -> Pixel {
                        Pixel::from_a8y8(u16::from_le_bytes(from.try_into().unwrap()))
                    },
                    Format::R5G6B5 => |from: &[u8]| -> Pixel {
                        Pixel::from_r5g6b5(u16::from_le_bytes(from.try_into().unwrap()))
                    },
                    Format::A1R5G5B5 => |from: &[u8]| -> Pixel {
                        Pixel::from_a1r5g5b5(u16::from_le_bytes(from.try_into().unwrap()))
                    },
                    Format::A4R4G4B4 => |from: &[u8]| -> Pixel {
                        Pixel::from_a4r4g4b4(u16::from_le_bytes(from.try_into().unwrap()))
                    },
                    Format::X8R8G8B8 => |from: &[u8]| -> Pixel {
                        Pixel::from_x8r8g8b8(u32::from_le_bytes(from.try_into().unwrap()))
                    },
                    Format::A8R8G8B8 => |from: &[u8]| -> Pixel {
                        Pixel::from_a8r8g8b8(u32::from_le_bytes(from.try_into().unwrap()))
                    },
                    n => unreachable!("can't convert {:?}", n)
                };

                let bytes_per_block = self.block_size_bytes();
                for (chunk, pixel) in from_bytes.chunks_exact(bytes_per_block).zip(to_pixels.iter_mut()) {
                    *pixel = converter(chunk);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
