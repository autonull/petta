# OpenCog TrueAGI Hyperon MeTTa PeTTa System - Comprehensive Refactoring Plan

**Version:** 1.0  
**Date:** 2026-05-01  
**Status:** Approved for Implementation  
**Total Codebase:** ~130K lines Rust code (85+ files)

---

## Executive Summary

This document outlines a complete architectural refactoring of the PeTTa system to achieve:
- **Elegant Architecture**: Clear separation of concerns, unified abstractions
- **Maximum Performance**: Optimized hot paths, zero-cost abstractions, reduced allocations
- **Developer Ergonomics**: Intuitive APIs, excellent error messages, minimal boilerplate
- **Code Integrity**: Type-safe interfaces, compile-time guarantees, comprehensive testing
- **MORK & PathMap Integration**: Full support for both backends with unified interface

### Key Metrics
- **Current State**: 129,680 lines across 85+ Rust files
- **Target Reduction**: 25-35% through consolidation (eliminate ~35K lines)
- **Performance Goal**: 2-3x improvement in hot paths
- **Test Coverage**: >95% for critical paths

---

## Current Architecture Analysis

### Code Distribution
```\nTotal: 129,680 lines\n├── PathMap:        28,772 lines (22%)\n├── MORK:           12,256 lines (9%)\n├── Engine:          ~5,000 lines (4%)\n├── Parser:          ~2,000 lines (2%)\n├── Utilities:       ~3,000 lines (2%)\n├── Prolog Backend: ~1,500 lines (1%)\n└── Tests/Benches:  ~8,000 lines (6%)\n    Remaining:      ~68K lines (53% - core logic, duplication)\n```\n\n### Critical Issues Identified

#### 1. **Massive Code Duplication** (HIGH IMPACT)
- Backend functionality duplicated across SwiplBackend and MorkBackend: ~300 lines
- PathMap zipper implementations duplicated: ~50K+ lines across multiple files
- Error handling scattered: 15+ error types across modules
- Config builders repeated in 3+ places

#### 2. **Poor Module Organization** (MEDIUM IMPACT)
- 15+ top-level modules in `rust/src/` with unclear responsibility boundaries
- Mix of library code (`lib.rs`) and application code (`main.rs`) in same crate
- Internal implementation details exposed alongside public API
- PathMap modules flat (30+ files in single directory)

#### 3. **Weak Type System Usage** (MEDIUM IMPACT)
- Stringly-typed configurations where enums would provide safety
- No compile-time state validation (engine can execute before initialization)
- Path types not differentiated (PathBuf everywhere)
- Missing type-level guarantees for backend capabilities

#### 4. **Performance Bottlenecks** (HIGH IMPACT)
- Excessive allocations in hot paths (String, Vec, Box)
- No small-vector optimization for small collections
- Backend trait objects cause cache misses
- PathMap operations not optimized for common cases

#### 5. **Error Handling Chaos** (HIGH IMPACT)
- 15+ different error types across modules
- Inconsistent error messages (some helpful, some cryptic)
- No source locations in parse errors
- No actionable suggestions for recovery

#### 6. **MORK Integration Issues** (MEDIUM IMPACT)
- MORK code gated behind feature flag, not integrated into main flow
- Separate module structure from Prolog backend
- Different error handling than main engine
- No unified capability detection

---

## Refactoring Strategy

### Guiding Principles

1. **Single Source of Truth**: Each concept defined once
2. **Zero-Cost Abstractions**: Type system work happens at compile time
3. **Fail Fast**: Compile-time errors preferred over runtime panics
4. **Progressive Disclosure**: Simple API for common cases, depth for advanced
5. **Performance by Default**: No runtime cost for unused features

### Phase Prioritization Matrix

| Phase | Impact | Effort | Risk | Priority |
|-------|--------|--------|------|----------|
| 1. Module Reorganization | High | Medium | Low | **1st** |
| 2. Backend Unification | **Critical** | High | Medium | **1st** |
| 3. Error Consolidation | High | Low | Low | **2nd** |
| 4. PathMap Optimization | **Critical** | High | Medium | **2nd** |
| 5. API Ergonomics | High | Medium | Low | **3rd** |
| 6. Type System Improvements | Medium | Medium | Medium | **3rd** |
| 7. Performance Tuning | High | High | Low | **4th** |

