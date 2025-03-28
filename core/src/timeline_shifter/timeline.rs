use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Timeline types for metadata fragmentation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimelineType {
    /// Primary timeline (core metadata)
    Primary,
    /// Identity timeline (personal info)
    Identity,
    /// Activity timeline (actions and behaviors)
    Activity,
    /// Social timeline (relationships)
    Social,
    /// Financial timeline (transactions)
    Financial,
    /// Custom timeline
    Custom(String),
}

impl TimelineType {
    /// Get description of timeline type
    pub fn description(&self) -> String {
        match self {
            TimelineType::Primary => "Primary timeline - core metadata".to_string(),
            TimelineType::Identity => "Identity timeline - personal info".to_string(),
            TimelineType::Activity => "Activity timeline - actions and behaviors".to_string(),
            TimelineType::Social => "Social timeline - relationships".to_string(),
            TimelineType::Financial => "Financial timeline - transactions".to_string(),
            TimelineType::Custom(name) => format!("Custom timeline - {}", name),
        }
    }
    
    /// Get recommended storage type for this timeline
    pub fn recommended_storage(&self) -> &'static str {
        match self {
            TimelineType::Primary => "Onchain", // Primary data is critical, needs blockchain security
            TimelineType::Identity => "IPFS",   // Identity data needs decentralization
            TimelineType::Activity => "IPFS",   // Activity data is frequently accessed
            TimelineType::Social => "Arweave",  // Social data needs permanence
            TimelineType::Financial => "ShadowRealm", // Financial data needs privacy
            TimelineType::Custom(_) => "IPFS",  // Default for custom timelines
        }
    }
    
    /// Get recommended privacy level for this timeline
    pub fn recommended_privacy_level(&self) -> u8 {
        match self {
            TimelineType::Primary => 1,    // Low privacy - needs accessibility
            TimelineType::Identity => 3,   // High privacy - sensitive personal data
            TimelineType::Activity => 2,   // Medium privacy - behavioral patterns
            TimelineType::Social => 2,     // Medium privacy - relationship data
            TimelineType::Financial => 4,  // Max privacy - financial data
            TimelineType::Custom(_) => 2,  // Default medium privacy
        }
    }
    
    /// Get all standard timeline types
    pub fn standard_timelines() -> Vec<TimelineType> {
        vec![
            TimelineType::Primary,
            TimelineType::Identity,
            TimelineType::Activity,
            TimelineType::Social,
            TimelineType::Financial,
        ]
    }
    
    /// Get default timeline distribution
    pub fn default_distribution() -> HashMap<TimelineType, f32> {
        let mut distribution = HashMap::new();
        distribution.insert(TimelineType::Primary, 0.4);
        distribution.insert(TimelineType::Identity, 0.15);
        distribution.insert(TimelineType::Activity, 0.15);
        distribution.insert(TimelineType::Social, 0.15);
        distribution.insert(TimelineType::Financial, 0.15);
        distribution
    }
}
