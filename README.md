## PeTTa

Efficient MeTTa language implementation in Rust + SWI-Prolog.

### Dependencies

- SWI-Prolog >= 9.3.x
- Rust >= 1.70 (for building the CLI wrapper)

### Building

```bash
sh build.sh
```

This verifies SWI-Prolog is available and builds the Rust CLI binary.

### Usage

Run a MeTTa file:

```bash
sh run.sh ./examples/fib.metta
./target/release/petta ./examples/fib.metta
```

Run the demo (no arguments):

```bash
./target/release/petta
```

Pass multiple files:

```bash
./target/release/petta ./examples/if.metta ./examples/state.metta
```

Verbose mode:

```bash
./target/release/petta -v ./examples/fib.metta
```

### Testing

Run the full example test suite:

```bash
sh test.sh
```

Run the Rust unit tests:

```bash
cargo test
```

### Using as a Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
petta = { path = "/path/to/petta" }
```

Then in Rust:

```rust
use petta::PettaEngine;
use std::path::Path;

let engine = PettaEngine::new(Path::new("/path/to/petta"), false)?;

// Process MeTTa code strings
let results = engine.process_metta_string("(= (myfunc $x) (+ $x 1)) !(myfunc 41)")?;
for r in &results {
    println!("{}", r.value);
}

// Load MeTTa files
let results = engine.load_metta_file(Path::new("examples/fib.metta"))?;
```

### Library System

MeTTa libraries in `lib/` are loaded automatically when `import!` is used from a MeTTa file.
The library path is resolved relative to the project root at runtime.

### Extension Libraries

Extension libraries can be imported from git repositories using `git-import!`:

```metta
(git-import! "https://github.com/example/my-metta-lib")
```

## Architecture

```
MeTTa (.metta files)
        |
        v
  [parser.pl]   -- DCG parser: sread/swrite convert between strings and Prolog terms
        |
        v
  [filereader.pl] -- Reads files, strips comments, identifies forms (! vs non-!)
        |
        v
  [translator.pl] -- Compiles MeTTa expressions to Prolog goals
        |              - Functions become Prolog clauses
        |              - Expressions become goal lists
        |              - reduce/call handle dispatch
        v
  [specializer.pl] -- Optional: creates specialized versions of higher-order calls
        |
        v
  [spaces.pl] -- Manages atom spaces (&self) as Prolog predicates
        |
        v
  SWI-Prolog WAM -- Executes compiled Prolog code
        |
        v
  Rust (petta) -- CLI wrapper + library for embedding
```

**MeTTa code** is the source language (S-expressions like `(= (fib $n) ...)`).
**SWI-Prolog** serves as the compilation target and execution engine (WAM).
**Rust** wraps SWI-Prolog as a subprocess, providing a clean CLI and embeddable library API.
