pub mod generic;

use generic::*;

pub trait HashAlgorithm {
    fn hash(&self, data: Vec<u8>) -> Vec<u8>;
}

pub fn find_hash_for_name(name: &str) -> Option<&dyn HashAlgorithm> {
    match name {
        "blake2b" => Some(&Blake2bHasher {}),
        "blake2s" => Some(&Blake2sHasher {}),
        "md2" => Some(&Md2Hasher {}),
        "md4" => Some(&Md4Hasher {}),
        "md5" => Some(&Md5Hasher {}),
        "sha1" => Some(&Sha1Hasher {}),
        "sha224" => Some(&Sha224Hasher {}),
        "sha256" => Some(&Sha256Hasher {}),
        "sha384" => Some(&Sha384Hasher {}),
        "sha512" => Some(&Sha512Hasher {}),
        "keccak224" => Some(&Keccak224Hasher {}),
        "keccak256" => Some(&Keccak256Hasher {}),
        "keccak384" => Some(&Keccak384Hasher {}),
        "keccak512" => Some(&Keccak512Hasher {}),
        "sha3_224" => Some(&Sha3_224Hasher {}),
        "sha3_256" => Some(&Sha3_256Hasher {}),
        "sha3_384" => Some(&Sha3_384Hasher {}),
        "sha3_512" => Some(&Sha3_512Hasher {}),
        "streebog256" => Some(&Streebog256Hasher {}),
        "streebog512" => Some(&Streebog512Hasher {}),
        _ => None,
    }
}
