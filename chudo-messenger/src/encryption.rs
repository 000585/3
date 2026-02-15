use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use x25519_dalek::{PublicKey, StaticSecret};
use sha2::{Sha256, Digest};
use hkdf::Hkdf;
use anyhow::Result;

pub struct CHUDOCipher {
    static_secret: StaticSecret,
    public_key: PublicKey,
}

pub struct RatchetState {
    pub root_key: [u8; 32],
    pub sending_chain: [u8; 32],
    pub receiving_chain: [u8; 32],
    pub sending_message_number: u64,
    pub receiving_message_number: u64,
}

impl CHUDOCipher {
    pub fn generate() -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);
        Self {
            static_secret: secret,
            public_key: public,
        }
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public_key.to_bytes()
    }

    pub fn initialize_ratchet(
        &self,
        their_public: [u8; 32],
        is_initiator: bool
    ) -> Result<RatchetState> {
        let their_pk = PublicKey::from(their_public);
        let shared_secret = self.static_secret.diffie_hellman(&their_pk);
        
        let hkdf = Hkdf::<Sha256>::new(None, shared_secret.as_bytes());
        let mut root_key = [0u8; 32];
        hkdf.expand(b"CHUDO-v1-RootKey", &mut root_key)
            .map_err(|e| anyhow::anyhow!("HKDF failed: {}", e))?;

        let hkdf_chain = Hkdf::<Sha256>::new(None, &root_key);
        let mut chain_keys = [0u8; 64];
        hkdf_chain.expand(b"CHUDO-v1-ChainKeys", &mut chain_keys)
            .map_err(|e| anyhow::anyhow!("HKDF chain failed: {}", e))?;

        let (sending, receiving) = if is_initiator {
            let mut s = [0u8; 32];
            let mut r = [0u8; 32];
            s.copy_from_slice(&chain_keys[0..32]);
            r.copy_from_slice(&chain_keys[32..64]);
            (s, r)
        } else {
            let mut s = [0u8; 32];
            let mut r = [0u8; 32];
            r.copy_from_slice(&chain_keys[0..32]);
            s.copy_from_slice(&chain_keys[32..64]);
            (s, r)
        };

        Ok(RatchetState {
            root_key,
            sending_chain: sending,
            receiving_chain: receiving,
            sending_message_number: 0,
            receiving_message_number: 0,
        })
    }
}
