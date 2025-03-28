/**
 * Solana utility functions for Project 89: Quantum Veil
 */

import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  sendAndConfirmTransaction,
  Commitment,
  ComputeBudgetProgram,
  VersionedTransaction,
  TransactionMessage,
} from '@solana/web3.js';
import * as borsh from 'borsh';
import { retry, sleep } from './index';

/**
 * Network options
 */
export enum SolanaNetwork {
  Mainnet = 'mainnet-beta',
  Devnet = 'devnet',
  Testnet = 'testnet',
  Localnet = 'localhost',
}

/**
 * Get Solana RPC endpoint for a network
 */
export function getSolanaEndpoint(network: SolanaNetwork): string {
  switch (network) {
    case SolanaNetwork.Mainnet:
      return 'https://api.mainnet-beta.solana.com';
    case SolanaNetwork.Devnet:
      return 'https://api.devnet.solana.com';
    case SolanaNetwork.Testnet:
      return 'https://api.testnet.solana.com';
    case SolanaNetwork.Localnet:
      return 'http://localhost:8899';
    default:
      throw new Error(`Unknown Solana network: ${network}`);
  }
}

/**
 * Create a new Solana connection
 */
export function createConnection(network: SolanaNetwork, commitment: Commitment = 'confirmed'): Connection {
  const endpoint = getSolanaEndpoint(network);
  return new Connection(endpoint, commitment);
}

/**
 * Create a Keypair from a secret key byte array
 */
export function createKeypairFromSecret(secretKey: Uint8Array): Keypair {
  return Keypair.fromSecretKey(secretKey);
}

/**
 * Load keypair from file
 */
export function loadKeypairFromFile(filePath: string): Keypair {
  if (typeof process === 'undefined') {
    throw new Error('This function can only be used in Node.js environment');
  }
  
  const fs = require('fs');
  const secretKeyString = fs.readFileSync(filePath, { encoding: 'utf8' });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

/**
 * Save keypair to file
 */
export function saveKeypairToFile(keypair: Keypair, filePath: string): void {
  if (typeof process === 'undefined') {
    throw new Error('This function can only be used in Node.js environment');
  }
  
  const fs = require('fs');
  const secretKeyString = JSON.stringify(Array.from(keypair.secretKey));
  fs.writeFileSync(filePath, secretKeyString, { encoding: 'utf8' });
}

/**
 * Check if a public key is valid
 */
export function isValidPublicKey(publicKeyString: string): boolean {
  try {
    new PublicKey(publicKeyString);
    return true;
  } catch (error) {
    return false;
  }
}

/**
 * Find a program derived address (PDA)
 */
export function findProgramDerivedAddress(
  seeds: Array<Buffer | Uint8Array>,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(seeds, programId);
}

/**
 * Get account data size for borsh-serializable class
 */
export function getAccountDataSize(schema: any, obj: any): number {
  const serialized = borsh.serialize(schema, obj);
  return serialized.length;
}

/**
 * Deserialize account data
 */
export function deserializeAccountData<T>(schema: any, accountType: any, data: Buffer): T {
  return borsh.deserialize(schema, accountType, data) as T;
}

/**
 * Create and send transaction with retry
 */
export async function createAndSendTransaction(
  connection: Connection,
  instructions: TransactionInstruction[],
  signers: Keypair[],
  feePayer: Keypair,
  maxRetries: number = 3
): Promise<string> {
  return retry(async () => {
    const transaction = new Transaction().add(...instructions);
    transaction.feePayer = feePayer.publicKey;
    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    
    return await sendAndConfirmTransaction(
      connection,
      transaction,
      [feePayer, ...signers],
      { commitment: 'confirmed' }
    );
  }, maxRetries);
}

/**
 * Create versioned transaction with priority fee
 */
export async function createVersionedTransaction(
  connection: Connection,
  instructions: TransactionInstruction[],
  signers: Keypair[],
  priorityFee: number = 0
): Promise<VersionedTransaction> {
  const latestBlockhash = await connection.getLatestBlockhash();
  
  // Add priority fee instruction if needed
  if (priorityFee > 0) {
    const priorityFeeIx = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: priorityFee
    });
    instructions.unshift(priorityFeeIx);
  }
  
  // Create message
  const messageV0 = new TransactionMessage({
    payerKey: signers[0].publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions,
  }).compileToV0Message();
  
  // Create transaction
  const transaction = new VersionedTransaction(messageV0);
  
  // Sign transaction
  transaction.sign(signers);
  
  return transaction;
}

