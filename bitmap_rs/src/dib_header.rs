use std::convert::TryFrom;

use custom_debug_derive::Debug;
use derive_try_from_primitive::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum DIBHeaderType {
    BitmapCoreHeader = 12,
    BitmapCoreHeader2 = 64,
    BitmapInfoHeader = 40,
    BitmapV2InfoHeader = 52,
    BitMapV3InfoHeader = 56,
    BitmapV4InfoHeader = 108,
    BitmapV5InfoHeader = 124,
}

#[derive(Debug)]
pub struct BitmapInfoHeader {
    image_width: i32,
    image_height: i32,
    bits_per_pixel: u16,
    compression: u32,
    image_size: u32,
    /// Pixel per metre X
    horizontal_resolution: i32,
    /// Pixel per metre Y
    vertical_resolution: i32,
    colors: u32,
    #[debug(skip)]
    _important_colors: u32,
}

#[derive(Debug)]
pub enum DIBHeader {
    BitmapInfoHeader(BitmapInfoHeader),
    Unsupported(DIBHeaderType),
}

impl BitmapInfoHeader {
    pub fn parse(input: crate::Input) -> crate::Result<Self> {
        use nom::{
            combinator::{map, verify},
            error::context,
            number::complete::{le_i32, le_u16, le_u32},
        };

        let (input, image_width) = context("Image Width", le_i32)(input)?;
        let (input, image_height) = context("Image Height", le_i32)(input)?;
        let (input, _) =
            context("Color Planes (must be 1)", verify(le_u16, |num| *num == 1))(input)?;
        let (input, bits_per_pixel) = context("Bits per Pixel", le_u16)(input)?;
        let (input, compression) = context("Compression Method", le_u32)(input)?;
        let (input, image_size) = context(
            "Raw Image Size",
            verify(le_u32, |num| *num != 0 || compression == 0),
        )(input)?;
        let (input, horizontal_resolution) = context("Pixel per metre X", le_i32)(input)?;
        let (input, vertical_resolution) = context("Pixel per metre Y", le_i32)(input)?;
        let (input, colors) = context(
            "Number of used Colors",
            map(le_u32, |num| {
                if num == 0 {
                    2u32.pow(bits_per_pixel as u32)
                } else {
                    num
                }
            }),
        )(input)?;
        let (rest, _important_colors) = context("Number of important Colors", le_u32)(input)?;

        Ok((
            rest,
            Self {
                image_width,
                image_height,
                bits_per_pixel,
                compression,
                image_size,
                horizontal_resolution,
                vertical_resolution,
                colors,
                _important_colors,
            },
        ))
    }
}

impl DIBHeader {
    pub fn parse(input: crate::Input) -> crate::Result<Self> {
        use nom::{
            bytes::complete::take,
            error::{context, ErrorKind, ParseError, VerboseError},
            number::complete::le_u32,
            Err,
        };

        let original_input = input;

        let (input, r#type) = context("DIB Header Type", le_u32)(input)?;
        match DIBHeaderType::try_from(r#type) {
            Ok(r#type) => {
                if let DIBHeaderType::BitmapInfoHeader = r#type {
                    let (rest, bitmap_info_header) = BitmapInfoHeader::parse(input)?;
                    Ok((rest, Self::BitmapInfoHeader(bitmap_info_header)))
                } else {
                    let (rest, _) = take(36usize)(input)?;
                    Ok((rest, Self::Unsupported(r#type)))
                }
            }
            Err(_) => Err(Err::Failure(VerboseError::from_error_kind(
                original_input,
                ErrorKind::Alt,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DIBHeaderType;
    use std::convert::TryFrom;

    #[test]
    fn try_enums() {
        assert_eq!(DIBHeaderType::BitmapCoreHeader as u32, 12);
        assert_eq!(
            DIBHeaderType::try_from(40),
            Ok(DIBHeaderType::BitmapInfoHeader)
        );
        assert_eq!(DIBHeaderType::try_from(48), Err(48));
    }
}