---

## Detailed Refactoring Plan

### Phase 1: Module Reorganization (Week 1-2)

**Goal**: Create clear architectural boundaries and logical grouping.

#### 1.1 New Directory Structure

```
rust/
├── src/
│   ├── lib.rs                    # Public API exports only
│   ├── bin/
│   │   └── petta.rs             # CLI entry point (was main.rs)
│   │
│   ├── api/                      # PUBLIC API (ergonomic layer)
│   │   ├── mod.rs               # PeTTa, PeTTaBuilder exports
│   │   ├── engine.rs            # PeTTaEngine public interface
│   │   ├── config.rs            # EngineConfig, Backend types
│   │   └── result.rs            # ExecutionResult, MettaResult
│   │
│   ├── core/                     # CORE FUNCTIONALITY
│   │   ├── mod.rs               # Core types and traits
│   │   ├── backend.rs           # Backend trait & capabilities
│   │   ├── errors.rs            # Unified error types
│   │   ├── values.rs            # MettaValue, MettaResult types
│   │   └── types.rs             # Type definitions
│   │
│   ├── backends/                 # BACKEND IMPLEMENTATIONS
│   │   ├── mod.rs               # Backend registry & selection
│   │   ├── swipl/               # SWI-Prolog backend
│   │   │   ├── mod.rs           # Module exports
│   │   │   ├── client.rs        # SWI-Prolog client
│   │   │   ├── protocol.rs      # Binary protocol
│   │   │   └── translator.rs    # MeTTa → Prolog translation
│   │   └── mork/                # MORK backend
│   │       ├── mod.rs           # Module exports
│   │       ├── engine.rs        # MORK engine wrapper
│   │       └── interpreter.rs   # Zipper interpreter
│   │
│   ├── parser/                   # PARSERS
│   │   ├── mod.rs               # Parser API
│   │   ├── sexpr.rs             # S-expression parser (nom)
│   │   └── metta.rs             # MeTTa language parser
│   │
│   ├── pathmap/                  # PATHMAP (reorganized)
│   │   ├── mod.rs               # PathMap exports
│   │   ├── trie/                # Core trie implementation
│   │   │   ├── mod.rs
│   │   │   ├── node.rs          # Trie node types
│   │   │   └── ops.rs           # Core operations
│   │   ├── zipper/              # Zipper implementations
│   │   │   ├── mod.rs
│   │   │   ├── base.rs          # Base zipper
│   │   │   ├── overlay.rs       # Overlay zipper
│   │   │   ├── product.rs       # Product zipper
│   │   │   └── write.rs         # Write zipper
│   │   ├── ring/                # Ring operations
│   │   ├── morphisms/           # Morphism operations
│   │   ├── utils/               # Utilities
│   │   └── arena/               # Arena allocation (optional)
│   │
│   ├── utils/                    # UTILITIES
│   │   ├── mod.rs
│   │   ├── formatter.rs         # Output formatting
│   │   ├── profiler.rs          # Profiling support
│   │   └── hasher.rs            # Hash utilities (GXHash)
│   │
│   └── internal/                 # INTERNAL (not public API)
│       ├── repl/                # REPL implementation
│       ├── cli/                 # CLI handling
│       ├── observability/       # Metrics & monitoring
│       └── differential/        # Differential testing
│
├── tests/                        # Integration tests
│   ├── engine_tests.rs
│   ├── backend_tests.rs
│   └── parser_tests.rs
│
└── benches/                      # Benchmarks
    ├── backend_bench.rs
    └── pathmap_bench.rs
```

#### 1.2 Migration Steps

1. Create new directory structure alongside old
2. Move files with `git mv` to preserve history
3. Update all imports (use `cargo check` iteratively)
4. Remove old structure once all imports resolve
5. Update documentation and examples

#### 1.3 Success Criteria
- [ ] All code compiles with new structure
- [ ] All tests pass
- [ ] No circular dependencies
- [ ] Clear separation: `api/` (public) vs `internal/` (private)

---

### Phase 2: Backend Unification (Week 2-4) ⭐ CRITICAL

