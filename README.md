# PeTTa 🧠

**A production-ready MeTTa runtime — built for embedding, testing, and scale.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](test.sh)

---

## What is MeTTa?

**MeTTa** (Meta Type Talk) is a typed, eager, expressive language built on unified pattern matching and direct expression manipulation. It serves as both a **programming language** and a **knowledge representation format** — the declarative substrate for the TrueAGI / OpenCog Hyperon ecosystem.

MeTTa programs operate on expressions (atoms, lists, and functions) with direct pattern matching, making it ideal for systems that must reason about their own code and data.

---

## What is PeTTa?

PeTTa is an efficient, embeddable runtime for MeTTa. It compiles MeTTa source to the **Warren Abstract Machine (WAM)**, leveraging decades of proven Prolog execution semantics. The result is mathematically rigorous, battle-tested, and genuinely comprehensible.

This implementation delivers MeTTa as a **Rust crate** with:

- A clean library API for embedding
- Comprehensive test coverage
- Persistent subprocess management
- Structured error handling
- Optional MORK acceleration

---

## Goals

| Goal | Description |
|------|-------------|
| **Embeddable** | Import MeTTa into Rust applications, services, and tools as a library |
| **Tested** | 145+ tests across unit, integration, and example layers |
| **Reliable** | Structured errors, health checks, automatic restart on crash |
| **Performant** | Persistent engine eliminates spawn overhead; parallel execution; optional MORK |
| **Maintainable** | Clean boundaries, documented protocol, modular code |
| **Future-proof** | Dual parser, MORK integration, WASM-ready |

PeTTa is the foundation for building real systems with MeTTa.

---

## MORK: The Acceleration Backend

**MORK** (Meta Type Talk Optimal Reduction Kernel) is an alternative execution backend based on zipper-based reduction. It provides:

- **Native Rust execution** — no Prolog dependency when MORK is enabled
- **Multi-threaded parallelism** — concurrent expression reduction
- **Optimized hot paths** — zipper-based traversal and modification

MORK isopt-in and currently requires Rust nightly. Enable it with:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release --features mork
```

For workloads where Prolog is unavailable or maximum throughput is critical, MORK delivers significant acceleration.

---

## Key Features

### Library Crate

Import MeTTa into any Rust application with a simple API:

```toml
[dependencies]
petta = { path = "/path/to/petta" }
```

### Dual Parser Architecture

- **Prolog DCG** — full MeTTa semantics, complete feature set
- **Native Rust (nom)** — fast S-expression parsing for common cases

### Structured Error Handling

Every failure mode is typed and traceable, from undefined functions to stack overflow.

### Query Profiling

Built-in timing instrumentation for performance analysis.

### Binary Protocol

Language-agnostic communication over stdin/stdout. Any language can implement a client to communicate with the PeTTa engine.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       MeTTa Source Files                         │
└───────────────────────────┬─────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Rust Host / CLI                              │
│   PeTTaEngine  ──  EngineConfig  ──  profiler::QueryProfile        │
│   binary protocol: [type][len][payload] ↔ [status][results]        │
└───────────────────────────┬─────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Prolog Core                                │
│   parser.pl → translator.pl → specializer.pl               │
│   metta.pl → spaces.pl → utils.pl                     │
│              WAM execution engine                      │
└─────────────────────────────────────────────────────────────────┘
```

The Prolog WAM is the engine — mathematically proven, declaratively clear, and remarkably resilient. PeTTa embeds it behind a clean Rust FFI boundary with proper lifecycle management.

---

## Quick Start

### Prerequisites

- **SWI-Prolog >= 9.3** 🐠
- **Rust (stable)** 🦀

### Build

```bash
cargo build --release
```

### Run

```bash
# Single file
./target/release/petta examples/fib.metta

# Multiple files (persistent engine)
./target/release/petta examples/if.metta examples/state.metta examples/math.metta

# Interactive REPL
./target/release/petta

# Timed execution
./target/release/petta -t examples/fib.metta
```

### Test

```bash
# Rust test suite
cargo test

# Example suite
sh test.sh
```

---

## Using as a Library

### Configuration

Configure the engine with custom paths, timeouts, and restart behavior.

### Engine Lifecycle

- Create engine with `PeTTaEngine::new()`
- Load files with `load_metta_file()`
- Execute strings with `process_metta_string()`
- Check health with `is_alive()`
- Shutdown explicitly or drop

### Parallel Execution

Process multiple MeTTa strings in parallel using rayon.

