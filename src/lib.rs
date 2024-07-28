#![no_std]

pub use crate::pixel::Pixel;

mod pixel;

/// Defines formats supported by the compressor/decompressor.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Formats {
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

impl Formats {
    /// Size of each block in pixels (in width and height).
    ///
    /// For compressed textures utilizing block compression, the size will be rounded *up* to the
    /// nearest block. For example, with DXT1, a 16x17 texture will be compressed as 16x20, and
    /// the extra 3 pixels will be undetermined but valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use macaroni_tex::Formats;
    ///
    /// assert_eq!(4, Formats::BC7.block_size_pixels());
    /// assert_eq!(1, Formats::A1R5G5B5.block_size_pixels());
    /// ```
    pub const fn block_size_pixels(self) -> usize {
        match self {
            Formats::DXT1 => 4,
            Formats::DXT3 => 4,
            Formats::DXT5 => 4,
            Formats::BC7 => 4,
            Formats::A8 => 1,
            Formats::Y8 => 1,
            Formats::AY8 => 1,
            Formats::A8Y8 => 1,
            Formats::R5G6B5 => 1,
            Formats::A1R5G5B5 => 1,
            Formats::A4R4G4B4 => 1,
            Formats::X8R8G8B8 => 1,
            Formats::A8R8G8B8 => 1,
            Formats::Palettized(_) => 1
        }
    }

    /// Number of bytes each block takes up.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use macaroni_tex::Formats;
    ///
    /// assert_eq!(16, Formats::BC7.block_size_bytes());
    /// assert_eq!(2, Formats::A1R5G5B5.block_size_bytes());
    /// ```
    pub const fn block_size_bytes(self) -> usize {
        match self {
            Formats::DXT1 => 8,
            Formats::DXT3 => 16,
            Formats::DXT5 => 16,
            Formats::BC7 => 16,
            Formats::A8 => 1,
            Formats::Y8 => 1,
            Formats::AY8 => 1,
            Formats::A8Y8 => 2,
            Formats::R5G6B5 => 2,
            Formats::A1R5G5B5 => 2,
            Formats::A4R4G4B4 => 2,
            Formats::X8R8G8B8 => 4,
            Formats::A8R8G8B8 => 4,
            Formats::Palettized(_) => 1
        }
    }
}