**Goal**: Eliminate backend duplication with unified trait-based interface.

#### 2.1 Current Problem

```rust
// Current: BackendState enum duplicates all methods
enum BackendState {
    Swipl(SwiplBackend),
    Mork(MorkBackend),
}

// Duplicated in both backends:
// - load_metta_file()
// - load_metta_files()
// - process_metta_string()
// - is_alive()
// - restart()
// - stderr_output()
// - shutdown()
// Total duplication: ~300 lines
```

#### 2.2 Unified Backend Trait

```rust
// core/backend.rs
use std::path::{Path, PathBuf};
use crate::values::MettaResult;
use crate::errors::BackendError;

/// Unified backend trait for all execution engines
pub trait Backend: Send + Sync {
    /// Backend name (e.g., "SWI-Prolog", "MORK")
    fn name(&self) -> &'static str;
    
    /// Backend version string
    fn version(&self) -> &'static str;
    
    /// Check if backend is alive and responsive
    fn is_alive(&self) -> bool;
    
    /// Get backend capabilities
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::default()
    }
    
    /// Load and execute a single MeTTa file
    fn load_file(&mut self, path: &Path) -> Result<Vec<MettaResult>, BackendError>;
    
    /// Load and execute multiple MeTTa files
    fn load_files(&mut self, paths: &[&Path]) -> Result<Vec<MettaResult>, BackendError>;
    
    /// Execute MeTTa code string
    fn execute(&mut self, code: &str) -> Result<Vec<MettaResult>, BackendError>;
    
    /// Restart backend (for crash recovery)
    fn restart(&mut self) -> Result<(), BackendError>;
    
    /// Shutdown backend gracefully
    fn shutdown(&mut self) -> Result<(), BackendError>;
}

/// Backend capabilities for feature detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendCapabilities {
    pub supports_parallel: bool,
    pub supports_streaming: bool,
    pub supports_incremental: bool,
    pub supports_persistence: bool,
    pub supports_transactions: bool,
}
```

#### 2.3 Backend Implementations

```rust
// backends/swipl/mod.rs
pub struct SwiplBackend {
    client: PrologClient,
    state: BackendState,
}

impl Backend for SwiplBackend {
    fn name(&self) -> &'static str { "SWI-Prolog" }
    
    fn version(&self) -> &'static str { "9.3+" }
    
    fn is_alive(&self) -> bool {
        self.client.ping()
    }
    
    fn load_file(&mut self, path: &Path) -> Result<Vec<MettaResult>, BackendError> {
        self.client.consult(path)?;
        Ok(vec![])
    }
    
    fn execute(&mut self, code: &str) -> Result<Vec<MettaResult>, BackendError> {
        self.client.query(code)
    }
    
    // ... other methods
}

// backends/mork/mod.rs
pub struct MorkBackend {
    engine: MorkEngine,
    space: AtomSpace,
}

impl Backend for MorkBackend {
    fn name(&self) -> &'static str { "MORK" }
    
    fn version(&self) -> &'static str { "1.0" }
    
    fn is_alive(&self) -> bool { true } // Always alive (in-process)
    
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            supports_parallel: true,
            supports_streaming: true,
            ..Default::default()
        }
    }
    
    fn load_file(&mut self, path: &Path) -> Result<Vec<MettaResult>, BackendError> {
        let code = std::fs::read_to_string(path)?;
        self.execute(&code)
    }
    
    fn execute(&mut self, code: &str) -> Result<Vec<MettaResult>, BackendError> {
        let expr = self.parser.parse(code)?;
        self.interpreter.execute(expr)
    }
    
    // ... other methods
}
```

#### 2.4 Backend Registry

