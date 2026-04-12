import { VectorStore } from './vector_store';

export class FaissSpace {
    private stores: Map<number, VectorStore> = new Map();
    private nextId: number = 1;

    create(dimension: number): number {
        const id = this.nextId++;
        this.stores.set(id, new VectorStore());
        return id;
    }

    add(id: number, atom: any, vector: number[]) {
        const store = this.stores.get(id);
        if (store) {
            store.add(JSON.stringify(atom), vector);
        }
    }

    search(id: number, vector: number[], k: number) {
        const store = this.stores.get(id);
        if (!store) return [];
        return store.search(vector, k).map(res => {
            try {
                return [JSON.parse(res.id), res.similarity];
            } catch (e) {
                return [res.id, res.similarity];
            }
        });
    }

    remove(id: number, atom: any) {
        // Our VectorStore doesn't natively have remove right now, but we don't strict need it for the SRI match.
        // It's a nice to have. Let's patch VectorStore directly if we need to.
    }
}

// Global symbol vector cache
const sym_vec: Record<number, Map<string, number[]>> = {};
const rand_range = 0.2;

// Seeded random helper for determinism based on string if we want it to be reproducible
// across runs, but original embed.pl just generated a random vector when first encountered and cached it.
// We will mimic: persistent across the instance
function random_float_signed() {
    return (Math.random() - 0.5) * rand_range;
}

function random_vec(dim: number): number[] {
    const v = [];
    for (let i = 0; i < dim; i++) {
        v.push(random_float_signed());
    }
    return v;
}

function get_sym_vector(dim: number, sym: string): number[] {
    if (!sym_vec[dim]) {
        sym_vec[dim] = new Map();
    }
    if (!sym_vec[dim].has(sym)) {
        sym_vec[dim].set(sym, random_vec(dim));
    }
    return sym_vec[dim].get(sym)!;
}

function vec_add(a: number[], b: number[]): number[] {
    const out = [];
    for (let i = 0; i < a.length; i++) {
        out.push((a[i] || 0) + (b[i] || 0));
    }
    return out;
}

function scale_vec(v: number[], s: number): number[] {
    return v.map(x => x * s);
}

function norm(v: number[]): number {
    let s = 0;
    for (const x of v) s += x * x;
    return Math.sqrt(s);
}

function normalize(v: number[]): number[] {
    const n = norm(v);
    if (n === 0) return v;
    return scale_vec(v, 1.0 / n);
}

// Returns a normalized vector
export function embed(expr: any, dim: number = 64): number[] {
    const v0 = embed0(expr, dim);
    return normalize(v0);
}

function embed0(expr: any, dim: number): number[] {
    // Treat string/number/boolean/symbol as atomic
    if (expr === null || expr === undefined || typeof expr !== 'object') {
        const symStr = String(expr);
        return get_sym_vector(dim, symStr);
    }

    // Arrays
    if (Array.isArray(expr)) {
        const base = get_sym_vector(dim, 'list');
        let acc = [...base];
        for (const x of expr) {
            const vx = embed0(x, dim);
            acc = vec_add(acc, vx);
        }
        return acc;
    }

    // Objects with "id" property like tau-prolog Variables, or funcs
    // The Prolog version has Term =.. [F|Args] where F is the functor.
    // If it's a Tau-Prolog term representation (usually an array of strings in TS version)
    // we'll just treat it as an array if it matches tau prolog list style.
    // If it's a POJO, we'll serialize to string
    const symStr = JSON.stringify(expr);
    return get_sym_vector(dim, symStr);
}
