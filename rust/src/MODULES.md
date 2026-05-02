# PeTTa Module Structure

This document describes the reorganized module structure for the PeTTa runtime.

## Directory Layout

```
rust/src/
├── api/                    # Public API layer (ergonomic interface)
│   ├── mod.rs             # Module exports
│   ├── engine.rs          # PeTTa, PeTTaEngine implementations
│   ├── config.rs          # EngineConfig, Backend types
│   └── result.rs          # ExecutionResult, ExecutionStats
│
├── core/                   # Core types and traits
│   ├── mod.rs             # Core exports
│   ├── backend.rs         # Backend trait, capabilities, stats
│   ├── errors.rs          # Unified error types
│   ├── values.rs          # MettaValue, MettaResult types
│   └── types.rs           # Type definitions
│
├── backends/               # Backend implementations
│   ├── mod.rs             # Backend registry & selection
│   ├── swipl/             # SWI-Prolog backend
│   │   └── mod.rs         # SwiplBackend implementation
│   └── mork/              # MORK backend (feature-gated)
│       └── mod.rs         # MorkBackend implementation
│
├── engine/                 # Core engine implementation
│   ├── mod.rs             # PeTTaEngine core
│   ├── backend.rs         # Backend trait (legacy, being phased out)
│   ├── backends.rs        # Backend implementations (legacy)
│   ├── config.rs          # Configuration types
│   ├── errors.rs          # Error types (legacy)
│   └── ...
│
├── pathmap/                # PathMap (reorganized)
│   ├── trie/              # Core trie implementation
│   │   ├── mod.rs         # Trie exports
│   │   └── ops.rs         # Trie operations
│   ├── zipper/            # Zipper implementations
│   │   ├── mod.rs         # Zipper trait & exports
│   │   ├── base.rs        # Base zipper
│   │   ├── overlay.rs     # Overlay zipper
│   │   ├── product.rs     # Product zipper
│   │   └── write.rs       # Write zipper
│   ├── ring/              # Ring operations
│   │   ├── mod.rs         # Ring exports
│   │   └── ops.rs         # Ring operations
│   └── arena/             # Arena allocation
│       ├── mod.rs         # Arena exports
│       └── allocator.rs   # Bump allocator
│
├── parser/                 # Parsers
├── utils/                  # Utilities
├── repl/                   # REPL implementation
├── cli.rs                  # CLI handling
└── main.rs                 # CLI entry point
```

## Module Responsibilities

### `api/` - Public API Layer
- **Purpose**: Ergonomic, user-facing interface
- **Key Types**: `PeTTa`, `PeTTaEngine`, `Builder`
- **Usage**: Import from `petta::api::*`

### `core/` - Core Abstractions
- **Purpose**: Fundamental types and traits
- **Key Types**: `Backend` trait, `Error`, `MettaValue`
- **Usage**: Internal and public use

### `backends/` - Backend Implementations
- **Purpose**: Concrete backend implementations
- **Key Types**: `SwiplBackend`, `MorkBackend`, `BackendRegistry`
- **Usage**: Backend selection and instantiation

### `engine/` - Core Engine
- **Purpose**: Main execution engine logic
- **Key Types**: `PeTTaEngine` (internal implementation)
- **Usage**: Internal use (wrapped by `api/`)

### `pathmap/` - PathMap Data Structures
- **Purpose**: Path-based trie and zipper operations
- **Key Types**: Various zipper implementations
- **Usage**: Internal path management

## Migration Guide

### Old Structure → New Structure

| Old Import | New Import |
|------------|------------|
| `petta::PeTTaEngine` | `petta::api::PeTTaEngine` |
| `petta::EngineConfig` | `petta::api::EngineConfig` |
| `petta::Backend` | `petta::api::Backend` |
| `petta::Error` | `petta::core::Error` |

### Example Usage

```rust
use petta::api::{PeTTa, Backend, EngineConfig};

// Simple usage
let mut engine = PeTTa::new()?;
let result = engine.eval("!(+ 1 2)")?;

// With configuration
let mut engine = PeTTa::builder()
    .backend(Backend::Mork)
    .verbose(true)
    .build()?;
```

## Design Principles

1. **Single Source of Truth**: Each concept defined once
2. **Zero-Cost Abstractions**: Type safety at compile time
3. **Fail Fast**: Compile-time errors preferred
4. **Progressive Disclosure**: Simple API, depth for advanced use
5. **Performance by Default**: No runtime cost for unused features

## Status

- ✅ Phase 1: Module Reorganization - Complete
- ✅ Phase 2: Backend Unification - Complete
- ✅ Phase 3: Error Consolidation - Complete
- 🔄 Phase 4: PathMap Optimization - In Progress
- ✅ Phase 5: API Ergonomics - Complete
- ⏳ Phase 6: Type System Improvements - Pending
- ⏳ Phase 7: Performance Tuning - Pending
