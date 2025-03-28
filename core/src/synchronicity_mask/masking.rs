use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use super::vrm_data::{PositionData, RotationData, VoiceData, GestureData};

/// Add noise to position data
pub fn add_position_noise(position: &mut PositionData, intensity: f32, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    
    position.x += (rng.gen::<f32>() - 0.5) * 2.0 * intensity * 10.0; // Scale for position
    position.y += (rng.gen::<f32>() - 0.5) * 2.0 * intensity * 10.0;
    position.z += (rng.gen::<f32>() - 0.5) * 2.0 * intensity * 10.0;
}

/// Add noise to quaternion rotation data
pub fn add_rotation_noise(rotation: &mut RotationData, intensity: f32, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Add small random rotation
    let noise_angle = intensity * std::f32::consts::PI * rng.gen::<f32>();
    let noise_axis_x = rng.gen::<f32>() * 2.0 - 1.0;
    let noise_axis_y = rng.gen::<f32>() * 2.0 - 1.0;
    let noise_axis_z = rng.gen::<f32>() * 2.0 - 1.0;
    
    // Normalize axis
    let mag = (noise_axis_x.powi(2) + noise_axis_y.powi(2) + noise_axis_z.powi(2)).sqrt();
    let noise_axis_x = noise_axis_x / mag;
    let noise_axis_y = noise_axis_y / mag;
    let noise_axis_z = noise_axis_z / mag;
    
    // Create noise quaternion
    let sin_half_angle = (noise_angle / 2.0).sin();
    let cos_half_angle = (noise_angle / 2.0).cos();
    
    let noise_quat_x = noise_axis_x * sin_half_angle;
    let noise_quat_y = noise_axis_y * sin_half_angle;
    let noise_quat_z = noise_axis_z * sin_half_angle;
    let noise_quat_w = cos_half_angle;
    
    // Apply noise quaternion (quaternion multiplication)
    let original_x = rotation.x;
    let original_y = rotation.y;
    let original_z = rotation.z;
    let original_w = rotation.w;
    
    rotation.x = original_w * noise_quat_x + original_x * noise_quat_w + original_y * noise_quat_z - original_z * noise_quat_y;
    rotation.y = original_w * noise_quat_y - original_x * noise_quat_z + original_y * noise_quat_w + original_z * noise_quat_x;
    rotation.z = original_w * noise_quat_z + original_x * noise_quat_y - original_y * noise_quat_x + original_z * noise_quat_w;
    rotation.w = original_w * noise_quat_w - original_x * noise_quat_x - original_y * noise_quat_y - original_z * noise_quat_z;
    
    // Normalize quaternion
    let mag = (rotation.x.powi(2) + rotation.y.powi(2) + rotation.z.powi(2) + rotation.w.powi(2)).sqrt();
    rotation.x /= mag;
    rotation.y /= mag;
    rotation.z /= mag;
    rotation.w /= mag;
}

/// Add noise to voice data
pub fn add_voice_noise(voice: &mut VoiceData, intensity: f32, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Add noise to frequency components
    for freq in &mut voice.frequency {
        *freq += (rng.gen::<f32>() - 0.5) * 2.0 * intensity * 100.0;
        *freq = freq.max(0.0); // Frequencies must be positive
    }
    
    // Add noise to amplitude components
    for amp in &mut voice.amplitude {
        *amp += (rng.gen::<f32>() - 0.5) * 2.0 * intensity;
        *amp = amp.max(0.0); // Amplitudes must be positive
    }
    
    // Add noise to pitch and timbre
    voice.pitch += (rng.gen::<f32>() - 0.5) * 2.0 * intensity * 50.0;
    voice.pitch = voice.pitch.max(0.0); // Pitch must be positive
    
    voice.timbre += (rng.gen::<f32>() - 0.5) * 2.0 * intensity;
    voice.timbre = voice.timbre.max(0.0).min(1.0); // Timbre is normalized 0.0-1.0
}

/// Add noise to gesture data
pub fn add_gesture_noise(gesture: &mut GestureData, intensity: f32, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Add noise to gesture intensity
    gesture.intensity += (rng.gen::<f32>() - 0.5) * 2.0 * intensity;
    gesture.intensity = gesture.intensity.max(0.0).min(1.0); // Intensity normalized 0.0-1.0
    
    // Add noise to gesture speed
    gesture.speed += (rng.gen::<f32>() - 0.5) * 2.0 * intensity * gesture.speed;
    gesture.speed = gesture.speed.max(0.1); // Speed must be positive
    
    // Add noise to joint rotations
    for (_, rotation) in gesture.joint_rotations.iter_mut() {
        // Use a different seed for each joint
        let joint_seed = seed.wrapping_add(rotation.w as u64);
        add_rotation_noise(rotation, intensity * 0.5, joint_seed);
    }
}

/// Create privacy-preserving randomized data
pub fn create_random_position(seed: u64) -> PositionData {
    let mut rng = StdRng::seed_from_u64(seed);
    
    PositionData {
        x: rng.gen_range(-10.0..10.0),
        y: rng.gen_range(-2.0..5.0), // Typical avatar height range
        z: rng.gen_range(-10.0..10.0),
    }
}

/// Create privacy-preserving randomized rotation
pub fn create_random_rotation(seed: u64) -> RotationData {
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Generate random quaternion components
    let x = rng.gen_range(-1.0..1.0);
    let y = rng.gen_range(-1.0..1.0);
    let z = rng.gen_range(-1.0..1.0);
    let w = rng.gen_range(-1.0..1.0);
    
    // Normalize quaternion
    let mag = (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt();
    
    RotationData {
        x: x / mag,
        y: y / mag,
        z: z / mag,
        w: w / mag,
    }
}
