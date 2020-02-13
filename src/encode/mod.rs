use clap::arg_enum;

pub fn encode_data(encoding: Encoding, data: Vec<u8>) -> Vec<u8> {
    encoding.encoding().encode(data.as_slice()).into_bytes()
}

pub fn decode_data(
    encoding: Encoding,
    data: Vec<u8>,
) -> Result<Vec<u8>, data_encoding::DecodeError> {
    encoding.encoding().decode(data.as_slice())
}

arg_enum! {
    #[derive(Debug)]
    pub enum Encoding {
        Hex,
        Base32,
        Base32Hex,
        Base64,
        Base64Url,
    }
}

impl Encoding {
    fn encoding(&self) -> data_encoding::Encoding {
        match self {
            Encoding::Hex => data_encoding::HEXUPPER_PERMISSIVE,
            Encoding::Base32 => data_encoding::BASE32,
            Encoding::Base32Hex => data_encoding::BASE32HEX,
            Encoding::Base64 => data_encoding::BASE64,
            Encoding::Base64Url => data_encoding::BASE64URL,
        }
    }
}
