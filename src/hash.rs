//! Contains all stuff that is required for the Hash mode operations.
#![allow(missing_docs)]

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
