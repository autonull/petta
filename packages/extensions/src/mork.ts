// Reimplement Mork vector integration inside TS instead of native python bindings
import { VectorStore } from './vector_store';
import { openrouter_embed } from './llm';

export class MorkSpace {
    private store: VectorStore;
    private atoms: any[] = [];

    constructor() {
        this.store = new VectorStore();
    }

    async addAtom(atom: any) {
        this.atoms.push(atom);
        // Optional: generate an embedding for text representation if string-like
        try {
            const strRep = JSON.stringify(atom);
            const embedding = await openrouter_embed('text-embedding-3-small', strRep);
            this.store.add(strRep, embedding);
        } catch(e) {
            // Ignore embed errors without openrouter keys
        }
        return true;
    }

    removeAtom(atom: any) {
        this.atoms = this.atoms.filter(a => JSON.stringify(a) !== JSON.stringify(atom));
        return true;
    }

    match(pattern: any) {
        // Simple structural matching mapping to tau-prolog style lists
        return this.atoms.filter(a => {
            return this.isMatch(pattern, a);
        });
    }

    private isMatch(pattern: any, atom: any): boolean {
        if (pattern === null || atom === null) return pattern === atom;
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
                if (pattern.length !== atom.length) return false;
                for (let i = 0; i < pattern.length; i++) {
                    if (!this.isMatch(pattern[i], atom[i])) return false;
                }
                return true;
            } else {
                if (pattern.id === 'Var') return true;
                return JSON.stringify(pattern) === JSON.stringify(atom);
            }
        }
        return pattern === atom;
    }
}
