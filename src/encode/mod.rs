pub trait Encoder {
    fn encode(&self, data: Vec<u8>) -> String;
    fn decode(&self, data: String) -> Vec<u8>;
}
