use digest::Digest;

pub fn generic_hash<D: Digest>(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = D::new();
    hasher.input(data);
    hasher.result().into_iter().collect::<Vec<u8>>()
}

pub fn hash_with_algorithm(alg: HashAlgorithm, data: Vec<u8>) -> Vec<u8> {
    match alg {
        HashAlgorithm::Blake3 => generic_hash::<blake3::Hasher>(data),
        HashAlgorithm::Md2 => generic_hash::<md2::Md2>(data),
        HashAlgorithm::Md4 => generic_hash::<md4::Md4>(data),
        HashAlgorithm::Md5 => generic_hash::<md5::Md5>(data),
        HashAlgorithm::Sha1 => generic_hash::<sha1::Sha1>(data),
        HashAlgorithm::Sha224 => generic_hash::<sha2::Sha224>(data),
        HashAlgorithm::Sha256 => generic_hash::<sha2::Sha256>(data),
        HashAlgorithm::Sha384 => generic_hash::<sha2::Sha384>(data),
        HashAlgorithm::Sha512 => generic_hash::<sha2::Sha512>(data),
        HashAlgorithm::Keccak224 => generic_hash::<sha3::Keccak224>(data),
        HashAlgorithm::Keccak256 => generic_hash::<sha3::Keccak256>(data),
        HashAlgorithm::Keccak384 => generic_hash::<sha3::Keccak384>(data),
        HashAlgorithm::Keccak512 => generic_hash::<sha3::Keccak512>(data),
        HashAlgorithm::Sha3_224 => generic_hash::<sha3::Sha3_224>(data),
        HashAlgorithm::Sha3_256 => generic_hash::<sha3::Sha3_256>(data),
        HashAlgorithm::Sha3_384 => generic_hash::<sha3::Sha3_384>(data),
        HashAlgorithm::Sha3_512 => generic_hash::<sha3::Sha3_512>(data),
        HashAlgorithm::Streebog256 => generic_hash::<streebog::Streebog256>(data),
        HashAlgorithm::Streebog512 => generic_hash::<streebog::Streebog512>(data),
    }
}

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
