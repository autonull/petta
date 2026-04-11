# PeTTa Status

## Current State (branch `swipl_b`)
- ✅ Rust subprocess wrapper works (15/15 unit tests, 139/139 example tests)
- ✅ SWI-Prolog `.pl` files are untouched originals
- ✅ Python files removed, scripts updated, README updated
- ❌ Still depends on external `swipl` binary (not self-contained)
- ❌ No persistent state between calls (one process per query)

## Architecture
```
Rust binary (petta) + Rust library (petta crate)
  └── SWI-Prolog subprocess (swipl)
        └── MeTTa runtime (7 Prolog files in src/)
              └── MeTTa code (.metta files)
```

## Scryer Prolog Migration — Attempted, Not Viable

### What we tried:
1. **In-process Rust API** (`scryer-prolog` crate 0.10.0): Crashes with `index out of bounds`
   on complex Prolog codebases. Internal Scryer bug in heap/trail management.
2. **Scryer subprocess**: `:- dynamic` and `:- discontiguous` directives cause
   `syntax_error(incomplete_reduction)` when loading via `consult/1`.
   DCG expansion with `library(dcgs)` is incompatible with SWI-style code.

### Why we reverted:
- The 7 `.pl` files (~1500 lines) were written for SWI Prolog's dialect.
- Converting to Scryer is NOT a search-and-replace job — it's rewriting a compiler.
- The TODO.md estimated "~45 line changes" — this was wildly inaccurate.
- SWI-Prolog works perfectly with 100% test pass rate.

### What would be needed for Scryer:
- Rewrite DCG parsing to use Scryer's `library(dcgs)` dialect (different from SWI's `dcg/basics`)
- Replace all `library(clpfd)` with `library(clpz)` and verify arithmetic semantics
- Replace `library(yall)` `>>` lambdas with `library(lambda)` `\X^Goal` syntax
- Replace `with_mutex/2`, `transaction/1`, `concurrent_and/2` (SWI-only)
- Replace `portray_clause/2`, `file_name_extension/3`, `atomic_list_concat/3`
- Handle `:- discontiguous` warnings via manual declarations
- Replace `random_between/3` with `random_integer/3`
- Remove `py-call` Python bindings
- Test all 139 examples for behavioral equivalence

**Bottom line:** Keep SWI-Prolog for now. Scryer is a future research direction.

## Running
```bash
cargo test          # 15 unit tests
sh test.sh          # 139 example tests
cargo run -- file.metta   # Run a MeTTa file
```
