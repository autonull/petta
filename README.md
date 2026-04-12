## PeTTa

Efficient MeTTa language implementation.

This project is a pure TypeScript and tau-prolog implementation of the original Python/SWI-Prolog PeTTa system. It is designed to be fully isomorphic and run seamlessly across Node.js environments and modern web browsers without requiring native C++ or SWI-Prolog dependencies.

Please check out the [Wiki](https://github.com/patham9/PeTTa/wiki) for more information on the original architecture.

### Features of the TypeScript Port

- **No Native Dependencies:** Does not require SWI-Prolog, Python, or native FFI extensions. It runs on a pure JavaScript Prolog engine (`tau-prolog`).
- **Isomorphic Execution:** Can be imported and evaluated directly in browsers and Node.js environments.
- **Monorepo Architecture:**
  - `@petta/core`: The core Prolog interpreter and AST evaluator.
  - `@petta/cli`: An executable TS-based test runner and CLI interface.
  - `@petta/extensions`: Replaces original Python/FFI extensions with TS-based vector-store and LLM integrations.
  - `@petta/stdlib`: A fully portable collection of standard libraries.

### Dependencies

- Node.js >= 18.x
- pnpm

### Usage

Install dependencies:

```bash
pnpm install
```

Example run:

```bash
npx tsx packages/cli/bin/petta.ts ./examples/nars_tuffy.metta
```

Running tests (executes all `.metta` files in `examples/`):

```bash
cd packages/cli
pnpm test
```

### MORK and FAISS spaces

The original FFI dependencies (`mork_ffi`, `faiss_ffi`) have been abstracted away. The `@petta/extensions` package provides modular vector-store adapters mimicking FAISS capabilities using standard TypeScript packages, running natively in memory.

### Extension libraries

Check out [Extension libraries](https://github.com/trueagi-io/PeTTa/wiki/Extension-libraries) for a set of extension libraries that can be invoked from MeTTa files directly from the git repository.

## Notebooks, Servers, Browser

Because the TypeScript port uses `tau-prolog` as its backend, the entire engine is embedded directly in JavaScript.

### MeTTa in the Browser

You can bundle `@petta/core` into web applications without needing WebAssembly compilations. See [Execution-in-browser](https://github.com/patham9/PeTTa/wiki/Execution-in-browser) for more information.

### Jupyter Notebook Support & MeTTa server

A Jupyter kernel and HTTP servers are theoretically compatible via standard `ts-node` or `tsx` executors, allowing high-performance native-feeling integration with standard Node.js server frameworks like Express or Fastify.
