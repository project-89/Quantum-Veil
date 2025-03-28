import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { 
  GlitchGangPrivacyClient, 
  PrivacyLevel, 
  VrmData, 
  PositionData,
  RotationData,
  VoiceData,
  GestureData
} from 'project-89-quantum-veil';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
import { VRM, VRMUtils } from '@pixiv/three-vrm';

// Global variables
let wallet = null;
let client = null;
let selectedNft = null;
let protectedMetadata = null;
let vrmScene = null;
let vrmModel = null;

// DOM Elements
const connectBtn = document.getElementById('connect-wallet');
const disconnectBtn = document.getElementById('disconnect-wallet');
const walletInfo = document.getElementById('wallet-info');
const walletAddress = document.getElementById('wallet-address');
const nftList = document.getElementById('nft-list');
const applyPrivacyBtn = document.getElementById('apply-privacy');
const downloadMetadataBtn = document.getElementById('download-metadata');
const metadataPreview = document.getElementById('metadata-preview');
const originalJson = document.getElementById('original-json');
const protectedJson = document.getElementById('protected-json');
const tabButtons = document.querySelectorAll('.tab-btn');
const tabPanes = document.querySelectorAll('.tab-pane');
const statusModal = document.getElementById('status-modal');
const modalTitle = document.getElementById('modal-title');
const modalMessage = document.getElementById('modal-message');
const closeModalBtn = document.querySelector('.close-modal');
const privacyLevel = document.getElementById('privacy-level');
const demoPrivacyLevel = document.getElementById('demo-privacy-level');
const demoViewerType = document.getElementById('demo-viewer-type');
const protectedAttributes = document.querySelectorAll('#protected-attributes input');

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
  initTabs();
  initWalletButtons();
  initPrivacyControls();
  initVrmDemo();
});

// Initialize tab functionality
function initTabs() {
  tabButtons.forEach(button => {
    button.addEventListener('click', () => {
      tabButtons.forEach(btn => btn.classList.remove('active'));
      tabPanes.forEach(pane => pane.classList.remove('active'));
      
      button.classList.add('active');
      const tabId = button.dataset.tab;
      document.getElementById(tabId).classList.add('active');
      
      // Init VRM if switching to VRM tab
      if (tabId === 'vrm-demo' && !vrmScene) {
        setupVrmScene();
      }
    });
  });
}

// Initialize wallet connection buttons
function initWalletButtons() {
  connectBtn.addEventListener('click', connectWallet);
  disconnectBtn.addEventListener('click', disconnectWallet);
}

// Initialize privacy control events
function initPrivacyControls() {
  privacyLevel.addEventListener('change', updateProtectedAttributes);
  applyPrivacyBtn.addEventListener('click', applyPrivacyProtection);
  downloadMetadataBtn.addEventListener('click', downloadProtectedMetadata);
}

// Update protected attributes based on privacy level
function updateProtectedAttributes() {
  const level = parseInt(privacyLevel.value);
  
  // Reset all checkboxes
  protectedAttributes.forEach(checkbox => {
    checkbox.checked = false;
  });
  
  // Set checkboxes based on privacy level
  if (level >= 1) { // Light
    document.getElementById('attr-agent-name').checked = true;
    document.getElementById('attr-secret-code').checked = true;
  }
  
  if (level >= 2) { // Medium
    document.getElementById('attr-mission').checked = true;
    document.getElementById('attr-origin').checked = true;
  }
  
  if (level >= 3) { // Heavy or Complete
    document.getElementById('attr-accessory').checked = true;
    document.getElementById('attr-symbols').checked = true;
  }
}

// Connect to wallet (mock implementation for demo)
async function connectWallet() {
  showModal('Connecting Wallet', 'Please approve the connection request in your wallet...');
  
  // Simulate wallet connection delay
  await new Promise(resolve => setTimeout(resolve, 1500));
  
  // Create demo wallet
  wallet = Keypair.generate();
  
  // Update UI
  connectBtn.classList.add('hidden');
  walletInfo.classList.remove('hidden');
  walletAddress.textContent = formatAddress(wallet.publicKey.toString());
  
  // Create privacy client
  client = new GlitchGangPrivacyClient(
    'https://api.devnet.solana.com',
    wallet,
    'GlchWrapperProgram111111111111111111111111111'
  );
  
  // Load demo NFTs
  loadDemoNfts();
  
  // Enable buttons
  applyPrivacyBtn.disabled = false;
  
  hideModal();
}

