use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};

/// Encrypt data using ChaCha20Poly1305
pub fn encrypt_data(data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err(format!("Invalid key length: {}, expected 32", key.len()));
    }
    
    if nonce.len() != 12 {
        return Err(format!("Invalid nonce length: {}, expected 12", nonce.len()));
    }
    
    let cipher_key = Key::from_slice(key);
    let cipher_nonce = Nonce::from_slice(nonce);
    
    let cipher = ChaCha20Poly1305::new(cipher_key);
    
    cipher.encrypt(cipher_nonce, data)
        .map_err(|e| format!("Encryption error: {}", e))
}

/// Decrypt data using ChaCha20Poly1305
pub fn decrypt_data(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err(format!("Invalid key length: {}, expected 32", key.len()));
    }
    
    if nonce.len() != 12 {
        return Err(format!("Invalid nonce length: {}, expected 12", nonce.len()));
    }
    
    let cipher_key = Key::from_slice(key);
    let cipher_nonce = Nonce::from_slice(nonce);
    
    let cipher = ChaCha20Poly1305::new(cipher_key);
    
    cipher.decrypt(cipher_nonce, ciphertext)
        .map_err(|e| format!("Decryption error: {}", e))
}

/// Create a deterministic key from a seed
pub fn derive_key_from_seed(seed: &[u8]) -> ([u8; 32], [u8; 12]) {
    use sha3::{Sha3_512, Digest};
    
    let mut hasher = Sha3_512::new();
    hasher.update(seed);
    let result = hasher.finalize();
    
    // Split the hash result into key and nonce
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
    
    key.copy_from_slice(&result[0..32]);
    nonce.copy_from_slice(&result[32..44]);
    
    (key, nonce)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32]; // All zeros for test
        let nonce = [0u8; 12]; // All zeros for test
        let data = b"Test data to encrypt";
        
        let encrypted = encrypt_data(data, &key, &nonce).unwrap();
        let decrypted = decrypt_data(&encrypted, &key, &nonce).unwrap();
        
        assert_eq!(data, &decrypted[..]);
    }
    
    #[test]
    fn test_key_derivation() {
        let seed = b"test seed for key derivation";
        
        let (key, nonce) = derive_key_from_seed(seed);
        
        // Ensure derivation is deterministic
        let (key2, nonce2) = derive_key_from_seed(seed);
        
        assert_eq!(key, key2);
        assert_eq!(nonce, nonce2);
    }
}
