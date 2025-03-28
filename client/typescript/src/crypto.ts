/**
 * Cryptographic utilities for Project 89: Quantum Veil
 */

import * as crypto from 'crypto';

/**
 * Encrypt data using AES-GCM
 * 
 * @param data Data to encrypt
 * @param key Encryption key (32 bytes)
 * @returns Encrypted data
 */
export function encrypt(data: Buffer, key: Uint8Array): Buffer {
  // Generate a random IV for each encryption
  const iv = crypto.randomBytes(16);
  
  // Create cipher
  const cipher = crypto.createCipheriv('aes-256-gcm', Buffer.from(key), iv);
  
  // Encrypt data
  const encrypted = Buffer.concat([
    cipher.update(data),
    cipher.final()
  ]);
  
  // Get authentication tag
  const authTag = cipher.getAuthTag();
  
  // Combine IV, encrypted data, and authentication tag
  const result = Buffer.concat([
    iv,
    authTag,
    encrypted
  ]);
  
  return result;
}

/**
 * Decrypt data using AES-GCM
 * 
 * @param encryptedData Encrypted data (IV + Auth Tag + Ciphertext)
 * @param key Decryption key (32 bytes)
 * @returns Decrypted data
 */
export function decrypt(encryptedData: Buffer, key: Uint8Array): Buffer {
  // Extract IV (first 16 bytes)
  const iv = encryptedData.subarray(0, 16);
  
  // Extract authentication tag (next 16 bytes)
  const authTag = encryptedData.subarray(16, 32);
  
  // Extract ciphertext (remaining bytes)
  const ciphertext = encryptedData.subarray(32);
  
  // Create decipher
  const decipher = crypto.createDecipheriv('aes-256-gcm', Buffer.from(key), iv);
  
  // Set authentication tag
  decipher.setAuthTag(authTag);
  
  // Decrypt data
  const decrypted = Buffer.concat([
    decipher.update(ciphertext),
    decipher.final()
  ]);
  
  return decrypted;
}

/**
 * Generate a random encryption key
 * 
 * @returns 32-byte random key
 */
export function generateEncryptionKey(): Uint8Array {
  return crypto.randomBytes(32);
}

/**
 * Derive a key from a password or seed phrase
 * 
 * @param password Password or seed phrase
 * @param salt Salt for derivation (optional)
 * @returns 32-byte derived key
 */
export function deriveKey(password: string, salt?: Buffer): Buffer {
  const saltBuffer = salt || crypto.randomBytes(16);
  return crypto.pbkdf2Sync(password, saltBuffer, 100000, 32, 'sha256');
}

/**
 * Calculate a hash of the data
 * 
 * @param data Data to hash
 * @returns SHA-256 hash
 */
export function hashData(data: Buffer | string): string {
  const hash = crypto.createHash('sha256');
  hash.update(typeof data === 'string' ? data : Buffer.from(data));
  return hash.digest('hex');
}

/**
 * Sign data with a private key
 * 
 * @param data Data to sign
 * @param privateKey Private key for signing
 * @returns Signature
 */
export function signData(data: Buffer, privateKey: crypto.KeyObject): Buffer {
  const sign = crypto.createSign('SHA256');
  sign.update(data);
  sign.end();
  return sign.sign(privateKey);
}

/**
 * Verify a signature
 * 
 * @param data Original data
 * @param signature Signature to verify
 * @param publicKey Public key for verification
 * @returns Whether the signature is valid
 */
export function verifySignature(data: Buffer, signature: Buffer, publicKey: crypto.KeyObject): boolean {
  const verify = crypto.createVerify('SHA256');
  verify.update(data);
  verify.end();
  return verify.verify(publicKey, signature);
}
