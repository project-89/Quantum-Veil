/**
 * Project 89: Quantum Veil
 * 
 * A privacy system for Glitch Gang NFTs on Solana
 */

// Export client
export { GlitchGangPrivacyClient } from './client';

// Export models
export { 
  PrivacyLevel,
  VrmData,
  PositionData,
  RotationData,
  VoiceData,
  GestureData,
  VrmDataType,
  AccessPermission
} from './models/vrm';

export {
  GlitchGangMetadata,
  Attribute,
  PrivateData,
  VrmConfig
} from './models/metadata';

export {
  TimelineType,
  EntropySource,
  StorageLocation,
  WrapperInstruction,
  PrivacyConfig,
  SynchronicityMask
} from './models';

// Export crypto utilities
export {
  encrypt,
  decrypt,
  generateEncryptionKey,
  deriveKey,
  hashData
} from './crypto';

// Banner
export const BANNER = `
  _____           _           _     ___  ___  
 |  __ \\         (_)         | |    |  \\/  |  
 | |__) | __ ___  _  ___  ___| |_   | .  . |  
 |  ___/ '__/ _ \\| |/ _ \\/ __| __|  | |\\/| |  
 | |   | | | (_) | |  __/ (__| |_   | |  | |  
 |_|   |_|  \\___/| |\\___|\\___|\\__|  \\_|  |_/  
               _/ |                           
              |__/                            
              
  QUANTUM VEIL: Privacy System for Glitch Gang NFTs
`;

// Example function
export async function runDemo() {
  try {
    console.log(BANNER);
    console.log("Glitch Gang Privacy System Demo");
    
    // Import required modules
    const { Keypair } = await import('@solana/web3.js');
    
    // Generate keypair (in a real app, this would come from a wallet)
    const ownerKeypair = Keypair.generate();
    
    // Create privacy client
    const client = new GlitchGangPrivacyClient(
      'https://api.devnet.solana.com',
      ownerKeypair,
      'GlchWrapperProgram111111111111111111111111111'
    );
    
    // Fetch original metadata
    const metadata = await client.fetchMetadata('https://example.com/metadata/glitchgang699.json');
    console.log("Original metadata loaded");
    
    // Apply privacy protections
    const protectedMetadata = client.protectMetadata(metadata, PrivacyLevel.Medium);
    console.log("Privacy protections applied");
    
    // Add VRM privacy settings
    const finalMetadata = client.addVrmPrivacy(
      protectedMetadata, 
      'https://models.glitchgang.io/avatars/699.vrm'
    );
    console.log("VRM privacy settings added");
    
    // Save protected metadata
    await client.saveProtectedMetadata(finalMetadata, 'protected_glitchgang699.json');
    
    // Example VRM data processing
    const vrmData: VrmData = {
      position: { x: 10.5, y: 2.0, z: -3.2 },
      rotation: { x: 0.0, y: 0.707, z: 0.0, w: 0.707 },
      voice: {
        frequency: [440.0, 880.0, 1320.0],
        amplitude: [0.8, 0.4, 0.2],
        pitch: 1.0,
        timbre: 0.5
      },
      gestures: [
        {
          name: "wave",
          intensity: 0.8,
          speed: 1.2,
          jointRotations: new Map()
        }
      ],
      animations: new Map(),
      customData: new Map()
    };
    
    // Process VRM data for different viewers
    const publicView = client.applyVrmPrivacyMask(vrmData);
    console.log("Public VRM view (position):", publicView.position);
    
    const trustedView = client.applyVrmPrivacyMask(vrmData, 'agent1.glitch.gang');
    console.log("Trusted agent VRM view (position):", trustedView.position);
    
    console.log("Demo completed successfully");
  } catch (error) {
    console.error("Demo failed:", error);
  }
}

// Run the demo if this module is executed directly
if (typeof window !== 'undefined') {
  // Browser environment - run when DOM is loaded
  window.addEventListener('DOMContentLoaded', runDemo);
} else if (require.main === module) {
  // Node.js environment - run immediately if this is the main module
  runDemo();
}
