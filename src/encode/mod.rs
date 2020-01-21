use data_encoding::{DecodeError, Encoding};

pub const ENCODINGS: [(&str, &dyn Encoder); 5] = [
    ("hex", &GenericEncoder::new(data_encoding::HEXLOWER)),
    ("base32", &GenericEncoder::new(data_encoding::BASE32)),
    ("base32hex", &GenericEncoder::new(data_encoding::BASE32HEX)),
    ("base64", &GenericEncoder::new(data_encoding::BASE64)),
    ("base64url", &GenericEncoder::new(data_encoding::BASE64URL)),
];

pub trait Encoder {
    fn encode(&self, data: Vec<u8>) -> String;
    fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>, DecodeError>;
}

pub struct GenericEncoder {
    encoding: Encoding,
}

impl GenericEncoder {
    pub const fn new(encoding: Encoding) -> Self {
        Self { encoding }
    }
}

impl Encoder for GenericEncoder {
    fn encode(&self, data: Vec<u8>) -> String {
        self.encoding.encode(data.as_slice())
    }

    fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>, DecodeError> {
        self.encoding.decode(data.as_slice())
    }
}

pub fn find_encoder_for_name(name: &str) -> Option<&dyn Encoder> {
    for pair in ENCODINGS.iter() {
        if name.eq_ignore_ascii_case(pair.0) {
            return Some(pair.1);
        }
    }
    None
}
