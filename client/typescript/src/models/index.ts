/**
 * Models index file for Project 89: Quantum Veil
 */

// Re-export all models
export * from './metadata';
export * from './vrm';

// Additional model types

/**
 * Timeline types for metadata fragmentation
 */
export enum TimelineType {
  Primary = "Primary",
  Identity = "Identity",
  Activity = "Activity",
  Social = "Social",
  Financial = "Financial",
  Custom = "Custom"
}

/**
 * Entropy sources for quantum-grade key generation
 */
export enum EntropySource {
  BlockchainHash = "BlockchainHash",
  TimeEntropy = "TimeEntropy",
  CosmicNoise = "CosmicNoise",
  AgentBehavior = "AgentBehavior"
}

/**
 * Storage location for metadata fragments
 */
export enum StorageLocation {
  Onchain = "Onchain",
  Arweave = "Arweave",
  Ipfs = "Ipfs",
  ShadowRealm = "ShadowRealm"
}

/**
 * Wrapper instruction types for Solana program
 */
export class WrapperInstruction {
  createWrapper?: { privacyConfigHash: string };
  updatePrivacy?: { newPrivacyConfigHash: string };
  grantAccess?: { account: string, level: number };
  revokeAccess?: { account: string };

  constructor(fields: {
    createWrapper?: { privacyConfigHash: string },
    updatePrivacy?: { newPrivacyConfigHash: string },
    grantAccess?: { account: string, level: number },
    revokeAccess?: { account: string }
  }) {
    this.createWrapper = fields.createWrapper;
    this.updatePrivacy = fields.updatePrivacy;
    this.grantAccess = fields.grantAccess;
    this.revokeAccess = fields.revokeAccess;
  }
}

// Wrapper instruction schema for borsh serialization
export const WrapperInstructionSchema = new Map([
  [
    WrapperInstruction,
    {
      kind: 'struct',
      fields: [
        ['createWrapper', { kind: 'option', type: { kind: 'struct', fields: [['privacyConfigHash', 'string']] } }],
        ['updatePrivacy', { kind: 'option', type: { kind: 'struct', fields: [['newPrivacyConfigHash', 'string']] } }],
        ['grantAccess', { kind: 'option', type: { kind: 'struct', fields: [['account', 'string'], ['level', 'u8']] } }],
        ['revokeAccess', { kind: 'option', type: { kind: 'struct', fields: [['account', 'string']] } }],
      ],
    },
  ],
]);

/**
 * Privacy wrapper state interface
 */
export interface PrivacyWrapper {
  originalNftMint: string;
  owner: string;
  privacyConfigHash: string;
  accessControls: Map<string, number>;
  lastUpdated: number;
}

/**
 * Privacy configuration for a Glitch Gang NFT
 */
export interface PrivacyConfig {
  owner: string;
  nftMint: string;
  currentKey: string;
  currentNonce: string;
  entropySources: EntropySource[];
  keyRotationFrequency: number;
  lastRotation: number;
  syncMask: SynchronicityMask;
}

/**
 * Synchronicity mask settings for VRM behavior obfuscation
 */
export interface SynchronicityMask {
  positionNoise: number;
  voiceNoise: number;
  gestureNoise: number;
  trustedAgents: string[];
}
