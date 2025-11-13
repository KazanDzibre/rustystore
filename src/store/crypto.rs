use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::{Engine as _, engine::general_purpose};
use rand::rngs::OsRng;
use rand::{RngCore, TryRngCore};

const AES_KEY_BYTES: [u8; 32] = [0x42; 32]; // fixed key

pub fn encrypt(plaintext: &[u8]) -> String {
    let key = Key::<Aes256Gcm>::from_slice(&AES_KEY_BYTES);
    let cipher = Aes256Gcm::new(key);

    // create an OsRng instance
    let mut rng = OsRng;

    let mut nonce_bytes = [0u8; 12];
    rng.try_fill_bytes(&mut nonce_bytes)
        .expect("failed to generate nonce");
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failed");

    // Store nonce + ciphertext together
    let mut combined = Vec::new();
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    general_purpose::STANDARD.encode(&combined)
}

pub fn decrypt(encoded: &str) -> Result<Vec<u8>, &'static str> {
    let data = general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "base64 decode failed")?;
    if data.len() < 12 {
        return Err("data too short");
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(&AES_KEY_BYTES);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed")
}
