use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// VRM data types that can be masked/protected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VrmDataType {
    /// Position in 3D space
    Position,
    /// Rotation (quaternion)
    Rotation,
    /// Scale
    Scale,
    /// Voice audio data
    Voice,
    /// Gesture animations
    Gesture,
    /// Pre-defined animations
    Animation,
    /// Interaction behaviors
    Interaction,
    /// Custom data type
    Custom(String),
}

/// VRM position data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionData {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
}

/// VRM rotation data (quaternion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationData {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32,
    /// Z component
    pub z: f32,
    /// W component
    pub w: f32,
}

/// VRM voice data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceData {
    /// Frequency components
    pub frequency: Vec<f32>,
    /// Amplitude components
    pub amplitude: Vec<f32>,
    /// Voice pitch
    pub pitch: f32,
    /// Voice timbre
    pub timbre: f32,
}

/// VRM gesture data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureData {
    /// Gesture name
    pub name: String,
    /// Gesture intensity
    pub intensity: f32,
    /// Gesture speed
    pub speed: f32,
    /// Joint rotations for this gesture
    pub joint_rotations: HashMap<String, RotationData>,
}

/// Combined VRM data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrmData {
    /// Position in 3D space
    pub position: PositionData,
    /// Rotation (quaternion)
    pub rotation: RotationData,
    /// Voice data (optional)
    pub voice: Option<VoiceData>,
    /// Gesture animations
    pub gestures: Vec<GestureData>,
    /// Animation parameters
    pub animations: HashMap<String, f32>,
    /// Custom data
    pub custom_data: HashMap<String, serde_json::Value>,
}

impl VrmData {
    /// Create a new VRM data instance with default values
    pub fn new() -> Self {
        Self {
            position: PositionData { x: 0.0, y: 0.0, z: 0.0 },
            rotation: RotationData { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            voice: None,
            gestures: Vec::new(),
            animations: HashMap::new(),
            custom_data: HashMap::new(),
        }
    }
    
    /// Get distance between two VRM positions
    pub fn distance(&self, other: &VrmData) -> f32 {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let dz = self.position.z - other.position.z;
        
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    /// Add a gesture
    pub fn add_gesture(&mut self, gesture: GestureData) {
        self.gestures.push(gesture);
    }
    
    /// Set animation parameter
    pub fn set_animation(&mut self, name: &str, value: f32) {
        self.animations.insert(name.to_string(), value);
    }
    
    /// Add custom data
    pub fn add_custom_data(&mut self, key: &str, value: serde_json::Value) {
        self.custom_data.insert(key.to_string(), value);
    }
}
