import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { VRM, VRMUtils } from '@pixiv/three-vrm';
import * as THREE from 'three';

import { encrypt, decrypt } from './crypto';
import { GlitchGangMetadata, Attribute } from './models/metadata';
import { VrmData, PositionData, RotationData, VoiceData, GestureData, PrivacyLevel } from './models/vrm';

/**
 * Glitch Gang Privacy Client for TypeScript
 */
export class GlitchGangPrivacyClient {
  private connection: Connection;
  public ownerKeypair: Keypair;
  private programId: PublicKey;
  private encryptionKey: Uint8Array;
  private vrmModel: VRM | null = null;

  constructor(
    endpoint: string,
    ownerKeypair: Keypair,
    programId: string,
    encryptionKey?: Uint8Array
  ) {
    this.connection = new Connection(endpoint);
    this.ownerKeypair = ownerKeypair;
    this.programId = new PublicKey(programId);
    
    // Generate encryption key if not provided
    if (encryptionKey) {
      this.encryptionKey = encryptionKey;
    } else {
      this.encryptionKey = new Uint8Array(32);
      crypto.getRandomValues(this.encryptionKey);
    }
  }

  // Fetch NFT metadata
  async fetchMetadata(metadataUri: string): Promise<GlitchGangMetadata> {
    console.log(`Fetching metadata from: ${metadataUri}`);
    
    const response = await fetch(metadataUri);
    if (!response.ok) {
      throw new Error(`Failed to fetch metadata: ${response.statusText}`);
    }
    
    return await response.json();
  }

  // Load VRM model
  async loadVrmModel(modelUri: string): Promise<VRM> {
    console.log(`Loading VRM model: ${modelUri}`);
    
    const loader = new THREE.GLTFLoader();
    
    return new Promise((resolve, reject) => {
      loader.load(
        modelUri,
        (gltf) => {
          VRMUtils.removeUnnecessaryJoints(gltf.scene);
          
          VRM.from(gltf).then((vrm) => {
            this.vrmModel = vrm;
            resolve(vrm);
          });
        },
        (progress) => console.log(`Loading model: ${(progress.loaded / progress.total) * 100}%`),
        (error) => reject(`Error loading VRM: ${error}`)
      );
    });
  }

  // Protect metadata with privacy features
  protectMetadata(metadata: GlitchGangMetadata, privacyLevel: PrivacyLevel): GlitchGangMetadata {
    console.log(`Applying privacy level: ${PrivacyLevel[privacyLevel]}`);
    
    const protectedMetadata = { ...metadata };
    
    // Select sensitive attributes to protect based on privacy level
    const sensitiveAttributes = this.getSensitiveAttributes(privacyLevel);
    
    // Extract sensitive attributes
    const privateAttrs: Attribute[] = [];
    const publicAttrs: Attribute[] = [];
    
    for (const attr of metadata.attributes) {
      if (sensitiveAttributes.includes(attr.trait_type)) {
        privateAttrs.push(attr);
      } else {
        publicAttrs.push(attr);
      }
    }
    
    // Replace protected attributes with placeholders
    protectedMetadata.attributes = publicAttrs;
    
    // Only encrypt if we have sensitive attributes
    if (privateAttrs.length > 0) {
      // Encrypt private attributes
      const privateJson = JSON.stringify(privateAttrs);
      const encrypted = encrypt(Buffer.from(privateJson), this.encryptionKey);
      const encryptedB64 = Buffer.from(encrypted).toString('base64');
      
      // Add private data section
      protectedMetadata.privateData = {
        privacyLevel: PrivacyLevel[privacyLevel],
        encryptedAttributes: encryptedB64
      };
    }
    
    return protectedMetadata;
  }
  
  // Get sensitive attributes based on privacy level
  private getSensitiveAttributes(privacyLevel: PrivacyLevel): string[] {
    switch (privacyLevel) {
      case PrivacyLevel.None:
        return [];
      case PrivacyLevel.Light:
        return ['Secret Code', 'Agent Name'];
      case PrivacyLevel.Medium:
        return ['Secret Code', 'Agent Name', 'Mission', 'Origin'];
      case PrivacyLevel.Heavy:
      case PrivacyLevel.Complete:
        return ['Secret Code', 'Agent Name', 'Mission', 'Origin', 'Accessory', 'Symbols'];
      default:
        return [];
    }
  }

