project-89/
│
├── Cargo.toml                     # Root project configuration (Rust workspace)
├── README.md                      # Project overview and setup instructions
├── LICENSE                        # License file (e.g., MIT or Apache-2.0)
│
├── solana/                        # Solana program components
│   ├── privacy_wrapper/           # Privacy wrapper Solana program
│   │   ├── Cargo.toml             # Program-specific dependencies
│   │   └── src/
│   │       ├── lib.rs             # Program entrypoint (from file#1: declares modules)
│   │       ├── instruction.rs     # Instruction definitions (from file#1: WrapperInstruction enum)
│   │       ├── processor.rs       # Instruction processors (from file#1: process_instruction and handlers)
│   │       ├── state.rs           # Program state (from file#1: PrivacyWrapper struct)
│   │       └── error.rs           # Custom error types (placeholder for future errors)
│   │
│   └── tests/                     # Solana program tests
│       └── integration_tests.rs   # Integration test suite (to be implemented)
│
├── core/                          # Core Rust library components
│   ├── Cargo.toml                 # Core library dependencies
│   └── src/
│       ├── lib.rs                 # Library entrypoint (exports sub-modules)
│       ├── quantum_veil/          # Quantum encryption system (from file#2)
│       │   ├── mod.rs             # Module declarations
│       │   ├── encryption.rs      # Encryption utilities (encrypt_data, decrypt_data from file#2)
│       │   ├── key_gen.rs         # Key generation (encryption_key from file#2)
│       │   └── config.rs          # Privacy configuration (PrivacyConfig from file#2)
│       │
│       ├── synchronicity_mask/    # VRM privacy masking (from file#2)
│       │   ├── mod.rs             # Module declarations
│       │   ├── vrm_data.rs        # VRM data structures (VrmData from file#2)
│       │   ├── privacy_levels.rs  # Privacy level definitions (PrivacyLevel from file#2)
│       │   └── masking.rs         # Data masking algorithms (process_vrm_data from file#2)
│       │
│       └── timeline_shifter/      # Metadata fragmentation (from file#2)
│           ├── mod.rs             # Module declarations
│           ├── fragment.rs        # Fragment definitions (MetadataFragment from file#2)
│           ├── storage.rs         # Storage logic (fracture_metadata from file#2)
│           └── timeline.rs        # Timeline definitions (TimelineType from file#2)
│
├── client/                        # Client implementations
│   ├── rust/                      # Rust client (from file#2)
│   │   ├── Cargo.toml             # Rust client dependencies
│   │   └── src/
│   │       ├── lib.rs             # Client entrypoint
│   │       ├── client.rs          # Main client impl (GlitchGangPrivacyClient from file#2)
│   │       ├── models.rs          # Data models (GlitchGangMetadata, PrivateData from file#2)
│   │       └── utils.rs           # Helper utilities (fetch_metadata, etc. from file#2)
│   │
│   └── typescript/                # TypeScript client (from file#3)
│       ├── package.json           # npm dependencies (solana/web3.js, three-vrm, etc.)
│       ├── tsconfig.json          # TypeScript configuration
│       └── src/
│           ├── index.ts           # Main exports (exports GlitchGangPrivacyClient)
│           ├── client.ts          # Client impl (GlitchGangPrivacyClient from file#3)
│           ├── models/            # Type definitions
│           │   ├── index.ts       # Type exports
│           │   ├── metadata.ts    # Metadata types (GlitchGangMetadata from file#3)
│           │   └── vrm.ts         # VRM types (VrmData from file#3)
│           ├── crypto/            # Cryptography wrappers
│           │   ├── index.ts       # Crypto exports
│           │   └── encryption.ts  # Encryption utils (encrypt, decrypt referenced in file#3)
│           └── utils/             # Helper utilities
│               ├── index.ts       # Utility exports
│               └── solana.ts      # Solana-specific helpers (connection handling)
│
├── examples/                      # Usage examples
│   ├── rust/                      # Rust examples (from file#2 main function)
│   │   ├── Cargo.toml             # Example dependencies
│   │   └── src/
│   │       └── main.rs            # CLI example (main function from file#2)
│   │
│   └── web/                       # Web examples (from file#4)
│       ├── package.json           # Web app dependencies (react, three, etc.)
│       ├── index.html             # Main HTML file
│       ├── public/                # Static assets
│       └── src/
│           ├── App.tsx            # Main React app (integrates PrivacyPanel)
│           ├── components/        # UI components
│           │   ├── PrivacyPanel.tsx  # Privacy controls UI (file#4)
│           │   └── VrmViewer.tsx  # 3D VRM viewer (placeholder for future impl)
│           └── utils/             # Frontend utilities
│               └── index.ts       # Utility exports
│
├── tools/                         # Utility tools
│   ├── keygen/                    # Key generation tool
│   │   ├── Cargo.toml             # Tool dependencies
│   │   └── src/
│   │       └── main.rs            # Key generator (placeholder for keypair generation)
│   └── metadata-viewer/           # Metadata explorer
│       ├── Cargo.toml             # Tool dependencies
│       └── src/
│           └── main.rs            # Metadata visualization (placeholder)
│
└── docs/                          # Documentation
    ├── architecture.md            # System architecture overview
    ├── api/                       # API documentation
    │   ├── rust.md                # Rust API docs (client.rs, core modules)
    │   └── typescript.md          # TypeScript API docs (client.ts)
    ├── diagrams/                  # Architectural diagrams
    │   ├── system-overview.svg    # High-level system overview
    │   └── data-flow.svg          # Data flow diagram
    └── guides/                    # Usage guides
        ├── getting-started.md     # Quickstart guide
        ├── privacy-controls.md    # Privacy wrapper and levels guide
        └── integration.md         # Integration with Daydreams framework
