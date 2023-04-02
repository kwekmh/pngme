use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use std::str::FromStr;

pub fn encode(file_path: &String, chunk_type: &String, message: &String, output_file: &Option<String>) -> crate::Result<()> {
    let bytes = std::fs::read(file_path)?;
    let mut img = Png::try_from(&bytes[..])?;

    let chunk_type_instance = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type_instance, message.as_bytes().to_vec());

    img.append_chunk(chunk);

    if let Some(file_to_write) = output_file {
        std::fs::write(file_to_write,&img.as_bytes())?;
    } else {
        std::fs::write(file_path,&img.as_bytes())?;
    }

    Ok(())
}

pub fn decode(file_path: &String, chunk_type: &String) -> crate::Result<Option<String>> {
    let bytes = std::fs::read(file_path)?;
    let img = Png::try_from(&bytes[..])?;

    if let Some(chunk) = img.chunk_by_type(chunk_type) {
        Ok(Some(chunk.data_as_string()?))
    } else {
        Ok(None)
    }
}

pub fn remove(file_path: &String, chunk_type: &String) -> crate::Result<()> {
    let bytes = std::fs::read(file_path)?;
    let mut img = Png::try_from(&bytes[..])?;

    match img.remove_chunk(chunk_type) {
        Ok(chunk) => {
            println!("Removed chunk {}", chunk);
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn print(file_path: &String) -> crate::Result<()> {
    let bytes = std::fs::read(file_path)?;
    let img = Png::try_from(&bytes[..])?;

    for chunk in img.chunks() {
        println!("Chunk type: {}, Data: {}", chunk.chunk_type(), chunk);
    }

    Ok(())
}