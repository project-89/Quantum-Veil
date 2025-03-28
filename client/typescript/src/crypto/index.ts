/**
 * Cryptographic utilities for Project 89: Quantum Veil
 */

import * as crypto from 'crypto';

// Export all encryption/decryption functions
export { 
  encrypt, 
  decrypt,
  encryptWithPassword,
  decryptWithPassword
} from './encryption';

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

/**
 * Generate a random seed for key derivation
 * 
 * @param length Length of seed in bytes
 * @returns Random seed
 */
export function generateRandomSeed(length: number = 32): Buffer {
  return crypto.randomBytes(length);
}

/**
 * Create a deterministic hash from multiple inputs
 * 
 * @param inputs Various inputs to hash together
 * @returns Combined hash
 */
export function createDeterministicHash(...inputs: (string | Buffer)[]): string {
  const hash = crypto.createHash('sha256');
  
  for (const input of inputs) {
    if (typeof input === 'string') {
      hash.update(input);
    } else {
      hash.update(input);
    }
  }
  
  return hash.digest('hex');
}

/**
 * Generate a time-based seed
 * 
 * @returns Seed based on current time
 */
export function generateTimeBasedSeed(): Buffer {
  const timestamp = BigInt(Date.now() * 1000000); // Microsecond precision
  const timestampBytes = Buffer.alloc(8);
  
  // Convert timestamp to bytes
  for (let i = 0; i < 8; i++) {
    timestampBytes[i] = Number((timestamp >> BigInt(i * 8)) & BigInt(0xff));
  }
  
  // Combine with random data
  const randomBytes = crypto.randomBytes(24);
  return Buffer.concat([timestampBytes, randomBytes]);
}
