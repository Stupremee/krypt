use digest::Digest;

pub const ALGORITHMS: [(&str, &dyn HashAlgorithm); 20] = [
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

macro_rules! generic_algorithm {
    ($arg:path, $name:ident) => {
        pub struct $name;
        impl $crate::hash::HashAlgorithm for $name {
            fn hash(&self, data: Vec<u8>) -> Vec<u8> {
                generic_hash::<$arg>(data)
            }
        }
    };
}

generic_algorithm!(blake2::Blake2b, Blake2bHasher);
generic_algorithm!(blake2::Blake2s, Blake2sHasher);
generic_algorithm!(md2::Md2, Md2Hasher);
generic_algorithm!(md4::Md4, Md4Hasher);
generic_algorithm!(md5::Md5, Md5Hasher);
generic_algorithm!(sha1::Sha1, Sha1Hasher);
generic_algorithm!(sha2::Sha224, Sha224Hasher);
generic_algorithm!(sha2::Sha256, Sha256Hasher);
generic_algorithm!(sha2::Sha384, Sha384Hasher);
generic_algorithm!(sha2::Sha512, Sha512Hasher);
generic_algorithm!(sha3::Keccak224, Keccak224Hasher);
generic_algorithm!(sha3::Keccak256, Keccak256Hasher);
generic_algorithm!(sha3::Keccak384, Keccak384Hasher);
generic_algorithm!(sha3::Keccak512, Keccak512Hasher);
generic_algorithm!(sha3::Sha3_224, Sha3_224Hasher);
generic_algorithm!(sha3::Sha3_256, Sha3_256Hasher);
generic_algorithm!(sha3::Sha3_384, Sha3_384Hasher);
generic_algorithm!(sha3::Sha3_512, Sha3_512Hasher);
generic_algorithm!(streebog::Streebog256, Streebog256Hasher);
generic_algorithm!(streebog::Streebog512, Streebog512Hasher);

fn generic_hash<D: Digest>(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = D::new();
    hasher.input(data);
    hasher.result().into_iter().collect::<Vec<u8>>()
}

pub fn find_hash_for_name(name: &str) -> Option<&dyn HashAlgorithm> {
    for pair in ALGORITHMS.iter() {
        if name.eq_ignore_ascii_case(pair.0) {
            return Some(pair.1);
        }
    }
    None
}
