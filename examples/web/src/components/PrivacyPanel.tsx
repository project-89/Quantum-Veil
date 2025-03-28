import React, { useState, useEffect, useRef } from 'react';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { GlitchGangPrivacyClient, PrivacyLevel, VrmData } from './daydreams-integration';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
import { VRMLoader } from '@pixiv/three-vrm';

// NFT Privacy Panel Component
const GlitchGangPrivacyPanel = ({ nftData, walletPublicKey }) => {
  const [metadata, setMetadata] = useState(null);
  const [protectedMetadata, setProtectedMetadata] = useState(null);
  const [privacyLevel, setPrivacyLevel] = useState(PrivacyLevel.Medium);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');
  const [viewerMode, setViewerMode] = useState('public');
  const [vrmLoaded, setVrmLoaded] = useState(false);
  
  const canvasRef = useRef(null);
  const sceneRef = useRef(null);
  const vrmModelRef = useRef(null);
  const clientRef = useRef(null);
  
  // Initialize privacy client
  useEffect(() => {
    if (nftData && walletPublicKey) {
      const dummyKeypair = Keypair.generate(); // In a real app, this would come from the wallet
      
      clientRef.current = new GlitchGangPrivacyClient(
        'https://api.devnet.solana.com',
        dummyKeypair,
        'GlchWrapperProgram111111111111111111111111111'
      );
      
      // Load NFT metadata
      fetchMetadata();
    }
  }, [nftData, walletPublicKey]);
  
  // Fetch NFT metadata
  const fetchMetadata = async () => {
    if (!clientRef.current) return;
    
    setLoading(true);
    setMessage('Loading metadata...');
    
    try {
      const metadata = await clientRef.current.fetchMetadata(nftData.metadataUri);
      setMetadata(metadata);
      
      // Apply current privacy level
      applyPrivacyLevel(metadata, privacyLevel);
      
      setMessage('Metadata loaded successfully');
    } catch (error) {
      console.error('Failed to load metadata:', error);
      setMessage(`Error: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };
  
  // Apply privacy level to metadata
  const applyPrivacyLevel = (metadata, level) => {
    if (!clientRef.current || !metadata) return;
    
    setLoading(true);
    setMessage(`Applying ${PrivacyLevel[level]} privacy level...`);
    
    try {
      // Apply privacy protections
      const protected_metadata = clientRef.current.protectMetadata(metadata, level);
      
      // Add VRM privacy settings if not already present
      if (!protected_metadata.privateData?.vrmConfig) {
        const vrmUri = 'https://models.glitchgang.io/avatars/default.vrm';
        const finalMetadata = clientRef.current.addVrmPrivacy(protected_metadata, vrmUri);
        setProtectedMetadata(finalMetadata);
      } else {
        setProtectedMetadata(protected_metadata);
      }
      
      setMessage(`Privacy level set to ${PrivacyLevel[level]}`);
      
      // Update 3D view if model is loaded
      if (vrmLoaded) {
        updateVrmPrivacy();
      }
    } catch (error) {
      console.error('Failed to apply privacy level:', error);
      setMessage(`Error: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };
  
  // Handle privacy level change
  const handlePrivacyLevelChange = (e) => {
    const newLevel = parseInt(e.target.value);
    setPrivacyLevel(newLevel);
    
    if (metadata) {
      applyPrivacyLevel(metadata, newLevel);
    }
  };
  
  // Handle viewer mode change
  const handleViewerModeChange = (e) => {
    setViewerMode(e.target.value);
    
    if (vrmLoaded) {
      updateVrmPrivacy();
    }
  };
  
  // Initialize 3D scene
  useEffect(() => {
    if (!canvasRef.current || sceneRef.current) return;
    
    const canvas = canvasRef.current;
    const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    renderer.setSize(canvas.clientWidth, canvas.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.outputEncoding = THREE.sRGBEncoding;
    
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x121214);
    
    const camera = new THREE.PerspectiveCamera(
      45, canvas.clientWidth / canvas.clientHeight, 0.1, 100
    );
    camera.position.set(0, 1.5, 3);
    
    const controls = new OrbitControls(camera, renderer.domElement);
    controls.target.set(0, 1, 0);
    controls.update();
    
    // Add lights
    const light = new THREE.DirectionalLight(0xffffff, 1);
    light.position.set(1, 1, 1);
    scene.add(light);
    
    scene.add(new THREE.AmbientLight(0xffffff, 0.5));
    
    // Grid helper
    const gridHelper = new THREE.GridHelper(10, 10);
    scene.add(gridHelper);
    
    sceneRef.current = { scene, camera, renderer, controls };
    
    // Animation loop
    const animate = () => {
      requestAnimationFrame(animate);
      
      if (vrmModelRef.current) {
        vrmModelRef.current.update(0.016); // Update VRM with ~60fps time delta
      }
      
      renderer.render(scene, camera);
    };
    
    animate();
    
    // Load VRM model if we have metadata
    if (protectedMetadata?.privateData?.vrmConfig) {
      loadVrmModel(protectedMetadata.privateData.vrmConfig.modelUri);
    }
    
    // Handle resize
    const handleResize = () => {
      if (!canvasRef.current || !sceneRef.current) return;
      
      const width = canvas.clientWidth;
      const height = canvas.clientHeight;
      
      sceneRef.current.camera.aspect = width / height;
      sceneRef.current.camera.updateProjectionMatrix();
      sceneRef.current.renderer.setSize(width, height);
    };
    
    window.addEventListener('resize', handleResize);
    
    return () => {
      window.removeEventListener('resize', handleResize);
      
      if (sceneRef.current) {
        sceneRef.current.renderer.dispose();
      }
    };
  }, []);
  
  // Load VRM model when metadata changes
  useEffect(() => {
    if (protectedMetadata?.privateData?.vrmConfig && canvasRef.current && sceneRef.current) {
      loadVrmModel(protectedMetadata.privateData.vrmConfig.modelUri);
    }
  }, [protectedMetadata]);
  
  // Load VRM model
  const loadVrmModel = async (modelUri) => {
    if (!sceneRef.current) return;
    
    setLoading(true);
    setMessage('Loading VRM model...');
    
    try {
      // Clear previous model
      if (vrmModelRef.current && vrmModelRef.current.scene) {
        sceneRef.current.scene.remove(vrmModelRef.current.scene);
      }
      
      // Use a placeholder model for demo
      const actualUri = 'https://cdn.glitch.me/3b2e7e42-e7a3-4f80-a902-a1c4b9d79754/AvatarSample_B.vrm?v=1656041929868';
      
      const loader = new VRMLoader();
      const gltf = await loader.loadAsync(actualUri);
      
      const vrm = gltf.userData.vrm;
      vrm.scene.position.set(0, 0, 0);
      
      sceneRef.current.scene.add(vrm.scene);
      vrmModelRef.current = vrm;
      
      setVrmLoaded(true);
      setMessage('VRM model loaded successfully');
      
      // Apply initial privacy
      updateVrmPrivacy();
    } catch (error) {
      console.error('Failed to load VRM model:', error);
      setMessage(`Error loading VRM: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };
  
  // Update VRM privacy settings
  const updateVrmPrivacy = () => {
    if (!vrmModelRef.current || !clientRef.current) return;
    
    // Sample VRM data
    const vrmData = {
      position: { x: 0, y: 0, z: 0 },
      rotation: { x: 0, y: 0, z: 0, w: 1 },
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
    
    // Apply privacy mask based on viewer mode
    const viewerId = viewerMode === 'trusted' ? 'agent1.glitch.gang' : undefined;
    const maskedData = clientRef.current.applyVrmPrivacyMask(vrmData, viewerId);
    
    // Apply masked position to the model
    vrmModelRef.current.scene.position.set(
      maskedData.position.x,
      maskedData.position.y,
      maskedData.position.z
    );
    
    // Apply masked rotation to the model
    const quaternion = new THREE.Quaternion(
      maskedData.rotation.x,
      maskedData.rotation.y,
      maskedData.rotation.z,
      maskedData.rotation.w
    );
    vrmModelRef.current.scene.quaternion.copy(quaternion);
    
    // Apply random pose based on privacy level
    if (privacyLevel > PrivacyLevel.Light) {
      const blendShapeProxy = vrmModelRef.current.blendShapeProxy;
      
      if (blendShapeProxy) {
        // Apply random expressions based on privacy level
        const expressionIntensity = privacyLevel * 0.25;
        
        blendShapeProxy.setValue('blink_l', Math.random() * expressionIntensity);
        blendShapeProxy.setValue('blink_r', Math.random() * expressionIntensity);
        
        if (privacyLevel >= Privac
