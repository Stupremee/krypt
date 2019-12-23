pub mod generic;

use generic::*;

pub(crate) const ALGORITHMS: [(&str, &dyn HashAlgorithm); 20] = [
    ("blake2b", &Blake2bHasher {}),
    ("blake2s", &Blake2sHasher {}),
    ("md2", &Md2Hasher {}),
    ("md4", &Md4Hasher {}),
    ("md5", &Md5Hasher {}),
    ("sha1", &Sha1Hasher {}),
    ("sha224", &Sha224Hasher {}),
    ("sha256", &Sha256Hasher {}),
    ("sha384", &Sha384Hasher {}),
    ("sha512", &Sha512Hasher {}),
    ("keccak224", &Keccak224Hasher {}),
    ("keccak256", &Keccak256Hasher {}),
    ("keccak384", &Keccak384Hasher {}),
    ("keccak512", &Keccak512Hasher {}),
    ("sha3_224", &Sha3_224Hasher {}),
    ("sha3_256", &Sha3_256Hasher {}),
    ("sha3_384", &Sha3_384Hasher {}),
    ("sha3_512", &Sha3_512Hasher {}),
    ("streebog256", &Streebog256Hasher {}),
    ("streebog512", &Streebog512Hasher {}),
];

pub trait HashAlgorithm {
    fn hash(&self, data: Vec<u8>) -> Vec<u8>;
}

pub fn find_hash_for_name(name: &str) -> Option<&dyn HashAlgorithm> {
    for pair in ALGORITHMS.iter() {
        if name.eq_ignore_ascii_case(pair.0) {
            return Some(pair.1);
        }
    }
    None
}