/**
 * Wait for transaction confirmation
 */
export async function waitForConfirmation(
  connection: Connection,
  signature: string,
  timeout: number = 60000,
  commitment: Commitment = 'confirmed'
): Promise<boolean> {
  const startTime = Date.now();
  
  while (Date.now() - startTime < timeout) {
    const status = await connection.getSignatureStatus(signature);
    
    if (status && status.value) {
      if (status.value.confirmationStatus === commitment) {
        return true;
      }
      
      if (status.value.err) {
        throw new Error(`Transaction failed: ${JSON.stringify(status.value.err)}`);
      }
    }
    
    await sleep(1000);
  }
  
  throw new Error(`Transaction confirmation timeout after ${timeout}ms`);
}

/**
 * Calculate the minimum balance for rent exemption
 */
export async function getMinimumBalanceForRentExemption(
  connection: Connection,
  dataSize: number
): Promise<number> {
  return await connection.getMinimumBalanceForRentExemption(dataSize);
}

/**
 * Create an account instruction
 */
export async function createAccountInstruction(
  connection: Connection,
  payer: PublicKey,
  newAccount: Keypair,
  programId: PublicKey,
  dataSize: number
): Promise<TransactionInstruction> {
  const lamports = await connection.getMinimumBalanceForRentExemption(dataSize);
  
  return SystemProgram.createAccount({
    fromPubkey: payer,
    newAccountPubkey: newAccount.publicKey,
    lamports,
    space: dataSize,
    programId,
  });
}

/**
 * Get NFT metadata address
 */
export function getNftMetadataAddress(
  mintAddress: PublicKey,
  metadataProgramId: PublicKey = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')
): PublicKey {
  const [metadataAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      metadataProgramId.toBuffer(),
      mintAddress.toBuffer(),
    ],
    metadataProgramId
  );
  
  return metadataAddress;
}

/**
 * Determine if an account exists
 */
export async function accountExists(
  connection: Connection,
  publicKey: PublicKey
): Promise<boolean> {
  try {
    const accountInfo = await connection.getAccountInfo(publicKey);
    return accountInfo !== null;
  } catch (error) {
    return false;
  }
}

/**
 * Create an account with seed
 */
export function createAccountWithSeedInstruction(
  from: PublicKey,
  newAccount: PublicKey,
  basePubkey: PublicKey,
  seed: string,
  lamports: number,
  space: number,
  programId: PublicKey
): TransactionInstruction {
  return SystemProgram.createAccountWithSeed({
    fromPubkey: from,
    newAccountPubkey: newAccount,
    basePubkey,
    seed,
    lamports,
    space,
    programId,
  });
}

/**
 * Check if a transaction is confirmed
 */
export async function isTransactionConfirmed(
  connection: Connection,
  signature: string
): Promise<boolean> {
  const status = await connection.getSignatureStatus(signature);
  return status !== null && 
         status.value !== null && 
         status.value.confirmationStatus === 'confirmed';
}

/**
 * Monitor an account for changes
 */
export function monitorAccount(
  connection: Connection,
  accountPubkey: PublicKey,
  callback: (accountInfo: any) => void,
  commitment: Commitment = 'confirmed'
): number {
  return connection.onAccountChange(
    accountPubkey,
    callback,
    commitment
  );
}

/**
 * Stop monitoring an account
 */
export function stopMonitoringAccount(
  connection: Connection,
  subscriptionId: number
): Promise<void> {
  return connection.removeAccountChangeListener(subscriptionId);
}

/**
 * Get SOL balance
 */
export async function getSolBalance(
  connection: Connection,
  publicKey: PublicKey
): Promise<number> {
  const balance = await connection.getBalance(publicKey);
  return balance / 1_000_000_000; // Convert lamports to SOL
}
