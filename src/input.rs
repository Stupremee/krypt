use std::io::{self, prelude::*};

const CHUNK_SIZE: usize = 0x2000;

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
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.chunk.get(self.idx);
        if let Some(byte) = byte {
            self.idx += 1;
            return Some(Ok(*byte));
        }

        self.chunk.clear();
        match self
            .read
            .by_ref()
            .take(CHUNK_SIZE as u64)
            .read_to_end(&mut self.chunk)
        {
            Ok(n) => {
                if n > 0 {
                    self.idx = 0;
                    self.next()
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}
