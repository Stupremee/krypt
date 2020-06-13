//! Contains all stuff that is required for the Hash mode operations.
#![allow(missing_docs)]

use crate::{chunk::ChunkRead, Result};
use digest::Digest;
use std::io::{Read, Write};
use structopt::clap::arg_enum;

arg_enum! {
    /// All hash algorithms that are supported by krypt.
    #[derive(Debug)]
    pub enum HashAlgorithm {
        Blake2b,
        Blake2s,

        Blake3,

        Md2,
        Md4,
        Md5,

        Sha1,

        Sha224,
        Sha256,
        Sha384,
        Sha512,

        Sha3_224,
        Sha3_256,
        Sha3_384,
        Sha3_512,

        Keccak224,
        Keccak256,
        Keccak384,
        Keccak512,

        Shabal192,
        Shabal224,
        Shabal256,
        Shabal384,
        Shabal512,

        Streebog256,
        Streebog512,

        Whirlpool,
    }
}

fn generic_hash<D: Digest, R: Read, W: Write>(
    data: &mut ChunkRead<R>,
    output: &mut W,
) -> Result<()> {
    let mut hasher = D::new();

    while let Some(chunk) = data.next() {
        hasher.update(chunk?);
    }

    let result = hasher.finalize();
    // FIXME: this is probably a very bad way
    // to copy the data but it works for now.
    output.write(result.as_slice())?;
    Ok(())
}

pub fn hash_with_algorithm<R: Read, W: Write>(
    alg: HashAlgorithm,
    data: &mut ChunkRead<R>,
    output: &mut W,
) -> Result<()> {
    match alg {
        HashAlgorithm::Blake2b => generic_hash::<blake2::Blake2b, R, W>(data, output),
        HashAlgorithm::Blake2s => generic_hash::<blake2::Blake2s, R, W>(data, output),
        HashAlgorithm::Blake3 => generic_hash::<blake3::Hasher, R, W>(data, output),
        HashAlgorithm::Md2 => generic_hash::<md2::Md2, R, W>(data, output),
        HashAlgorithm::Md4 => generic_hash::<md4::Md4, R, W>(data, output),
        HashAlgorithm::Md5 => generic_hash::<md5::Md5, R, W>(data, output),
        HashAlgorithm::Sha1 => generic_hash::<sha1::Sha1, R, W>(data, output),
        HashAlgorithm::Sha224 => generic_hash::<sha2::Sha224, R, W>(data, output),
        HashAlgorithm::Sha256 => generic_hash::<sha2::Sha256, R, W>(data, output),
        HashAlgorithm::Sha384 => generic_hash::<sha2::Sha384, R, W>(data, output),
        HashAlgorithm::Sha512 => generic_hash::<sha2::Sha512, R, W>(data, output),
        HashAlgorithm::Sha3_224 => generic_hash::<sha3::Sha3_224, R, W>(data, output),
        HashAlgorithm::Sha3_256 => generic_hash::<sha3::Sha3_256, R, W>(data, output),
        HashAlgorithm::Sha3_384 => generic_hash::<sha3::Sha3_384, R, W>(data, output),
        HashAlgorithm::Sha3_512 => generic_hash::<sha3::Sha3_512, R, W>(data, output),
        HashAlgorithm::Keccak224 => generic_hash::<sha3::Keccak224, R, W>(data, output),
        HashAlgorithm::Keccak256 => generic_hash::<sha3::Keccak256, R, W>(data, output),
        HashAlgorithm::Keccak384 => generic_hash::<sha3::Keccak384, R, W>(data, output),
        HashAlgorithm::Keccak512 => generic_hash::<sha3::Keccak512, R, W>(data, output),
        HashAlgorithm::Shabal192 => generic_hash::<shabal::Shabal192, R, W>(data, output),
        HashAlgorithm::Shabal224 => generic_hash::<shabal::Shabal224, R, W>(data, output),
        HashAlgorithm::Shabal256 => generic_hash::<shabal::Shabal256, R, W>(data, output),
        HashAlgorithm::Shabal384 => generic_hash::<shabal::Shabal384, R, W>(data, output),
        HashAlgorithm::Shabal512 => generic_hash::<shabal::Shabal512, R, W>(data, output),
        HashAlgorithm::Streebog256 => generic_hash::<streebog::Streebog256, R, W>(data, output),
        HashAlgorithm::Streebog512 => generic_hash::<streebog::Streebog512, R, W>(data, output),
        HashAlgorithm::Whirlpool => generic_hash::<whirlpool::Whirlpool, R, W>(data, output),
    }
}