```rust
// backends/mod.rs
use std::collections::HashMap;
use box::Box;

pub struct BackendRegistry {
    backends: HashMap<String, Box<dyn Backend>>,
}

impl BackendRegistry {
    /// Auto-select backend based on features and availability
    pub fn auto_select() -> Result<Box<dyn Backend>, BackendError> {
        #[cfg(feature = "mork")]
        {
            // Prefer MORK if available
            return Ok(Box::new(MorkBackend::new()));
        }
        
        #[cfg(feature = "swipl")]
        {
            return Ok(Box::new(SwiplBackend::new()?));
        }
        
        Err(BackendError::NoBackendAvailable)
    }
    
    /// Create specific backend
    pub fn create(backend_type: BackendType) -> Result<Box<dyn Backend>, BackendError> {
        match backend_type {
            BackendType::Swipl => Ok(Box::new(SwiplBackend::new()?)),
            #[cfg(feature = "mork")]
            BackendType::Mork => Ok(Box::new(MorkBackend::new())),
            _ => Err(BackendError::UnknownBackend),
        }
    }
}
```

#### 2.5 Engine Simplification

```rust
// api/engine.rs
pub struct PeTTaEngine {
    backend: Box<dyn Backend>,
    config: EngineConfig,
    state: EngineState,
    restarts: u32,
}

impl PeTTaEngine {
    pub fn new(config: &EngineConfig) -> Result<Self, Error> {
        let backend = BackendRegistry::create(config.backend)?;
        
        Ok(Self {
            backend,
            config: config.clone(),
            state: EngineState::Initialized,
            restarts: 0,
        })
    }
    
    pub fn execute(&mut self, code: &str) -> Result<Vec<MettaResult>, Error> {
        self.retry_on_crash(|backend| backend.execute(code))
    }
    
    fn retry_on_crash<F, T>(&mut self, mut f: F) -> Result<T, Error>
    where
        F: FnMut(&mut Box<dyn Backend>) -> Result<T, BackendError>,
    {
        let mut attempts = 0;
        loop {
            match f(&mut self.backend) {
                Err(BackendError::ChildClosed) if attempts < self.config.max_restarts => {
                    attempts += 1;
                    self.backend.restart()?;
                    self.restarts += 1;
                }
                other => return other.map_err(Error::from),
            }
        }
    }
}
```

#### 2.6 Success Criteria
- [ ] Backend trait fully abstracts SwiplBackend and MorkBackend
- [ ] BackendState enum eliminated
- [ ] 300+ lines of duplication removed
- [ ] All tests pass with both backends
- [ ] Backend switching seamless

---

### Phase 3: Error Handling Consolidation (Week 3-4)

**Goal**: Unified error types with actionable messages and source locations.

#### 3.1 Unified Error Type

```rust
// core/errors.rs
use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum Error {
    // Backend errors
    #[error("Backend error: {0}")]
    Backend(#[from] BackendError),
    
    // Parse errors with location
    #[error("Parse error at {location}: {message}")]
    Parse {
        message: String,
        location: SourceLocation,
    },
    
    // Type errors with context
    #[error("Type error: expected {expected}, found {found} in {context}")]
    Type {
        expected: String,
        found: String,
        context: String,
    },
    
    // Execution errors
    #[error("Execution failed: {0}")]
    Execution(String),
    
    // Configuration errors
    #[error("Invalid configuration: {0}")]
    Config(String),
    
    // I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Source location for error reporting
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", 
            self.file.display(), 
            self.line, 
            self.column
        )
    }
}
```

#### 3.2 Backend Error Type

```rust
// core/backend.rs
#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Backend not available: {0}")]
    NotAvailable(String),
    
    #[error("Backend crashed: {0}")]
    Crash(String),
    
    #[error("Backend child process closed")]
    ChildClosed,
    
    #[error("Unknown backend")]
    UnknownBackend,
    
    #[error("No backend available")]
    NoBackendAvailable,
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Type error: {0}")]
    Type(String),
}
```

#### 3.3 Error Context Enhancement

```rust
// core/errors.rs
pub trait ErrorContext<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;
    fn suggestion(self, suggestion: impl Into<String>) -> Self;
}

impl<T, E: std::fmt::Display> ErrorContext<T> for std::result::Result<T, E> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::Execution(format!("{}: {}", msg.into(), e)))
    }
    
    fn suggestion(self, _suggestion: impl Into<String>) -> Self {
        // Could store suggestion in error type
        self
    }
}

// Usage example
fn parse_metta(code: &str) -> Result<Expr> {
    parse_expression(code)
        .context("Failed to parse expression")
        .suggestion("Check syntax and parentheses matching")
}
```

