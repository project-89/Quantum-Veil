use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::str::FromStr;
use std::collections::HashMap;

use project_89::{
    GlitchGangPrivacyClient,
    VrmData,
    PositionData,
    RotationData,
    VoiceData,
    GestureData,
    PrivacyLevel,
};

/// Example program to demonstrate VRM protection features
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    println!("\n⧂ PROJECT 89: QUANTUM VEIL ⧂");
    println!("VRM Privacy Protection Example");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    let (nft_mint, keypair_path, trusted_agent) = parse_args(&args)?;
    
    // Create or load keypair
    let keypair = if keypair_path.is_empty() {
        println!("Generating new keypair for demo...");
        Keypair::new()
    } else {
        println!("Loading keypair from {}...", keypair_path);
        let keypair_bytes = std::fs::read(keypair_path)?;
        Keypair::from_bytes(&keypair_bytes)?
    };
    
    println!("Using wallet: {}", keypair.pubkey());
    
    // Parse NFT mint address
    let nft_mint_pubkey = Pubkey::from_str(&nft_mint)?;
    println!("NFT mint: {}", nft_mint_pubkey);
    
    // Create privacy client
    let client = GlitchGangPrivacyClient::new(
        "https://api.devnet.solana.com", // Use devnet for testing
        keypair,
    );
    
    // Create example VRM data
    println!("\nCreating example VRM data...");
    let vrm_data = create_example_vrm_data();
    
    // Demonstrate privacy features
    println!("\nDemonstrating privacy masking at different levels...");
    
    // Process VRM data for different privacy levels
    demonstrate_privacy_levels(&client, &vrm_data, &nft_mint_pubkey)?;
    
    // Process VRM data for different viewers
    demonstrate_viewer_privacy(&client, &vrm_data, &nft_mint_pubkey, trusted_agent)?;
    
    // Apply privacy to metadata
    demonstrate_vrm_metadata_privacy(&client, &nft_mint_pubkey)?;
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ VRM privacy demonstration complete");
    println!("");
    println!("For more information, see the Project 89: Quantum Veil documentation");
    
    Ok(())
}

/// Parse command line arguments
fn parse_args(args: &[String]) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let nft_mint = if args.len() > 1 { args[1].clone() } else { "5VLAQFhyYQnQPCGQm8kLZ8pzgkyCgdszJMbJ3GbJ4kLJ".to_string() };
    let keypair_path = if args.len() > 2 { args[2].clone() } else { "".to_string() };
    let trusted_agent = if args.len() > 3 { args[3].clone() } else { "agent1.glitch.gang".to_string() };
    
    Ok((nft_mint, keypair_path, trusted_agent))
}

/// Create example VRM data for demonstration
fn create_example_vrm_data() -> VrmData {
    VrmData {
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
    }
}

/// Demonstrate VRM privacy at different privacy levels
fn demonstrate_privacy_levels(
    client: &GlitchGangPrivacyClient,
    vrm_data: &VrmData,
    nft_mint: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nOriginal VRM position: ({:.1}, {:.1}, {:.1})", 
        vrm_data.position.x, vrm_data.position.y, vrm_data.position.z);
    
    let privacy_levels = [
        PrivacyLevel::None,
        PrivacyLevel::Light,
        PrivacyLevel::Medium,
        PrivacyLevel::Heavy,
        PrivacyLevel::Complete,
    ];
    
    for level in &privacy_levels {
        // Apply privacy mask
        let masked_data = client.process_vrm_data_with_level(vrm_data, *level, nft_mint)?;
        
        println!("{:?} privacy: ({:.1}, {:.1}, {:.1})", 
            level,
            masked_data.position.x, masked_data.position.y, masked_data.position.z);
    }
    
    Ok(())
}

/// Demonstrate VRM privacy for different viewers
fn demonstrate_viewer_privacy(
    client: &GlitchGangPrivacyClient,
    vrm_data: &VrmData,
    nft_mint: &Pubkey,
    trusted_agent: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nPrivacy by viewer type:");
    
    // Public view (no viewer ID)
    let public_view = client.process_vrm_data(vrm_data, None, nft_mint)?;
    println!("Public view position: ({:.1}, {:.1}, {:.1}) - heavily masked", 
        public_view.position.x, public_view.position.y, public_view.position.z);
    
    // Trusted agent view
    let trusted_view = client.process_vrm_data(vrm_data, Some(&trusted_agent), nft_mint)?;
    println!("Trusted agent view position: ({:.1}, {:.1}, {:.1}) - partially masked", 
        trusted_view.position.x, trusted_view.position.y, trusted_view.position.z);
    
    // Owner view
    let owner_view = client.process_vrm_data(vrm_data, Some(&client.owner_keypair.pubkey().to_string()), nft_mint)?;
    println!("Owner view position: ({:.1}, {:.1}, {:.1}) - unmasked", 
        owner_view.position.x, owner_view.position.y, owner_view.position.z);
    
    Ok(())
}

/// Demonstrate VRM metadata privacy protection
fn demonstrate_vrm_metadata_privacy(
    client: &GlitchGangPrivacyClient, 
    nft_mint: &Pubkey
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nAdding VRM privacy to metadata...");
    
    // Create example metadata
    let mut metadata = client.fetch_metadata_from_mint(nft_mint).await?;
    
    // Add VRM privacy settings
    let model_uri = "https://models.glitch.gang/avatar699.vrm";
    client.add_vrm_privacy(&mut metadata, model_uri)?;
    
    println!("✓ VRM privacy settings added to metadata");
    println!("  - Model URI: {}", model_uri);
    println!("  - Privacy settings applied for position, voice, and gestures");
    
    Ok(())
}
