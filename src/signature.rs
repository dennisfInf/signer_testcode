pub trait DigitalSignature {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, ErrorStack>;
    fn verify(&self, sig: &[u8], msg: &[u8]) -> Result<bool, ErrorStack>;
}

impl DigitalSignature for PKey<Private> {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let mut signer = Signer::new(MessageDigest::null(), self)?;
        signer.sign_oneshot_to_vec(data)
    }

    fn verify(&self, sig: &[u8], msg: &[u8]) -> Result<bool, ErrorStack> {
        let mut verifier = Verifier::new(MessageDigest::null(), self)?;
        verifier.verify_oneshot(sig, msg)
    }
}