#### 3.4 Success Criteria
- [ ] All error types consolidated into unified enum
- [ ] Parse errors include source locations
- [ ] Error messages are actionable
- [ ] No panic! or unwrap() in production code

---

### Phase 4: PathMap Optimization (Week 4-8) ⭐ CRITICAL

**Goal**: Optimize PathMap for performance while maintaining functionality.

#### 4.1 Current Issues

PathMap has 28,772 lines with:
- Massive duplication in zipper implementations
- Multiple overlapping abstractions
- No clear separation between core trie and operations
- Performance issues with large tries

#### 4.2 Optimization Strategy

1. **Core Trie Simplification**
   - Reduce node types to essential variants
   - Eliminate redundant state
   - Use SmallVec for child collections

2. **Zipper Consolidation**
   - Common base implementation for all zippers
   - Eliminate code duplication across zipper types
   - Use trait-based composition

3. **Memory Optimization**
   - Arena allocation for batch operations
   - Reduce pointer chasing
   - Cache-friendly node layout

4. **Performance Optimizations**
   - Inline hot paths
   - Use SIMD for path comparisons
   - Parallel operations for large tries

#### 4.3 New PathMap Architecture

```rust
// pathmap/trie/mod.rs
pub struct Trie<V> {
    root: Node<V>,
}

enum Node<V> {
    Empty,
    Leaf(V),
    Branch(Box<BranchNode<V>>),
}

struct BranchNode<V> {
    children: SmallVec<[(u8, Node<V>); 4]>, // SmallVec for common case
}

// pathmap/zipper/mod.rs
pub struct Zipper<'a, V> {
    focus: &'a mut Node<V>,
    path: Vec<PathComponent<V>>,
}

enum PathComponent<V> {
    Sibling(u8, Node<V>),
    Parent(u8),
}
```

#### 4.4 Success Criteria
- [ ] 30% reduction in PathMap lines of code
- [ ] 2x performance improvement on common operations
- [ ] Clear separation: trie core vs operations
- [ ] All existing tests pass

---

### Phase 5: API Ergonomics (Week 5-6)

**Goal**: Fluent, intuitive API with zero boilerplate for common cases.

#### 5.1 Simplified API

```rust
// api/mod.rs

// Zero-boilerplate usage
let mut engine = PeTTa::new()?;
let result = engine.eval("!(+ 1 2)")?;
assert_eq!(result, "3");

// Fluent builder
let mut engine = PeTTa::builder()
    .backend(Backend::Mork)
    .verbose(true)
    .build()?;

// Load files
engine.load("defs.metta")?;
engine.load_all(&["rules.metta", "queries.metta"])?;

// Execute
let result = engine.execute("!(your-query)")?;
let first = result.first()?.as_string()?;
```

#### 5.2 Consistent Result Types

```rust
// api/result.rs
pub struct ExecutionResult {
    values: Vec<MettaValue>,
    stats: ExecutionStats,
    warnings: Vec<Warning>,
}

impl ExecutionResult {
    pub fn first(&self) -> Option<&MettaValue> { self.values.first() }
    pub fn as_string(&self) -> Option<String> { /* ... */ }
    pub fn as_int(&self) -> Option<i64> { /* ... */ }
    pub fn stats(&self) -> &ExecutionStats { &self.stats }
}

pub struct ExecutionStats {
    pub duration: Duration,
    pub reductions: usize,
    pub allocations: usize,
}
```

#### 5.3 Success Criteria
- [ ] Common cases require zero boilerplate
- [ ] Consistent result types across all operations
- [ ] Fluent builder for advanced configuration
- [ ] Comprehensive examples in documentation

---

### Phase 6: Type System Improvements (Week 6-7)

**Goal**: Leverage Rust's type system for compile-time safety.

#### 6.1 Type-State Pattern

```rust
// api/engine.rs
pub struct PeTTa<State> {
    engine: PeTTaEngine,
    _state: PhantomData<State>,
}

// State types
pub struct Uninitialized;
pub struct Initialized;
pub struct Running;

impl PeTTa<Uninitialized> {
    pub fn new() -> Result<PeTTa<Initialized>, Error> {
        // ...
    }
}

impl PeTTa<Initialized> {
    pub fn start(self) -> Result<PeTTa<Running>, Error> {
        // ...
    }
}

impl PeTTa<Running> {
    pub fn execute(&mut self, code: &str) -> Result<ExecutionResult, Error> {
        // ...
    }
    
    pub fn stop(self) -> Result<PeTTa<Initialized>, Error> {
        // ...
    }
}
```