// Disconnect wallet
function disconnectWallet() {
  wallet = null;
  client = null;
  selectedNft = null;
  
  // Update UI
  connectBtn.classList.remove('hidden');
  walletInfo.classList.add('hidden');
  nftList.innerHTML = '<div class="loading-indicator">Connect wallet to view your NFTs</div>';
  
  // Disable buttons
  applyPrivacyBtn.disabled = true;
  downloadMetadataBtn.disabled = true;
  
  // Hide metadata preview
  metadataPreview.classList.add('hidden');
}

// Load demo NFTs
async function loadDemoNfts() {
  nftList.innerHTML = '<div class="loading-indicator">Loading your NFTs...</div>';
  
  // Simulate loading delay
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  // Demo NFT data
  const demoNfts = [
    {
      mint: '3jKpTiKAAtnJMLcQsNk82ua7crubQ86e8KfTQB9fKDwp',
      name: 'Glitch Gang #699',
      image: './public/images/nft1.png',
      metadata: {
        name: "Glitch Gang #699 - VertexStream Navigator",
        symbol: "GG",
        description: "A mysterious entity from the Glitch Gang collective, wielding the power of digital chaos.",
        attributes: [
          { trait_type: "Background", value: "Cyber Haze" },
          { trait_type: "Hood", value: "Starlight Shroud" },
          { trait_type: "Mask", value: "Wraithveil Mask" },
          { trait_type: "Eyes", value: "Neon Void Gaze" },
          { trait_type: "Top", value: "Ecliptic Jacket" },
          { trait_type: "Accessory", value: "Static Emitter" },
          { trait_type: "Symbols", value: "Cyber Symbols" },
          { trait_type: "Origin", value: "Manifested from pure data corruption" },
          { trait_type: "Mission", value: "To protect the sanctity of digital entropy" },
          { trait_type: "Secret Code", value: "GLITCH-8983-ALPHA" },
          { trait_type: "Agent Name", value: "VertexStream Navigator" }
        ],
        image: "./public/images/nft1.png",
        properties: {
          files: [{ uri: "./public/images/nft1.png" }]
        }
      }
    },
    {
      mint: '5VLAQFhyYQnQPCGQm8kLZ8pzgkyCgdszJMbJ3GbJ4kLJ',
      name: 'Glitch Gang #42',
      image: './public/images/nft2.png',
      metadata: {
        name: "Glitch Gang #42 - Quantum Phantom",
        symbol: "GG",
        description: "A digital ghost in the machine, manipulating code and reality.",
        attributes: [
          { trait_type: "Background", value: "Digital Void" },
          { trait_type: "Hood", value: "Quantum Shroud" },
          { trait_type: "Mask", value: "Binary Face" },
          { trait_type: "Eyes", value: "Code Scanners" },
          { trait_type: "Top", value: "Glitch Coat" },
          { trait_type: "Accessory", value: "Entropy Cube" },
          { trait_type: "Symbols", value: "Cryptic Runes" },
          { trait_type: "Origin", value: "Escaped from a quantum mainframe" },
          { trait_type: "Mission", value: "Destabilize surveillance networks" },
          { trait_type: "Secret Code", value: "PHANTOM-42-ZERO" },
          { trait_type: "Agent Name", value: "Quantum Phantom" }
        ],
        image: "./public/images/nft2.png",
        properties: {
          files: [{ uri: "./public/images/nft2.png" }]
        }
      }
    }
  ];
  
  // Clear loading indicator
  nftList.innerHTML = '';
  
  // Create NFT items
  demoNfts.forEach(nft => {
    const nftItem = document.createElement('div');
    nftItem.className = 'nft-item';
    nftItem.innerHTML = `
      <img src="${nft.image}" alt="${nft.name}">
      <div class="nft-name">${nft.name}</div>
    `;
    
    // Add click event
    nftItem.addEventListener('click', () => {
      // Deselect all
      document.querySelectorAll('.nft-item').forEach(item => {
        item.classList.remove('selected');
      });
      
      // Select this one
      nftItem.classList.add('selected');
      
      // Set selected NFT
      selectedNft = nft;
      
      // Show metadata
      showMetadata(nft.metadata);
    });
    
    nftList.appendChild(nftItem);
  });
}