  // Decrypt protected metadata
  decryptMetadata(protectedMetadata: GlitchGangMetadata): GlitchGangMetadata {
    if (!protectedMetadata.privateData?.encryptedAttributes) {
      return protectedMetadata;
    }
    
    try {
      // Decrypt the private attributes
      const encryptedData = Buffer.from(protectedMetadata.privateData.encryptedAttributes, 'base64');
      const decryptedData = decrypt(encryptedData, this.encryptionKey);
      const privateAttrs: Attribute[] = JSON.parse(decryptedData.toString());
      
      // Merge with public attributes
      const fullMetadata = { ...protectedMetadata };
      fullMetadata.attributes = [...protectedMetadata.attributes, ...privateAttrs];
      
      return fullMetadata;
    } catch (error) {
      console.error('Failed to decrypt metadata:', error);
      return protectedMetadata;
    }
  }

  // Add VRM privacy settings
  addVrmPrivacy(metadata: GlitchGangMetadata, modelUri: string): GlitchGangMetadata {
    console.log(`Adding VRM privacy settings for model: ${modelUri}`);
    
    const updatedMetadata = { ...metadata };
    
    const privacySettings: Record<string, string> = {
      position: 'medium',
      voice: 'high',
      gesture: 'medium',
      animation: 'low'
    };
    
    const vrmConfig = {
      modelUri,
      privacySettings
    };
    
    // Create or update private data section
    if (!updatedMetadata.privateData) {
      updatedMetadata.privateData = {
        privacyLevel: 'Medium',
        vrmConfig
      };
    } else {
      updatedMetadata.privateData.vrmConfig = vrmConfig;
    }
    
    return updatedMetadata;
  }

  // Apply privacy mask to VRM data
  applyVrmPrivacyMask(
    vrmData: VrmData, 
    viewerId?: string,
    nftMint?: string
  ): VrmData {
    console.log(`Applying VRM privacy mask${viewerId ? ` for viewer: ${viewerId}` : ''}`);
    
    // If no VRM model or no privacy settings, return unmodified data
    if (!this.vrmModel) {
      return vrmData;
    }
    
    // Check if viewer is trusted
    const isTrusted = viewerId 
      ? this.isViewerTrusted(viewerId, nftMint)
      : false;
    
    if (isTrusted) {
      return vrmData;
    }
    
    // Clone data to avoid modifying original
    const maskedData: VrmData = JSON.parse(JSON.stringify(vrmData));
    
    // Apply noise to position
    this.addPositionNoise(maskedData.position, 0.5);
    
    // Apply noise to rotation
    this.addRotationNoise(maskedData.rotation, 0.3);
    
    // Apply noise to voice if present
    if (maskedData.voice) {
      this.addVoiceNoise(maskedData.voice, 0.7);
    }
    
    // Apply noise to gestures
    for (const gesture of maskedData.gestures) {
      this.addGestureNoise(gesture, 0.4);
    }
    
    return maskedData;
  }
  
  // Check if viewer is trusted
  private isViewerTrusted(viewerId: string, nftMint?: string): boolean {
    // In a real implementation, this would check the trusted agents list
    // For demo purposes, we'll just check a hardcoded list
    const trustedAgents = ['agent1.glitch.gang', 'agent2.glitch.gang'];
    return trustedAgents.includes(viewerId);
  }
  
  // Add noise to position data
  private addPositionNoise(position: PositionData, intensity: number): void {
    position.x += (Math.random() - 0.5) * 2 * intensity;
    position.y += (Math.random() - 0.5) * 2 * intensity;
    position.z += (Math.random() - 0.5) * 2 * intensity;
  }
  
