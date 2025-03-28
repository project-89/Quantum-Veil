use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::str::FromStr;
use std::fs;
use std::collections::HashMap;

// Import the privacy client
use project_89::{
    GlitchGangPrivacyClient,
    PrivacyLevel,
    VrmData,
    PositionData,
    RotationData,
    VoiceData,
    GestureData,
};

/// Project 89: Quantum Veil - Demo CLI
///
/// This example demonstrates how to use the Glitch Gang privacy system
/// to protect existing NFTs on Solana.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    println!("\n⧂ PROJECT 89: QUANTUM VEIL ⧂");
    println!("Privacy system for Glitch Gang NFTs\n");
    
    // Get or generate keypair
    let keypair = get_or_create_keypair()?;
    
    println!("Using wallet: {}", keypair.pubkey());
    
    // Create privacy client
    let client = GlitchGangPrivacyClient::new(
        "https://api.devnet.solana.com", // Use devnet for testing
        keypair,
    );
    
    // Demo NFT details
    let nft_mint_address = "3jKpTiKAAtnJMLcQsNk82ua7crubQ86e8KfTQB9fKDwp";
    let nft_mint = Pubkey::from_str(nft_mint_address)?;
    let metadata_uri = "https://example.com/metadata/glitchgang699.json";
    
    // If metadata file exists locally, use it
    let metadata_json = fs::read_to_string("glitchgang_metadata.json").unwrap_or_else(|_| {
        r#"{
          "name": "Glitch Gang #699 - VertexStream Navigator",
          "symbol": "GG",
          "description": "A mysterious entity from the Glitch Gang collective, wielding the power of digital chaos.",
          "attributes": [
            {
              "trait_type": "Background",
              "value": "Cyber Haze"
            },
            {
              "trait_type": "Hood",
              "value": "Starlight Shroud"
            },
            {
              "trait_type": "Mask",
              "value": "Wraithveil Mask"
            },
            {
              "trait_type": "Eyes",
              "value": "Neon Void Gaze"
            },
            {
              "trait_type": "Top",
              "value": "Ecliptic Jacket"
            },
            {
              "trait_type": "Accessory",
              "value": "Static Emitter"
            },
            {
              "trait_type": "Symbols",
              "value": "Cyber Symbols"
            },
            {
              "trait_type": "Origin",
              "value": "Manifested from pure data corruption"
            },
            {
              "trait_type": "Mission",
              "value": "To protect the sanctity of digital entropy"
            },
            {
              "trait_type": "Secret Code",
              "value": "GLITCH-8983-ALPHA"
            },
            {
              "trait_type": "Agent Name",
              "value": "VertexStream Navigator"
            }
          ],
          "image": "https://na-assets.pinit.io/3jKpTiKAAtnJMLcQsNk82ua7crubQ86e8KfTQB9fKDwp/f4eb836b-82ec-441f-bfd6-e6ea0458092f/110",
          "properties": {
            "files": [
              {
                "uri": "https://na-assets.pinit.io/3jKpTiKAAtnJMLcQsNk82ua7crubQ86e8KfTQB9fKDwp/f4eb836b-82ec-441f-bfd6-e6ea0458092f/110"
              }
            ]
          }
        }"#.to_string()
    });
    
    // Parse the JSON
    let original_metadata = serde_json::from_str(&metadata_json)?;
    
    println!("⧂ STEP 1: Create Privacy Wrapper");
    println!("Creating privacy wrapper for NFT mint: {}", nft_mint);
    
    // In a real app, this would create an on-chain wrapper
    // For demo, we just simulate the process
    let wrapper_account = Keypair::new();
    println!("Wrapper account created: {}", wrapper_account.pubkey());
    
    // Apply different privacy levels
    demo_privacy_levels(&client, &original_metadata).await?;
    
    // Demo VRM privacy
    demo_vrm_privacy(&client, &nft_mint).await?;
    
    println!("\n⧂ Privacy protection complete");
    println!("Glitch Gang NFT is now protected by Project 89: Quantum Veil");
    
    Ok(())
}