// Show NFT metadata
function showMetadata(metadata) {
  // Display original metadata
  originalJson.textContent = JSON.stringify(metadata, null, 2);
  
  // Clear protected metadata
  protectedJson.textContent = '';
  
  // Show preview section
  metadataPreview.classList.remove('hidden');
  
  // Reset protected metadata
  protectedMetadata = null;
  downloadMetadataBtn.disabled = true;
}

// Apply privacy protection
async function applyPrivacyProtection() {
  if (!selectedNft || !client) return;
  
  showModal('Applying Privacy Protection', 'Encrypting sensitive attributes...');
  
  try {
    // Get selected privacy level
    const level = parseInt(privacyLevel.value);
    
    // Get VRM model URL if provided
    const vrmModelUrl = document.getElementById('vrm-model').value;
    
    // Apply privacy protection
    protectedMetadata = await client.protectMetadata(selectedNft.metadata, level);
    
    // Add VRM privacy settings if URL provided
    if (vrmModelUrl) {
      client.addVrmPrivacy(protectedMetadata, vrmModelUrl);
    }
    
    // Display protected metadata
    protectedJson.textContent = JSON.stringify(protectedMetadata, null, 2);
    
    // Enable download button
    downloadMetadataBtn.disabled = false;
    
    hideModal();
  } catch (error) {
    console.error('Error applying privacy protection:', error);
    showModal('Error', 'Failed to apply privacy protection. Please try again.');
    
    setTimeout(hideModal, 3000);
  }
}

// Download protected metadata
function downloadProtectedMetadata() {
  if (!protectedMetadata) return;
  
  // Create blob and download link
  const json = JSON.stringify(protectedMetadata, null, 2);
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  
  const a = document.createElement('a');
  a.href = url;
  a.download = `protected_${selectedNft.mint}.json`;
  a.click();
  
  URL.revokeObjectURL(url);
}

// Initialize VRM demo
function initVrmDemo() {
  // VRM demo controls
  demoPrivacyLevel.addEventListener('change', updateVrmPrivacy);
  demoViewerType.addEventListener('change', updateVrmPrivacy);
  
  document.getElementById('protect-position').addEventListener('change', updateVrmPrivacy);
  document.getElementById('protect-rotation').addEventListener('change', updateVrmPrivacy);
  document.getElementById('protect-gesture').addEventListener('change', updateVrmPrivacy);
  document.getElementById('protect-voice').addEventListener('change', updateVrmPrivacy);
  
  // Camera control buttons
  document.getElementById('reset-camera').addEventListener('click', resetCamera);
  document.getElementById('toggle-animation').addEventListener('click', toggleAnimation);
}

// Set up Three.js scene for VRM
function setupVrmScene() {
  const canvas = document.getElementById('vrm-canvas');
  
  // Create renderer
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setSize(canvas.clientWidth, canvas.clientHeight);
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.outputEncoding = THREE.sRGBEncoding;
  
  // Create scene
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x121214);
  
  // Create camera
  const camera = new THREE.PerspectiveCamera(
    45, canvas.clientWidth / canvas.clientHeight, 0.1, 100
  );
  camera.position.set(0, 1.5, 3);
  
  // Add orbit controls
  const controls = new OrbitControls(camera, renderer.domElement);
  controls.target.set(0, 1, 0);
  controls.update();
  
  // Add lights
  const light = new THREE.DirectionalLight(0xffffff, 1);
  light.position.set(1, 1, 1);
  scene.add(light);
  
  scene.add(new THREE.AmbientLight(0xffffff, 0.5));
  
  // Add grid helper
  const gridHelper = new THREE.GridHelper(10, 10);
  scene.add(gridHelper);
  
  // Store scene
  vrmScene = { scene, camera, renderer, controls, animating: false };
  
  // Animation loop
  function animate() {
    requestAnimationFrame(animate);
    
    if (vrmModel) {
      vrmModel.update(0.016); // Update VRM with ~60fps time delta
    }
    
    renderer.render(scene, camera);
  }
  
  animate();
  
  // Handle window resize
  window.addEventListener('resize', () => {
    if (!canvas) return;
    
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;
    
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    renderer.setSize(width, height);
  });
  
  // Load demo VRM model
  loadVrmModel('./public/models/sample.vrm');
}

