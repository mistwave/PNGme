use std::path::Path;
use std::str::FromStr;
use crate::args::Commands;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;


impl Commands {
    pub fn encode<P: AsRef<Path>>(path: P, chunk_type: Option<String>, message: Option<String>) -> Result<Png> {
        let mut png = Png::from_file(path)?;
        png.append_chunk(
            Chunk::new(
                ChunkType::from_str(&chunk_type.unwrap())?,
                message.unwrap().into_bytes(),
            )
        );
        Ok(png)
    }
    pub fn decode<P: AsRef<Path>>(path: P, chunk_type: Option<String>) -> Option<String> {
        let png = Png::from_file(path).ok()?;
        png.chunk_by_type(&chunk_type.unwrap())
            .and_then(|chunk| chunk.data_as_string().ok())
    }
    pub fn remove<P: AsRef<Path>>(path: P, chunk_type: Option<String>) -> Result<Png> {
        let mut png = Png::from_file(path)?;
        png.remove_chunk(&chunk_type.unwrap())?;
        Ok(png)
    }
    pub fn print<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
        let png = Png::from_file(path)?;
        Ok(
            png.chunks().iter()
                .map(|chunk| chunk.chunk_type().to_string())
                .collect()
        )
    }
}