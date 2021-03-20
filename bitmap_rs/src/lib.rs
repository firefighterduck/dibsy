mod dib_file;
mod dib_header;
mod file_header;

pub use dib_file::DIBFile;

pub type Input<'a> = &'a [u8];
pub type Result<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;