// Load VRM model
async function loadVrmModel(modelUrl) {
  if (!vrmScene) return;
  
  showModal('Loading VRM Model', 'Please wait while the model loads...');
  
  try {
    // For demo purposes, use a placeholder URL
    const placeholderUrl = 'https://cdn.glitch.me/3b2e7e42-e7a3-4f80-a902-a1c4b9d79754/AvatarSample_B.vrm?v=1656041929868';
    
    const loader = new GLTFLoader();
    
    loader.load(
      placeholderUrl,
      (gltf) => {
        VRMUtils.removeUnnecessaryJoints(gltf.scene);
        
        VRM.from(gltf).then((vrm) => {
          // Remove any previous model
          if (vrmModel && vrmModel.scene) {
            vrmScene.scene.remove(vrmModel.scene);
          }
          
          // Add new model
          vrm.scene.position.set(0, 0, 0);
          vrmScene.scene.add(vrm.scene);
          vrmModel = vrm;
          
          hideModal();
          
          // Apply initial privacy
          updateVrmPrivacy();
        });
      },
      (progress) => {
        const percentage = Math.floor((progress.loaded / progress.total) * 100);
        modalMessage.textContent = `Loading model: ${percentage}%`;
      },
      (error) => {
        console.error('Error loading VRM:', error);
        showModal('Error', 'Failed to load VRM model. Please try again.');
        setTimeout(hideModal, 3000);
      }
    );
  } catch (error) {
    console.error('Error loading VRM model:', error);
    showModal('Error', 'Failed to load VRM model. Please try again.');
    setTimeout(hideModal, 3000);
  }
}

