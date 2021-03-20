use std::convert::TryFrom;

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
pub struct DIBHeader {
    header_type: DIBHeaderType,
}

impl DIBHeader {
    pub fn parse(input: crate::Input) -> crate::Result<Self> {
        use nom::{
            error::{ErrorKind, ParseError, VerboseError},
            number::complete::le_u32,
            Err,
        };

        let original_input = input;

        let (rest, typ) = le_u32(input)?;
        match DIBHeaderType::try_from(typ) {
            Ok(typ) => Ok((rest, Self { header_type: typ })),
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