---

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `swipl` | SWI-Prolog subprocess backend | on |
| `mork` | MORK zipper-based backend (nightly) | off |
| `parallel` | Rayon parallel batch | off |
| `profiling` | Query timing instrumentation | off |
| `fast-hasher` | gxhash acceleration | off |

---

## Project Structure

```
petta/
├── Cargo.toml          # Workspace
├── prolog/           # WAM engine
│   ├── parser.pl
│   ├── translator.pl
│   ├── specializer.pl
│   ├── spaces.pl
│   ├── filereader.pl
│   ├── metta.pl
│   └── utils.pl
├── lib/              # MeTTa stdlib
├── examples/         # 145+ examples
├── rust/
│   ├── src/
│   │   ├── lib.rs      # PeTTaEngine, protocol, errors
│   │   ├── main.rs     # CLI
│   │   ├── cli.rs     # Arg parsing
│   │   ├── repl.rs    # REPL
│   │   ├── profiler.rs
│   │   ├── parser/    # Native Rust parser
│   │   ├── engine/   # Subprocess management
│   │   ├── mork/    # MORK backend
│   │   └── pathmap/  # PathMap 0.3
│   └── tests/       # Rust test suite
```

---

## Binary Protocol

Language-agnostic communication over stdin/stdout:

**Request**: `[type:1][len:4][payload:N]`
- `F` = file, `S` = string, `Q` = quit, `C` = cancel

**Response**: `[status:1][...results...]`
- `0` = success, `1` = error

Any language can implement a client. 🌐

---

## The Opportunity

MeTTa represents a unique convergence — a language that is both **executable logic** and **knowledge representation**. It is the substrate for reasoning systems that can manipulate their own representations.

But language potential is realized only through runtime quality. A language without a tested, embeddable, production-ready runtime is a research prototype, not an engineering foundation.

**PeTTa is that foundation.** Built for:

- Services that expose MeTTa over HTTP/gRPC
- Tools that embed MeTTa as a DSL
- Edge deployments with minimal resources
- Research requiring reliable, reproducible execution

The best is yet to come. 🚀

---

## Dependencies & Credits

PeTTa integrates several open-source components:

### Core PeTTa

| Component | Author | License | Repository |
|-----------|--------|---------|-------------|
| PeTTa (Rust CLI + library) | Patrick Hammer | MIT | trueagi-io/PeTTa |
| PeTTa Prolog backend | Patrick Hammer | MIT | trueagi-io/PeTTa |

### MORK Ecosystem

| Component | Author | License | Repository |
|-----------|--------|---------|-------------|
| MORK (Meta Type Talk Optimal Reduction Kernel) | Adam Vandervorst, TrueAGI | (see repo) | trueagi-io/MORK |
| mork-expr | Adam Vandervorst | (see repo) | trueagi-io/MORK/expr |
| mork-frontend | Adam Vandervorst | (see repo) | trueagi-io/MORK/frontend |
| mork-interning | Adam Vandervorst | (see repo) | trueagi-io/MORK/interning |

### Dependencies

| Component | Author | License | Repository |
|-----------|--------|---------|-------------|
| PathMap | Adam Vandervorst | MIT | Adam-Vandervorst/PathMap |
| SWI-Prolog | Jan Wielemaker et al. | BSD-2 | SWI-Prolog/swipl-devel |
| thiserror | David Tolnay | MIT / Apache-2.0 | dtolnay/thiserror |
| tracing | Tokio contributors | MIT | tokio-rs/tracing |
| nom | Geoffroy Couprie | MIT | Geal/nom |
| rayon | Niko Matsakis, Josh Stone | MIT / Apache-2.0 | rayon-rs/rayon |
| smallvec | Servo developers | MIT / Apache-2.0 | servo/rust-smallvec |
| gxhash | Tommy et al. | MIT | luketpeterson/gxhash |
| xxhash-rust | Rust community | MIT / Apache-2.0 | rust-lang/xxhash-rust |
| tempfile | Steven Allen | MIT / Apache-2.0 | Stebalien/tempfile |
| faiss (optional) | Facebook Research | MIT | facebookresearch/faiss |

### Related Projects

| Project | Description | Repository |
|---------|-------------|-------------|
| hyperon-experimental | Reference MeTTa implementation | trueagi-io/hyperon-experimental |
| metta-wam | MeTTa WAM interpreter | trueagi-io/metta-wam |
| PLN | Probabilistic Logic Networks | trueagi-io/PLN |
| chaining | Forward/backward chaining | trueagi-io/chaining |
| metta-examples | MeTTa example library | trueagi-io/metta-examples |

---

## License

MIT — Copyright 2025 Patrick Hammer