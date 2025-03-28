use serde::{Serialize, Deserialize};

/// Privacy level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PrivacyLevel {
    /// No privacy protection
    None = 0,
    /// Light privacy - minor obfuscation
    Light = 1,
    /// Medium privacy - noticeable obfuscation
    Medium = 2,
    /// Heavy privacy - significant obfuscation
    Heavy = 3,
    /// Complete privacy - full obfuscation
    Complete = 4,
}

impl PrivacyLevel {
    /// Get intensity factor based on privacy level (0.0 - 1.0)
    pub fn intensity_factor(&self) -> f32 {
        match self {
            PrivacyLevel::None => 0.0,
            PrivacyLevel::Light => 0.25,
            PrivacyLevel::Medium => 0.5,
            PrivacyLevel::Heavy => 0.75,
            PrivacyLevel::Complete => 1.0,
        }
    }
    
    /// Get text description of privacy level
    pub fn description(&self) -> &'static str {
        match self {
            PrivacyLevel::None => "No privacy protection",
            PrivacyLevel::Light => "Light privacy - minor obfuscation",
            PrivacyLevel::Medium => "Medium privacy - noticeable obfuscation",
            PrivacyLevel::Heavy => "Heavy privacy - significant obfuscation",
            PrivacyLevel::Complete => "Complete privacy - full obfuscation",
        }
    }
    
    /// Get from numeric value
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(PrivacyLevel::None),
            1 => Some(PrivacyLevel::Light),
            2 => Some(PrivacyLevel::Medium),
            3 => Some(PrivacyLevel::Heavy),
            4 => Some(PrivacyLevel::Complete),
            _ => None,
        }
    }
}

/// Access permission for VRM data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AccessPermission {
    /// Anyone can access the data
    Public,
    /// Only specified agents can access the data
    Restricted(Vec<String>),
    /// Only the owner can access the data
    OwnerOnly,
}

impl AccessPermission {
    /// Check if an agent has access
    pub fn has_access(&self, agent_id: Option<&str>, owner: &str) -> bool {
        match self {
            AccessPermission::Public => true,
            AccessPermission::Restricted(allowed_agents) => {
                if let Some(agent) = agent_id {
                    allowed_agents.contains(&agent.to_string()) || agent == owner
                } else {
                    false
                }
            },
            AccessPermission::OwnerOnly => {
                if let Some(agent) = agent_id {
                    agent == owner
                } else {
                    false
                }
            },
        }
    }
    
    /// Get text description of access permission
    pub fn description(&self) -> String {
        match self {
            AccessPermission::Public => "Public - anyone can access".to_string(),
            AccessPermission::Restricted(agents) => {
                format!("Restricted - only {} agents can access", agents.len())
            },
            AccessPermission::OwnerOnly => "Owner only - restricted to NFT owner".to_string(),
        }
    }
}
