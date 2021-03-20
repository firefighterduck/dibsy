use crate::{Input, Result};
use custom_debug_derive::Debug;

#[derive(Debug, Clone, Copy)]
enum Signature {
    #[debug(format = "BM")]
    BM,
    UnsupportedOS2,
}

impl Signature {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            branch::alt,
            combinator::{value, verify},
            number::complete::le_u8,
            sequence::pair,
        };

        let two_chars = |c1, c2, sig| {
            value(
                sig,
                pair(
                    verify(le_u8, move |c| char::from(*c) == c1),
                    verify(le_u8, move |c| char::from(*c) == c2),
                ),
            )
        };

        let (input, sig) = alt((
            two_chars('B', 'M', Self::BM),
            two_chars('B', 'A', Self::UnsupportedOS2),
            two_chars('C', 'I', Self::UnsupportedOS2),
            two_chars('C', 'P', Self::UnsupportedOS2),
            two_chars('I', 'C', Self::UnsupportedOS2),
            two_chars('P', 'T', Self::UnsupportedOS2),
        ))(input)?;
        Ok((input, sig))
    }
}

#[derive(Debug)]
pub struct FileHeader {
    signature: Signature,
    file_size: u32,
    file_offset_pixels: u32,
}

impl FileHeader {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            bytes::complete::take, error::context, number::complete::le_u32, sequence::tuple,
        };

        let (input, (signature, file_size, _, file_offset_pixels)) = tuple((
            context("Signature", Signature::parse),
            context("FileSize", le_u32),
            context("Padding", take(4_usize)),
            context("File Offset to PixelArry", le_u32),
        ))(input)?;
        Ok((
            input,
            Self {
                signature,
                file_size,
                file_offset_pixels,
            },
        ))
    }
}
