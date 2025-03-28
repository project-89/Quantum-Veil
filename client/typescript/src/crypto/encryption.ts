/**
 * Encryption utilities for Project 89: Quantum Veil
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
 * Encrypt data using a password instead of a key
 * 
 * @param data Data to encrypt
 * @param password Password to use for encryption
 * @returns Encrypted data with salt
 */
export function encryptWithPassword(data: Buffer, password: string): Buffer {
  // Generate a random salt
  const salt = crypto.randomBytes(16);
  
  // Derive a key from the password
  const key = crypto.pbkdf2Sync(password, salt, 100000, 32, 'sha256');
  
  // Encrypt the data
  const encrypted = encrypt(data, key);
  
  // Combine salt and encrypted data
  return Buffer.concat([salt, encrypted]);
}

/**
 * Decrypt data using a password
 * 
 * @param encryptedData Encrypted data with salt
 * @param password Password to use for decryption
 * @returns Decrypted data
 */
export function decryptWithPassword(encryptedData: Buffer, password: string): Buffer {
  // Extract salt (first 16 bytes)
  const salt = encryptedData.subarray(0, 16);
  
  // Extract encrypted data (remaining bytes)
  const encrypted = encryptedData.subarray(16);
  
  // Derive the key from the password and salt
  const key = crypto.pbkdf2Sync(password, salt, 100000, 32, 'sha256');
  
  // Decrypt the data
  return decrypt(encrypted, key);
}

/**
 * Create a secure key from multiple entropy sources
 * 
 * @param entropyInputs Array of entropy sources
 * @returns Secure key
 */
export function createSecureKey(entropyInputs: Buffer[]): Buffer {
  // Combine all entropy inputs
  const combinedEntropy = Buffer.concat(entropyInputs);
  
  // Hash the combined entropy
  const hash = crypto.createHash('sha512');
  hash.update(combinedEntropy);
  
  // Return the first 32 bytes as the key
  return Buffer.from(hash.digest().subarray(0, 32));
}

/**
 * Encrypt data with ChaCha20-Poly1305 (alternative to AES-GCM)
 * Note: This requires Node.js 16+ or appropriate polyfill
 * 
 * @param data Data to encrypt
 * @param key Encryption key
 * @returns Encrypted data
 */
export function encryptChaCha20(data: Buffer, key: Uint8Array): Buffer {
  // Ensure we have a properly sized key
  if (key.length !== 32) {
    throw new Error('ChaCha20-Poly1305 requires a 32-byte key');
  }

  try {
    // Generate a random nonce
    const nonce = crypto.randomBytes(12);
    
    // Create cipher
    const cipher = crypto.createCipheriv('chacha20-poly1305', key, nonce, {
      authTagLength: 16
    });
    
    // Encrypt data
    const ciphertext = Buffer.concat([
      cipher.update(data),
      cipher.final()
    ]);
    
    // Get authentication tag
    const authTag = cipher.getAuthTag();
    
    // Combine nonce, authentication tag, and ciphertext
    return Buffer.concat([nonce, authTag, ciphertext]);
  } catch (e) {
    // Fallback to AES-GCM if ChaCha20-Poly1305 is not available
    console.warn('ChaCha20-Poly1305 not available, falling back to AES-GCM');
    return encrypt(data, key);
  }
}

/**
 * Decrypt data encrypted with ChaCha20-Poly1305
 * 
 * @param encryptedData Encrypted data
 * @param key Decryption key
 * @returns Decrypted data
 */
export function decryptChaCha20(encryptedData: Buffer, key: Uint8Array): Buffer {
  // Ensure we have a properly sized key
  if (key.length !== 32) {
    throw new Error('ChaCha20-Poly1305 requires a 32-byte key');
  }

  try {
    // Extract nonce (first 12 bytes)
    const nonce = encryptedData.subarray(0, 12);
    
    // Extract auth tag (next 16 bytes)
    const authTag = encryptedData.subarray(12, 28);
    
    // Extract ciphertext (remaining bytes)
    const ciphertext = encryptedData.subarray(28);
    
    // Create decipher
    const decipher = crypto.createDecipheriv('chacha20-poly1305', key, nonce, {
      authTagLength: 16
    });
    
    // Set auth tag
    decipher.setAuthTag(authTag);
    
    // Decrypt
    const decrypted = Buffer.concat([
      decipher.update(ciphertext),
      decipher.final()
    ]);
    
    return decrypted;
  } catch (e) {
    // If it's not a ChaCha20 ciphertext, try with AES-GCM
    console.warn('Failed to decrypt with ChaCha20-Poly1305, trying AES-GCM');
    return decrypt(encryptedData, key);
  }
}
