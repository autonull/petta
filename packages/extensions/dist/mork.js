"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MorkSpace = void 0;
// Reimplement Mork vector integration inside TS instead of native python bindings
const vector_store_1 = require("./vector_store");
const llm_1 = require("./llm");
class MorkSpace {
    store;
    atoms = [];
    constructor() {
        this.store = new vector_store_1.VectorStore();
    }
    async addAtom(atom) {
        this.atoms.push(atom);
        // Optional: generate an embedding for text representation if string-like
        try {
            const strRep = JSON.stringify(atom);
            const embedding = await (0, llm_1.openrouter_embed)('text-embedding-3-small', strRep);
            this.store.add(strRep, embedding);
        }
        catch (e) {
            // Ignore embed errors without openrouter keys
        }
        return true;
    }
    removeAtom(atom) {
        this.atoms = this.atoms.filter(a => JSON.stringify(a) !== JSON.stringify(atom));
        return true;
    }
    match(pattern) {
        // Simple structural matching mapping to tau-prolog style lists
        return this.atoms.filter(a => {
            return this.isMatch(pattern, a);
        });
    }
    isMatch(pattern, atom) {
        if (pattern === null || atom === null)
            return pattern === atom;
        if (typeof pattern !== typeof atom) {
            // Tau-prolog var representations usually come through as object representations
            // Simplest naive unification for now
            if (pattern && typeof pattern === 'object' && pattern.id === 'Var') {
                return true;
            }
            return false;
        }
        if (typeof pattern === 'object') {
            if (Array.isArray(pattern)) {
                if (pattern.length !== atom.length)
                    return false;
                for (let i = 0; i < pattern.length; i++) {
                    if (!this.isMatch(pattern[i], atom[i]))
                        return false;
                }
                return true;
            }
            else {
                if (pattern.id === 'Var')
                    return true;
                return JSON.stringify(pattern) === JSON.stringify(atom);
            }
        }
        return pattern === atom;
    }
}
exports.MorkSpace = MorkSpace;
