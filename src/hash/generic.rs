use digest::Digest;

macro_rules! generic_algorithm {
    ($arg:path, $name:ident) => {
        pub struct $name;
        impl super::HashAlgorithm for $name {
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
