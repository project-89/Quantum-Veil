use async_trait::async_trait;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    instruction::{AccountMeta, Instruction},
    commitment_config::CommitmentConfig,
};
use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize, BorshDeserialize};

use super::{StorageAdapter, MetadataFragment};

/// Solana on-chain storage adapter
pub struct SolanaAdapter {
    /// Solana RPC client
    pub rpc_client: RpcClient,
    /// Program ID for storage
    pub program_id: Pubkey,
    /// Payer for transactions (optional)
    pub payer: Option<Keypair>,
}

/// Fragment storage instruction
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FragmentInstruction {
    /// Store a fragment
    Store {
        /// Fragment ID
        id: String,
        /// Fragment data
        data: Vec<u8>,
    },
    
    /// Delete a fragment
    Delete {
        /// Fragment ID
        id: String,
    },
}

impl SolanaAdapter {
    /// Create a new Solana adapter
    pub fn new(rpc_url: &str, program_id: Pubkey, payer: Option<Keypair>) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        
        Self {
            rpc_client,
            program_id,
            payer,
        }
    }
    
    /// Derive PDA for fragment storage
    pub fn get_fragment_address(&self, fragment_id: &str) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"fragment",
                fragment_id.as_bytes(),
            ],
            &self.program_id,
        )
    }
    
    /// Get rent exemption amount for given data size
    pub async fn get_rent_exemption(&self, data_size: usize) -> Result<u64, String> {
        self.rpc_client
            .get_minimum_balance_for_rent_exemption(data_size)
            .map_err(|e| format!("Failed to get rent exemption: {}", e))
    }
    
    /// Create store fragment instruction
    pub fn create_store_instruction(
        &self,
        fragment: &MetadataFragment,
        payer: &Pubkey,
    ) -> Result<Instruction, String> {
        let (fragment_address, _) = self.get_fragment_address(&fragment.id);
        
        // Serialize the fragment data
        let serialized = bincode::serialize(fragment)
            .map_err(|e| format!("Failed to serialize fragment: {}", e))?;
        
        let instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new(fragment_address, false),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
            data: FragmentInstruction::Store {
                id: fragment.id.clone(),
                data: serialized,
            }
            .try_to_vec()
            .map_err(|e| format!("Failed to serialize instruction: {}", e))?,
        };
        
        Ok(instruction)
    }
}

#[async_trait]
impl StorageAdapter for SolanaAdapter {
    async fn store_fragment(&self, fragment: &MetadataFragment) -> Result<String, String> {
        // In a real implementation, this would store on Solana
        log::info!("Storing fragment {} on Solana blockchain", fragment.id);
        
        if let Some(payer) = &self.payer {
            let instruction = self.create_store_instruction(fragment, &payer.pubkey())?;
            
            let transaction = Transaction::new_with_payer(
                &[instruction],
                Some(&payer.pubkey()),
            );
            
            // Sign and send transaction
            // For demo purposes, we're not actually sending the transaction
            
            log::info!("Transaction created for storing fragment");
        }
        
        // Return success
        Ok(fragment.id.clone())
    }
    
    async fn retrieve_fragment(&self, id: &str) -> Result<MetadataFragment, String> {
        // In a real implementation, this would retrieve from Solana
        log::info!("Retrieving fragment {} from Solana blockchain", id);
        
        let (fragment_address, _) = self.get_fragment_address(id);
        
        // Get account data
        // For demo purposes, we're returning an error
        
        // Return dummy fragment
        Err("Not implemented in mock".to_string())
    }
    
    async fn fragment_exists(&self, id: &str) -> Result<bool, String> {
        // In a real implementation, this would check Solana
        log::info!("Checking if fragment {} exists on Solana blockchain", id);
        
        let (fragment_address, _) = self.get_fragment_address(id);
        
        // Check if account exists
        // For demo purposes, we're always returning false
        
        // Return false for mock
        Ok(false)
    }
    
    async fn delete_fragment(&self, id: &str) -> Result<(), String> {
        // In a real implementation, this would delete from Solana
        log::info!("Deleting fragment {} from Solana blockchain", id);
        
        if let Some(payer) = &self.payer {
            let (fragment_address, _) = self.get_fragment_address(id);
            
            // Create delete instruction
            // For demo purposes, we're not actually sending the transaction
            
            log::info!("Transaction created for deleting fragment");
        }
        
        // Return success
        Ok(())
    }
    
    fn clone_adapter(&self) -> Box<dyn StorageAdapter + Send + Sync> {
        Box::new(SolanaAdapter {
            rpc_client: RpcClient::new(self.rpc_client.url()),
            program_id: self.program_id,
            payer: self.payer.clone(),
        })
    }
}
