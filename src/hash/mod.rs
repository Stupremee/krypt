pub trait HashAlgorithm {
    fn hash(&self, data: Vec<u8>) -> Vec<u8>;
}

pub fn hash_for_name(name: &str) -> Option<&dyn HashAlgorithm> {
    match name {
        _ => None,
    }
}
