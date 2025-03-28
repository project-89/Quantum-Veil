use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;
use std::fs::File;
use std::io::Read;

use project_89::{
    GlitchGangPrivacyClient,
    GlitchGangMetadata,
    PrivacyLevel,
    EntropySource,
};

/// Example program to wrap an existing NFT with privacy features
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    println!("\n⧂ PROJECT 89: QUANTUM VEIL ⧂");
    println!("NFT Privacy Wrapper Example");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    let (keypair_path, nft_mint, metadata_path) = parse_args(&args)?;
    
    // Load keypair
    let keypair = load_keypair(keypair_path)?;
    println!("Using wallet: {}", keypair.pubkey());
    
    // Parse NFT mint address
    let nft_mint_pubkey = Pubkey::from_str(&nft_mint)?;
    println!("NFT mint: {}", nft_mint_pubkey);
    
    // Load metadata file
    let metadata = load_metadata(metadata_path)?;
    println!("Loaded metadata for: {}", metadata.name);
    
    // Create privacy client
    let client = GlitchGangPrivacyClient::new(
        "https://api.devnet.solana.com", // Use devnet for testing
        keypair,
    );
    
    // Create wrapper account
    println!("\nCreating privacy wrapper...");
    let wrapper_account = client.create_wrapper(&nft_mint_pubkey, &metadata).await?;
    println!("✓ Wrapper created: {}", wrapper_account);
    
    // Apply privacy protections to metadata
    println!("\nApplying privacy protections...");
    let protected_metadata = client.protect_metadata(&metadata, PrivacyLevel::Medium).await?;
    
    // Save protected metadata
    let output_path = format!("protected_{}.json", nft_mint);
    
    println!("\nSaving protected metadata to {}", output_path);
    std::fs::write(
        &output_path,
        serde_json::to_string_pretty(&protected_metadata)?,
    )?;
    println!("✓ Protected metadata saved");
    
    // Print summary
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ NFT successfully wrapped with privacy protection");
    println!("  - Wrapper account: {}", wrapper_account);
    println!("  - Protected metadata: {}", output_path);
    println!("  - Privacy level: {:?}", PrivacyLevel::Medium);
    println!("");
    println!("You can now update the NFT metadata URI to point to the protected metadata");
    println!("Or use the protected metadata with your Glitch Gang applications");
    
    Ok(())
}

/// Parse command line arguments
fn parse_args(args: &[String]) -> Result<(&str, &str, &str), Box<dyn std::error::Error>> {
    if args.len() < 4 {
        println!("Usage: wrap_nft <keypair_path> <nft_mint> <metadata_path>");
        println!("  - keypair_path: Path to the wallet keypair file");
        println!("  - nft_mint: Mint address of the NFT to wrap");
        println!("  - metadata_path: Path to the NFT metadata JSON file");
        std::process::exit(1);
    }
    
    Ok((&args[1], &args[2], &args[3]))
}

/// Load keypair from file
fn load_keypair(keypair_path: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    let mut file = File::open(keypair_path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    
    let keypair = if keypair_path.ends_with(".json") {
        // Parse JSON format
        let keypair_bytes: Vec<u8> = serde_json::from_reader(file)?;
        Keypair::from_bytes(&keypair_bytes)?
    } else {
        // Parse binary format
        Keypair::from_bytes(&bytes)?
    };
    
    Ok(keypair)
}

/// Load metadata from file
fn load_metadata(metadata_path: &str) -> Result<GlitchGangMetadata, Box<dyn std::error::Error>> {
    let file = File::open(metadata_path)?;
    let metadata: GlitchGangMetadata = serde_json::from_reader(file)?;
    Ok(metadata)
}