#### 6.2 Path Types

```rust
// core/types.rs
pub struct ProjectRoot(PathBuf);
pub struct MettaFile(PathBuf);

impl ProjectRoot {
    pub fn resolve(&self, path: &str) -> MettaFile {
        MettaFile(self.0.join(path))
    }
}

// Usage
let root = ProjectRoot::new("/path/to/project")?;
let file = root.resolve("defs.metta"); // Type-safe
engine.load(file)?;
```

#### 6.3 Success Criteria
- [ ] Compile-time state validation
- [ ] Path type safety
- [ ] No runtime cost for type safety

---

### Phase 7: Performance Optimization (Week 7-10)

**Goal**: Maximize performance through targeted optimizations.

#### 7.1 Hot Path Optimization

1. **Backend Selection**: Cache backend capabilities
2. **Expression Parsing**: Use nom with zero-copy where possible
3. **Memory Allocation**: SmallVec for small collections
4. **String Handling**: SmartString or Box<str> for owned strings

#### 7.2 MORK-Specific Optimizations

1. **Parallel Execution**: Use rayon for batch operations
2. **SIMD**: Use SIMD for path comparisons
3. **Arena Allocation**: Batch allocate expressions
4. **Caching**: Cache parsed expressions

#### 7.3 PathMap Optimizations

1. **Node Layout**: Cache-friendly node structure
2. **Zipper Operations**: Inline hot paths
3. **Memory Pooling**: Reuse allocated nodes
4. **Parallel Operations**: Parallel map/reduce on tries

#### 7.4 Success Criteria
- [ ] 2-3x performance improvement on benchmarks
- [ ] Reduced memory allocations
- [ ] No regression in test suite

---

## MORK Backend Strategy

### Current State
- MORK is a separate module with 12,256 lines
- Gated behind `mork` feature flag
- Different error handling than Prolog backend
- Separate interpreter implementation

### Integration Plan

1. **Unify Interface**: Implement unified `Backend` trait
2. **Shared Error Types**: Use common error types
3. **Feature Detection**: Use capabilities for feature detection
4. **Seamless Switching**: Allow runtime backend switching

### MORK-Specific Optimizations

1. **Interpreter**: Optimize zipper-based execution
2. **Space Management**: Efficient atom space operations
3. **Parallel Execution**: Leverage rayon for parallelism
4. **Memory Management**: Arena allocation for expressions

---

## PathMap Strategy

### Current State
- 28,772 lines of code
- 30+ files in flat structure
- Massive duplication in zipper implementations
- Performance issues with large tries

### Optimization Plan

1. **Core Simplification**: Reduce to essential operations
2. **Zipper Consolidation**: Common base implementation
3. **Memory Optimization**: Arena allocation, reduced allocations
4. **Performance**: SIMD, parallel operations

### PathMap Module Structure

```
pathmap/
├── mod.rs                 # Public API
├── core/                  # Core trie implementation
│   ├── mod.rs
│   ├── node.rs           # Node types
│   └── ops.rs            # Core operations
├── zipper/               # Zipper implementations
│   ├── mod.rs
│   ├── base.rs          # Base zipper
│   ├── overlay.rs       # Overlay zipper
│   └── product.rs       # Product zipper
├── ring/                # Ring operations
├── morphisms/           # Morphism operations
├── utils/               # Utilities
└── arena/               # Arena allocation
```

---

## Testing Strategy

### Test Categories

1. **Unit Tests**: All public functions
2. **Integration Tests**: Backend parity, end-to-end
3. **Property Tests**: Parser, trie operations
4. **Benchmarks**: Performance regression detection

### Test Coverage Goals

- **Core Engine**: >95%
- **Backends**: >90%
- **PathMap**: >85%
- **Parser**: >95%

### Differential Testing

