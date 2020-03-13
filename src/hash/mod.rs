use crate::input::ChunkRead;
use clap::arg_enum;
use digest::Digest;
use std::io::Read;

arg_enum! {
    #[derive(Debug)]
    pub enum HashAlgorithm {
        Blake3,
        Md2,
        Md4,
        Md5,
        Sha1,
        Sha224,
        Sha256,
        Sha384,
        Sha512,
        Keccak224,
        Keccak256,
        Keccak384,
        Keccak512,
        Sha3_224,
        Sha3_256,
        Sha3_384,
        Sha3_512,
        Streebog256,
        Streebog512,
    }
}

fn generic_hash<D: Digest, R: Read>(data: ChunkRead<R>) -> Vec<u8> {
    let mut hasher = D::new();
    let mut item = data.next();
    while let Some(item) = item {
        hasher.input(item);
    }
    hasher.result().into_iter().collect::<Vec<u8>>()
}

pub fn hash_with_algorithm<R: Read>(alg: HashAlgorithm, data: ChunkRead<R>) -> Vec<u8> {
    match alg {
        HashAlgorithm::Blake3 => generic_hash::<blake3::Hasher, R>(data),
        HashAlgorithm::Md2 => generic_hash::<md2::Md2, R>(data),
        HashAlgorithm::Md4 => generic_hash::<md4::Md4, R>(data),
        HashAlgorithm::Md5 => generic_hash::<md5::Md5, R>(data),
        HashAlgorithm::Sha1 => generic_hash::<sha1::Sha1, R>(data),
        HashAlgorithm::Sha224 => generic_hash::<sha2::Sha224, R>(data),
        HashAlgorithm::Sha256 => generic_hash::<sha2::Sha256, R>(data),
        HashAlgorithm::Sha384 => generic_hash::<sha2::Sha384, R>(data),
        HashAlgorithm::Sha512 => generic_hash::<sha2::Sha512, R>(data),
        HashAlgorithm::Keccak224 => generic_hash::<sha3::Keccak224, R>(data),
        HashAlgorithm::Keccak256 => generic_hash::<sha3::Keccak256, R>(data),
        HashAlgorithm::Keccak384 => generic_hash::<sha3::Keccak384, R>(data),
        HashAlgorithm::Keccak512 => generic_hash::<sha3::Keccak512, R>(data),
        HashAlgorithm::Sha3_224 => generic_hash::<sha3::Sha3_224, R>(data),
        HashAlgorithm::Sha3_256 => generic_hash::<sha3::Sha3_256, R>(data),
        HashAlgorithm::Sha3_384 => generic_hash::<sha3::Sha3_384, R>(data),
        HashAlgorithm::Sha3_512 => generic_hash::<sha3::Sha3_512, R>(data),
        HashAlgorithm::Streebog256 => generic_hash::<streebog::Streebog256, R>(data),
        HashAlgorithm::Streebog512 => generic_hash::<streebog::Streebog512, R>(data),
    }
}
