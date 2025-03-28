/**
 * VRM data models for Glitch Gang NFTs
 */

/**
 * Privacy level enum
 */
export enum PrivacyLevel {
  None = 0,
  Light = 1,
  Medium = 2, 
  Heavy = 3,
  Complete = 4
}

/**
 * VRM position data
 */
export interface PositionData {
  x: number;
  y: number;
  z: number;
}

/**
 * VRM rotation data (quaternion)
 */
export interface RotationData {
  x: number;
  y: number;
  z: number;
  w: number;
}

/**
 * VRM voice data
 */
export interface VoiceData {
  frequency: number[];
  amplitude: number[];
  pitch: number;
  timbre: number;
}

/**
 * VRM gesture data
 */
export interface GestureData {
  name: string;
  intensity: number;
  speed: number;
  jointRotations: Map<string, RotationData>;
}

/**
 * Combined VRM data
 */
export interface VrmData {
  position: PositionData;
  rotation: RotationData;
  voice?: VoiceData;
  gestures: GestureData[];
  animations: Map<string, number>;
  customData: Map<string, any>;
}

/**
 * VRM data types that can be masked/protected
 */
export enum VrmDataType {
  Position = "Position",
  Rotation = "Rotation",
  Scale = "Scale",
  Voice = "Voice",
  Gesture = "Gesture",
  Animation = "Animation",
  Interaction = "Interaction"
}

/**
 * Access permission for VRM data
 */
export enum AccessPermission {
  Public = "Public",
  Restricted = "Restricted",
  OwnerOnly = "OwnerOnly"
}