  // Add noise to rotation data
  private addRotationNoise(rotation: RotationData, intensity: number): void {
    // Create small random rotation quaternion
    const noiseAngle = intensity * Math.PI * Math.random();
    const noiseAxisX = Math.random() * 2 - 1;
    const noiseAxisY = Math.random() * 2 - 1;
    const noiseAxisZ = Math.random() * 2 - 1;
    
    // Normalize axis
    const mag = Math.sqrt(noiseAxisX**2 + noiseAxisY**2 + noiseAxisZ**2);
    const normalizedX = noiseAxisX / mag;
    const normalizedY = noiseAxisY / mag;
    const normalizedZ = noiseAxisZ / mag;
    
    // Create noise quaternion
    const sinHalfAngle = Math.sin(noiseAngle / 2);
    const cosHalfAngle = Math.cos(noiseAngle / 2);
    
    const noiseQuatX = normalizedX * sinHalfAngle;
    const noiseQuatY = normalizedY * sinHalfAngle;
    const noiseQuatZ = normalizedZ * sinHalfAngle;
    const noiseQuatW = cosHalfAngle;
    
    // Apply noise quaternion (quaternion multiplication)
    const originalX = rotation.x;
    const originalY = rotation.y;
    const originalZ = rotation.z;
    const originalW = rotation.w;
    
    rotation.x = originalW * noiseQuatX + originalX * noiseQuatW + originalY * noiseQuatZ - originalZ * noiseQuatY;
    rotation.y = originalW * noiseQuatY - originalX * noiseQuatZ + originalY * noiseQuatW + originalZ * noiseQuatX;
    rotation.z = originalW * noiseQuatZ + originalX * noiseQuatY - originalY * noiseQuatX + originalZ * noiseQuatW;
    rotation.w = originalW * noiseQuatW - originalX * noiseQuatX - originalY * noiseQuatY - originalZ * noiseQuatZ;
    
    // Normalize quaternion
    const rotMag = Math.sqrt(rotation.x**2 + rotation.y**2 + rotation.z**2 + rotation.w**2);
    rotation.x /= rotMag;
    rotation.y /= rotMag;
    rotation.z /= rotMag;
    rotation.w /= rotMag;
  }
  
  // Add noise to voice data
  private addVoiceNoise(voice: VoiceData, intensity: number): void {
    // Add noise to frequency components
    for (let i = 0; i < voice.frequency.length; i++) {
      voice.frequency[i] += (Math.random() - 0.5) * 2 * intensity * 100;
    }
    
    // Add noise to amplitude components
    for (let i = 0; i < voice.amplitude.length; i++) {
      voice.amplitude[i] += (Math.random() - 0.5) * 2 * intensity;
      voice.amplitude[i] = Math.max(0, voice.amplitude[i]);
    }
    
    // Add noise to pitch and timbre
    voice.pitch += (Math.random() - 0.5) * 2 * intensity * 50;
    voice.timbre += (Math.random() - 0.5) * 2 * intensity;
    voice.timbre = Math.max(0, Math.min(1, voice.timbre));
  }
  
  // Add noise to gesture data
  private addGestureNoise(gesture: GestureData, intensity: number): void {
    gesture.intensity += (Math.random() - 0.5) * 2 * intensity;
    gesture.intensity = Math.max(0, Math.min(1, gesture.intensity));
    
    gesture.speed += (Math.random() - 0.5) * 2 * intensity;
    gesture.speed = Math.max(0.1, gesture.speed);
  }

  // Save protected metadata
  async saveProtectedMetadata(
    metadata: GlitchGangMetadata,
    outputPath: string
  ): Promise<void> {
    const json = JSON.stringify(metadata, null, 2);
    
    try {
      // In a browser environment, trigger a download
      if (typeof window !== 'undefined') {
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = outputPath;
        a.click();
        
        URL.revokeObjectURL(url);
      } 
      // In Node.js, write to file
      else {
        const fs = require('fs');
        fs.writeFileSync(outputPath, json);
      }
      
      console.log(`Protected metadata saved to ${outputPath}`);
    } catch (error) {
      console.error('Failed to save metadata:', error);
      throw error;
    }
  }
}
