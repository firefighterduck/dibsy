use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("usage: dibsy FILE");
    let input = fs::read(&input_path)?;

    let (_, header) = bitmap_rs::DIBFile::parse(&input[..]).map_err(|_| "Nope".to_string())?;
    println!("The file so far: {:?}", header);

    Ok(())
}