/// Demonstrate different privacy levels
async fn demo_privacy_levels(
    client: &GlitchGangPrivacyClient,
    original_metadata: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⧂ STEP 2: Apply Privacy Levels");
    
    // Parse into our metadata type
    let metadata = serde_json::from_value(original_metadata.clone())?;
    
    // Test different privacy levels
    let privacy_levels = [
        PrivacyLevel::None,
        PrivacyLevel::Light,
        PrivacyLevel::Medium,
        PrivacyLevel::Heavy,
        PrivacyLevel::Complete,
    ];
    
    for level in &privacy_levels {
        println!("\nApplying privacy level: {:?}", level);
        
        // Apply privacy protection
        let protected_metadata = client.protect_metadata(&metadata, *level).await?;
        
        // Report on protected attributes
        match level {
            PrivacyLevel::None => {
                println!("No attributes protected");
            },
            PrivacyLevel::Light => {
                println!("Protected attributes: Secret Code, Agent Name");
            },
            PrivacyLevel::Medium => {
                println!("Protected attributes: Secret Code, Agent Name, Mission, Origin");
            },
            PrivacyLevel::Heavy | PrivacyLevel::Complete => {
                println!("Protected attributes: Secret Code, Agent Name, Mission, Origin, Accessory, Symbols");
            },
        }
        
        // Save the protected metadata (only for Medium level)
        if *level == PrivacyLevel::Medium {
            let output_file = "protected_metadata.json";
            let json = serde_json::to_string_pretty(&protected_metadata)?;
            fs::write(output_file, json)?;
            println!("Saved Medium privacy level metadata to {}", output_file);
        }
    }
    
    Ok(())
}

/// Demonstrate VRM privacy features
async fn demo_vrm_privacy(
    client: &GlitchGangPrivacyClient,
    nft_mint: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⧂ STEP 3: VRM Privacy Demonstration");
    println!("Creating example VRM data...");
    
    // Create example VRM data
    let vrm_data = VrmData {
        position: PositionData {
            x: 10.5,
            y: 2.0,
            z: -3.2,
        },
        rotation: RotationData {
            x: 0.0,
            y: 0.707,
            z: 0.0,
            w: 0.707,
        },
        voice: Some(VoiceData {
            frequency: vec![440.0, 880.0, 1320.0],
            amplitude: vec![0.8, 0.4, 0.2],
            pitch: 1.0,
            timbre: 0.5,
        }),
        gestures: vec![
            GestureData {
                name: "wave".to_string(),
                intensity: 0.8,
                speed: 1.2,
                joint_rotations: HashMap::new(),
            }
        ],
        animations: HashMap::new(),
        custom_data: HashMap::new(),
    };
    
    // Process for different viewers
    println!("\nProcessing VRM data for different viewers:");
    
    // Public view (anonymous)
    let public_view = client.process_vrm_data(&vrm_data, None, nft_mint)?;
    println!("- Public view position: ({:.1}, {:.1}, {:.1}) - heavily masked", 
        public_view.position.x, public_view.position.y, public_view.position.z);
    
    // Trusted agent view
    let trusted_agent = "agent1.glitch.gang";
    let trusted_view = client.process_vrm_data(&vrm_data, Some(trusted_agent), nft_mint)?;
    println!("- Trusted agent view position: ({:.1}, {:.1}, {:.1}) - partially masked", 
        trusted_view.position.x, trusted_view.position.y, trusted_view.position.z);
    
    // Owner view (unmasked)
    let owner_view = client.process_vrm_data(&vrm_data, Some(&client.owner_keypair.pubkey().to_string()), nft_mint)?;
    println!("- Owner view position: ({:.1}, {:.1}, {:.1}) - unmasked", 
        owner_view.position.x, owner_view.position.y, owner_view.position.z);
    
    Ok(())
}

/// Get or create a test keypair
fn get_or_create_keypair() -> Result<Keypair, Box<dyn std::error::Error>> {
    // First, try to load from file
    let keypair_file = "test_wallet.json";
    let keypair = if let Ok(keypair_bytes) = fs::read_to_string(keypair_file) {
        // Parse keypair from file
        let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_bytes)?;
        Keypair::from_bytes(&keypair_bytes)?
    } else {
        // Generate a new keypair
        let keypair = Keypair::new();
        
        // Save it for future use
        let keypair_bytes = keypair.to_bytes();
        fs::write(keypair_file, serde_json::to_string(&keypair_bytes)?)?;
        
        keypair
    };
    
    Ok(keypair)
}
