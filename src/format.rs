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

    /// 8-bit alpha, blue, green, and red
    A8B8G8R8,

    /// 8-bit palettized
    P8([Pixel; 256]),
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
            Format::A8B8G8R8 => 1,
            Format::P8(_) => 1
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
            Format::A8B8G8R8 => 4,
            Format::P8(_) => 1
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
            Format::P8(palette) => {
                encode_palettized(from_pixels.iter(), &palette)
                    .zip(to_bytes.iter_mut())
                    .for_each(|(input, output)| *output = input as u8)
            }

            // Simple conversion (uncompressed)
            _ => {
                let converter = match self {
                    Format::A8 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_a8()),
                    Format::Y8 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_y8()),
                    Format::AY8 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_ay8()),
                    Format::A8Y8 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_a8y8()),
                    Format::R5G6B5 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_r5g6b5()),
                    Format::A1R5G5B5 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_a1r5g5b5()),
                    Format::A4R4G4B4 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_a4r4g4b4()),
                    Format::X8R8G8B8 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_x8r8g8b8()),
                    Format::A8R8G8B8 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_a8r8g8b8()),
                    Format::A8B8G8R8 => |pixel: Pixel, to: &mut [u8]| to.copy_from_slice(&pixel.as_a8b8g8r8()),
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
            Format::P8(p) => {
                for (chunk, pixel) in from_bytes.iter().zip(to_pixels.iter_mut()) {
                    *pixel = p[*chunk as usize]
                }
            },

            // Simple conversion (uncompressed)
            _ => {
                let converter = match self {
                    Format::A8 => |from: &[u8]| Pixel::from_a8(from.try_into().unwrap()),
                    Format::Y8 => |from: &[u8]| Pixel::from_y8(from.try_into().unwrap()),
                    Format::AY8 => |from: &[u8]| Pixel::from_ay8(from.try_into().unwrap()),
                    Format::A8Y8 => |from: &[u8]| Pixel::from_a8y8(from.try_into().unwrap()),
                    Format::R5G6B5 => |from: &[u8]| Pixel::from_r5g6b5(from.try_into().unwrap()),
                    Format::A1R5G5B5 => |from: &[u8]| Pixel::from_a1r5g5b5(from.try_into().unwrap()),
                    Format::A4R4G4B4 => |from: &[u8]| Pixel::from_a4r4g4b4(from.try_into().unwrap()),
                    Format::X8R8G8B8 => |from: &[u8]| Pixel::from_x8r8g8b8(from.try_into().unwrap()),
                    Format::A8R8G8B8 => |from: &[u8]| Pixel::from_a8r8g8b8(from.try_into().unwrap()),
                    Format::A8B8G8R8 => |from: &[u8]| Pixel::from_a8b8g8r8(from.try_into().unwrap()),
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

/// Return an iterator that encodes one iterator of pixels into indices that correspond to a
/// given palette.
///
/// # Panics
///
/// Panics if `palette.is_empty()`
pub fn encode_palettized<'a, 'b, I: Iterator<Item = &'a Pixel> + 'a + 'b>(
    from_pixels: I,
    palette: &'b [Pixel]
) -> impl Iterator<Item = usize> + 'a + 'b where 'b: 'a {
    assert!(palette.len() > 0, "empty palette");

    let mut no_alpha = true;
    let mut one_bit_alpha = true;
    let mut alpha_255_exists = false;
    for i in palette {
        if i.alpha != 255 {
            no_alpha = false;
            if i.alpha != 0 {
                one_bit_alpha = false;
            }
        } else {
            alpha_255_exists = true;
        }
    }

    // If all pixels in the palette are 100% transparent, disregard the alpha when
    // encoding, because it doesn't make sense to check alpha when finding a pixel.
    if one_bit_alpha && alpha_255_exists {
        one_bit_alpha = false;
        no_alpha = true;
    }

    let get_distance = if one_bit_alpha || no_alpha {
        Pixel::distance_rgb
    } else {
        Pixel::distance_argb
    };

    from_pixels
        .map(move |pixel| {
            let mut output = None;
            let mut distance = u32::MAX;

            for i in 0..palette.len() {
                let palette_pixel = palette[i];

                // For one-bit alpha, consider alpha as binary rather than calculating the difference.
                if one_bit_alpha {
                    if pixel.alpha <= 127 && pixel.alpha == 255 {
                        continue;
                    }
                    if pixel.alpha > 127 && pixel.alpha == 0 {
                        continue;
                    }
                }

                let new_distance = get_distance(palette_pixel, pixel);

                if distance > new_distance {
                    distance = new_distance;
                    output = Some(i);
                }
            }

            output.expect("no pixels found")
        })
}

#[cfg(test)]
mod tests;
