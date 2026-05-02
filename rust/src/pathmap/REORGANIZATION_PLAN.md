# PathMap Reorganization Plan

## Current State
- Total: 28,772 lines across 20+ files
- Flat structure with poor separation of concerns
- Multiple zipper implementations with duplication
- Core trie logic mixed with operations

## Target Structure (per TODO.md)
```
pathmap/
├── mod.rs              # Public API exports
├── trie/               # Core trie implementation
│   ├── mod.rs
│   ├── node.rs         # Trie node types  
│   └── ops.rs          # Core operations
├── zipper/             # Zipper implementations
│   ├── mod.rs
│   ├── base.rs         # Base zipper
│   ├── overlay.rs      # Overlay zipper
│   ├── product.rs      # Product zipper
│   └── write.rs        # Write zipper
├── ring/               # Ring operations
├── morphisms/          # Morphism operations
├── utils/              # Utilities
└── arena/              # Arena allocation (optional)
```

## Migration Steps

### Step 1: Consolidate Zipper Files (Priority: HIGH)
- [ ] Merge `zipper.rs` (5607 lines) into `zipper/base.rs`
- [ ] Merge `write_zipper.rs` (6402 lines) into `zipper/write.rs`
- [ ] Merge `product_zipper.rs` (1964 lines) into `zipper/product.rs`
- [ ] Merge `overlay_zipper.rs` (391 lines) into `zipper/overlay.rs`
- [ ] Remove old flat files after migration

### Step 2: Separate Core Trie (Priority: HIGH)
- [ ] Move trie core logic from `trie_core/` to `trie/`
- [ ] Create clear `ops.rs` for core operations
- [ ] Ensure `trie/` has no zipper dependencies

### Step 3: Eliminate Duplication (Priority: MEDIUM)
- [ ] Identify common zipper patterns
- [ ] Create shared base implementations
- [ ] Use trait-based composition

### Step 4: Performance Optimization (Priority: MEDIUM)
- [ ] Add SmallVec for child collections
- [ ] Inline hot paths
- [ ] Add arena allocation support

## Expected Benefits
- 30% reduction in lines of code (~8,600 lines)
- Clear separation: trie core vs operations
- 2x performance on common operations
- Easier to maintain and extend

## Risks
- High complexity - requires deep understanding
- Potential for breaking changes
- Time-intensive (estimated 2-3 weeks)
