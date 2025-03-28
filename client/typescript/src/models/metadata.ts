/**
 * Metadata models for Glitch Gang NFTs
 */

/**
 * NFT attribute
 */
export interface Attribute {
  trait_type: string;
  value: string;
}

/**
 * File reference
 */
export interface File {
  uri: string;
}

/**
 * NFT properties 
 */
export interface Properties {
  files: File[];
}

/**
 * VRM configuration 
 */
export interface VrmConfig {
  modelUri: string;
  privacySettings: {
    [key: string]: string;
  };
}

/**
 * Private data section added by privacy system
 */
export interface PrivateData {
  privacyLevel: string;
  encryptedAttributes?: string;
  timelineFragments?: string[];
  vrmConfig?: VrmConfig;
}

/**
 * Glitch Gang NFT metadata
 */
export interface GlitchGangMetadata {
  name: string;
  symbol: string;
  description: string;
  attributes: Attribute[];
  image: string;
  properties: Properties;
  privateData?: PrivateData;
}
