//! Implementation of [`Iterator`] that will read the data of a file
//! in chunks instead of reading the whole file into memory.
//!
//! [`Iterator`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html

use std::{
    collections::VecDeque,
    io::{self, Read},
};

/// The default size for a chunk of data.
///
/// The default size is 8 MiB
pub const CHUNK_SIZE: usize = 0x100000 * 8;

/// The implementation of a [`Iterator`] that will read data from [`Read`]
/// using chunks.
#[derive(Debug)]
pub struct ChunkRead<R: Read> {
    read: R,
    chunk: VecDeque<u8>,
    chunk_size: usize,
}

impl<R: Read> ChunkRead<R> {
    /// Creates a new `ChunkRead` that will read from the given `Read`.
    pub fn new(read: R) -> Self {
        Self {
            read,
            chunk: VecDeque::with_capacity(CHUNK_SIZE),
            chunk_size: CHUNK_SIZE,
        }
    }

    /// Creates a new `ChunkRead` that will read from the given `Read`
    /// in chunks of size `chunk_size`.
    pub fn with_chunk_size(chunk_size: usize, read: R) -> Self {
        Self {
            read,
            chunk_size,
            chunk: VecDeque::with_capacity(chunk_size),
        }
    }

    /// Unwraps this `ChunkRead` and returns the underlying `Reader`.
    pub fn into_inner(self) -> R {
        self.read
    }
}

impl<R: Read> Iterator for ChunkRead<R> {
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.chunk.pop_front() {
            Some(Ok(byte))
        } else {
            for byte in self.read.by_ref().bytes().take(CHUNK_SIZE) {
                match byte {
                    Ok(byte) => self.chunk.push_back(byte),
                    Err(err) => return Some(Err(err)),
                }
            }
            self.chunk.pop_front().map(|val| Ok(val))
        }
    }
}

impl<R: Read> Read for ChunkRead<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = buf.len();
        let iter = Iterator::take(self, len).zip(0..len);
        let mut read = 0;
        for (byte, idx) in iter {
            buf[idx] = byte?;
            read += 1;
        }
        Ok(read)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    #[test]
    fn chunk_read() {
        let data = vec![12u8, 13, 14, 15, 16, 17, 18, 19, 20];
        let read = ChunkRead::new(Cursor::new(data.clone()));

        let read = read.collect::<Result<Vec<_>, io::Error>>();
        assert!(read.is_ok());
        assert_eq!(data, read.unwrap());
    }

    #[test]
    fn small_chunk_size() {
        let data = vec![12u8, 13, 14, 15, 16, 17, 18, 19, 20];
        let read = ChunkRead::with_chunk_size(1, Cursor::new(data.clone()));

        let read = read.collect::<Result<Vec<_>, io::Error>>();
        assert!(read.is_ok());
        assert_eq!(data, read.unwrap());
    }
}