Continue using differential testing between backends:
- Prolog backend as reference
- MORK backend for performance
- Automatic parity checks

---

## Migration Plan

### Week 1-2: Foundation
- [ ] Module reorganization
- [ ] Backend trait definition
- [ ] Error type consolidation

### Week 3-4: Core Implementation
- [ ] Backend implementations (Swipl, Mork)
- [ ] PathMap optimization
- [ ] Parser improvements

### Week 5-6: API Enhancement
- [ ] Fluent API implementation
- [ ] Type system improvements
- [ ] Documentation

### Week 7-8: Performance
- [ ] Hot path optimization
- [ ] Memory optimization
- [ ] Benchmark suite

### Week 9-10: Polish
- [ ] Final testing
- [ ] Documentation complete
- [ ] Migration guide
- [ ] Release preparation

---

## Risk Mitigation

### Technical Risks

1. **Backend Compatibility**: Maintain test suite throughout
2. **Performance Regression**: Comprehensive benchmarks
3. **Breaking Changes**: Provide migration guide
4. **Feature Loss**: Feature parity checks

### Mitigation Strategies

1. **Incremental Refactoring**: One module at a time
2. **Comprehensive Testing**: Test-driven refactoring
3. **Documentation**: Keep docs updated
4. **Community Feedback**: Regular updates and reviews

---

## Success Metrics

### Code Quality
- [ ] 25-35% reduction in total lines (eliminate ~35K lines)
- [ ] Clear module boundaries
- [ ] Comprehensive documentation
- [ ] >90% test coverage

### Performance
- [ ] 2-3x improvement on benchmarks
- [ ] Reduced memory allocations
- [ ] Faster compile times (10-15%)

### Developer Experience
- [ ] Zero-boilerplate common cases
- [ ] Actionable error messages
- [ ] Clear API documentation
- [ ] Positive developer feedback

### Functionality
- [ ] All existing features preserved
- [ ] MORK and PathMap fully integrated
- [ ] Backend parity maintained
- [ ] No regression in test suite

---

## Conclusion

This comprehensive refactoring plan will transform PeTTa into a modern, ergonomic, and high-performance MeTTa runtime. The key focuses are:

1. **Unified Architecture**: Single source of truth, no duplication
2. **Type Safety**: Compile-time guarantees, type-state pattern
3. **Performance**: Optimized hot paths, reduced allocations
4. **Ergonomics**: Intuitive APIs, excellent error messages
5. **Integration**: Full MORK and PathMap support

The end result will be a codebase that is a pleasure to work with, easy to extend, and performant in production.

---

## Appendix: File Checklist

### Files to Create
- [ ] `src/api/mod.rs`
- [ ] `src/api/engine.rs`
- [ ] `src/api/config.rs`
- [ ] `src/api/result.rs`
- [ ] `src/core/mod.rs`
- [ ] `src/core/backend.rs`
- [ ] `src/core/errors.rs`
- [ ] `src/core/values.rs`
- [ ] `src/backends/mod.rs`
- [ ] `src/backends/swipl/mod.rs`
- [ ] `src/backends/mork/mod.rs`
- [ ] `src/parser/mod.rs`
- [ ] `src/pathmap/trie/mod.rs`
- [ ] `src/pathmap/zipper/mod.rs`

### Files to Migrate
- [ ] `src/engine/mod.rs` → `src/api/engine.rs`
- [ ] `src/engine/config.rs` → `src/api/config.rs`
- [ ] `src/engine/backend.rs` → `src/core/backend.rs`
- [ ] `src/engine/errors.rs` → `src/core/errors.rs`
- [ ] `src/values.rs` → `src/core/values.rs`
- [ ] `src/parser/mod.rs` → `src/parser/sexpr.rs`
- [ ] `src/pathmap/mod.rs` → `src/pathmap/trie/mod.rs`

### Files to Deprecate
- [ ] `src/main.rs` (replace with `src/bin/petta.rs`)
- [ ] `src/core.rs` (merge into `src/api/`)
- [ ] `src/engine/backend.rs` (consolidate)
- [ ] `src/engine/backends.rs` (consolidate)

---

*Last Updated: 2026-05-01*  
*Status: Ready for Implementation* ✅
