use ring::{
    rand,
    signature::{self,Ed25519KeyPair}
};

pub(crate) struct AppState {
    pub(crate) key_pair:Ed25519KeyPair,
}

impl AppState {
    pub(crate) async fn generate() -> Self {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

        AppState {
            key_pair
        }
    }
}