// Update VRM privacy based on controls
function updateVrmPrivacy() {
  if (!vrmModel) return;
  
  // Get privacy settings
  const privacyLevel = parseInt(demoPrivacyLevel.value);
  const viewerType = demoViewerType.value;
  const protectPosition = document.getElementById('protect-position').checked;
  const protectRotation = document.getElementById('protect-rotation').checked;
  const protectGesture = document.getElementById('protect-gesture').checked;
  const protectVoice = document.getElementById('protect-voice').checked;
  
  // Create sample VRM data
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
  
  // Apply privacy effects based on level and viewer type
  let positionNoise = 0;
  let rotationNoise = 0;
  let gestureNoise = 0;
  let voiceDistortion = 0;
  
  // Determine if viewer is trusted
  const isTrusted = viewerType === 'owner' || viewerType === 'trusted';
  
  if (!isTrusted) {
    // Apply noise based on privacy level
    switch (privacyLevel) {
      case 1: // Light
        positionNoise = 0.2;
        rotationNoise = 0.1;
        gestureNoise = 0.2;
        voiceDistortion = 20;
        break;
      case 2: // Medium
        positionNoise = 0.5;
        rotationNoise = 0.3;
        gestureNoise = 0.5;
        voiceDistortion = 50;
        break;
      case 3: // Heavy
        positionNoise = 1.0;
        rotationNoise = 0.6;
        gestureNoise = 0.8;
        voiceDistortion = 80;
        break;
      case 4: // Complete
        positionNoise = 2.0;
        rotationNoise = 1.0;
        gestureNoise = 1.0;
        voiceDistortion = 100;
        break;
    }
    
    // Apply selected protections
    if (!protectPosition) positionNoise = 0;
    if (!protectRotation) rotationNoise = 0;
    if (!protectGesture) gestureNoise = 0;
    if (!protectVoice) voiceDistortion = 0;
    
    // Apply to model
    if (protectPosition) {
      // Add random position offset
      vrmModel.scene.position.x = (Math.random() - 0.5) * positionNoise;
      vrmModel.scene.position.y = (Math.random() - 0.5) * positionNoise;
      vrmModel.scene.position.z = (Math.random() - 0.5) * positionNoise;
    } else {
      vrmModel.scene.position.set(0, 0, 0);
    }
    
    if (protectRotation && vrmModel.humanoid) {
      // Add random rotation to head
      const head = vrmModel.humanoid.getBoneNode(THREE.VRMSchema.HumanoidBoneName.Head);
      if (head) {
        head.rotation.x = (Math.random() - 0.5) * rotationNoise;
        head.rotation.y = (Math.random() - 0.5) * rotationNoise;
        head.rotation.z = (Math.random() - 0.5) * rotationNoise * 0.5;
      }
    }
    
    if (protectGesture && vrmModel.blendShapeProxy) {
      // Add random expressions
      const blendProxy = vrmModel.blendShapeProxy;
      blendProxy.setValue('blink', Math.random() * gestureNoise);
      blendProxy.setValue('a', Math.random() * gestureNoise * 0.5);
      blendProxy.setValue('angry', Math.random() * gestureNoise * 0.3);
    }
  } else {
    // Reset model for trusted viewers
    vrmModel.scene.position.set(0, 0, 0);
    
    if (vrmModel.humanoid) {
      const head = vrmModel.humanoid.getBoneNode(THREE.VRMSchema.HumanoidBoneName.Head);
      if (head) {
        head.rotation.set(0, 0, 0);
      }
    }
    
    if (vrmModel.blendShapeProxy) {
      const blendProxy = vrmModel.blendShapeProxy;
      blendProxy.setValue('blink', 0);
      blendProxy.setValue('a', 0);
      blendProxy.setValue('angry', 0);
    }
  }
  
  // Update stats display
  document.getElementById('position-stats').querySelector('.stat-value').textContent = 
    protectPosition && !isTrusted 
      ? `(${vrmModel.scene.position.x.toFixed(2)}, ${vrmModel.scene.position.y.toFixed(2)}, ${vrmModel.scene.position.z.toFixed(2)})` 
      : '(0.00, 0.00, 0.00)';
  
  document.getElementById('rotation-stats').querySelector('.stat-value').textContent = 
    protectRotation && !isTrusted ? 'Randomized' : 'Original';
  
  document.getElementById('gesture-stats').querySelector('.stat-value').textContent = 
    protectGesture && !isTrusted ? `${(gestureNoise * 100).toFixed(0)}%` : '0%';
  
  document.getElementById('voice-stats').querySelector('.stat-value').textContent = 
    protectVoice && !isTrusted ? `${voiceDistortion.toFixed(0)}%` : '0%';
}

// Reset camera to default position
function resetCamera() {
  if (!vrmScene) return;
  
  vrmScene.camera.position.set(0, 1.5, 3);
  vrmScene.controls.target.set(0, 1, 0);
  vrmScene.controls.update();
}

// Toggle animation
function toggleAnimation() {
  if (!vrmScene || !vrmModel) return;
  
  vrmScene.animating = !vrmScene.animating;
  
  if (vrmScene.animating) {
    // Start simple animation
    const animate = () => {
      if (!vrmScene.animating) return;
      
      // Rotate slightly
      vrmModel.scene.rotation.y += 0.01;
      
      requestAnimationFrame(animate);
    };
    
    animate();
    
    document.getElementById('toggle-animation').textContent = 'Stop Animation';
  } else {
    document.getElementById('toggle-animation').textContent = 'Toggle Animation';
  }
}

// Show modal
function showModal(title, message) {
  modalTitle.textContent = title;
  modalMessage.textContent = message;
  statusModal.classList.remove('hidden');
}

// Hide modal
function hideModal() {
  statusModal.classList.add('hidden');
}

// Close modal button
closeModalBtn.addEventListener('click', hideModal);

// Format address for display
function formatAddress(address) {
  return `${address.substring(0, 4)}...${address.substring(address.length - 4)}`;
}
