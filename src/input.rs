use std::io::{self, prelude::*};

pub const CHUNK_SIZE: usize = 0x2000;

pub struct ChunkRead<R> {
    read: R,
    chunk: Vec<u8>,
    idx: usize,
}

impl<R> ChunkRead<R> {
    pub fn new(read: R) -> Self {
        Self {
            read,
            chunk: Vec::with_capacity(CHUNK_SIZE),
            idx: 0,
        }
    }
}

impl<R> Iterator for ChunkRead<R>
where
    R: Read,
{
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(CHUNK_SIZE);

        match self
            .read
            .by_ref()
            .take(CHUNK_SIZE as u64)
            .read_to_end(&mut chunk)
        {
            Ok(n) => {
                if n == 0 {
                    return None
                }
            }
            Err(e) => return Some(Err(e)),
        };
        std::mem::swap(&mut chunk, &mut self.chunk);
        Some(Ok(chunk))
    }
}
