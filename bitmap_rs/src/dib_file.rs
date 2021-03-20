use crate::{dib_header::DIBHeader, file_header::FileHeader};

#[derive(Debug)]
pub struct DIBFile {
    file_header: FileHeader,
    dib_header: DIBHeader,
}

impl DIBFile {
    pub fn parse(input: crate::Input) -> crate::Result<Self> {
        let (input, file_header) = FileHeader::parse(input)?;
        let (rest, dib_header) = DIBHeader::parse(input)?;
        Ok((
            rest,
            Self {
                file_header,
                dib_header,
            },
        ))
    }
}
