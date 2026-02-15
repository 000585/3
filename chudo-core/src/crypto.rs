use sha3::{Digest, Sha3_256};
use serde::Serialize;
use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey};
use rand::{rngs::OsRng, RngCore};

// --- ??????? ??????????? (SHA3) ---
pub fn hash<T: Serialize + ?Sized>(data: &T) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    let encoded = bincode::serialize(data).expect("Failed to serialize data");
    hasher.update(&encoded);
    hasher.finalize().into()
}

// --- kHeavyHash (????????? ????????) ---
pub fn k_heavy_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let mut state = hasher.finalize().to_vec();

    for i in 0..64 {
        for j in 0..state.len() {
             let next_val = state[(j + 1) % state.len()];
             state[j] = state[j].wrapping_add(next_val ^ (i as u8));
        }
    }

    let mut final_hasher = Sha3_256::new();
    final_hasher.update(&state);
    final_hasher.finalize().to_vec()
}

// --- ?????? ? ??????? ---
#[derive(Debug)]
pub struct KeyPair {
    key: SigningKey,
}

impl KeyPair {
    pub fn random() -> Self {
        let mut csprng = OsRng;
        let mut bytes = [0u8; 32];
        csprng.fill_bytes(&mut bytes); // ?????????? ????? ???????
        let key = SigningKey::from_bytes(&bytes);
        Self { key }
    }

    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.key.sign(message).to_bytes()
    }

    pub fn public_key(&self) -> [u8; 32] {
        self.key.verifying_key().to_bytes()
    }
}

// --- ???????? ??????? ---
pub fn verify(pub_key: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool {
    if let Ok(verifying_key) = VerifyingKey::from_bytes(pub_key) {
        // ? ?????? 2.0 from_bytes ?? ?????????? Result
        let sig = Signature::from_bytes(signature);
        return verifying_key.verify(message, &sig).is_ok();
    }
    false
}
