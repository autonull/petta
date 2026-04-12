"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.SessionManager = void 0;
const tau_prolog_1 = __importDefault(require("tau-prolog"));
const extensions_1 = require("@petta/extensions");
const core_pl_1 = require("./core_pl");
// Require core modules
require('tau-prolog/modules/lists')(tau_prolog_1.default);
require('tau-prolog/modules/random')(tau_prolog_1.default);
require('tau-prolog/modules/format')(tau_prolog_1.default);
require('tau-prolog/modules/os')(tau_prolog_1.default);
require('tau-prolog/modules/js')(tau_prolog_1.default);
class SessionManager {
    session;
    constructor() {
        this.session = tau_prolog_1.default.create(10000);
    }
    async setupBindings() {
        // Here we map custom JS predicates so Prolog can call Node.js / TS functions.
        // 1. js_read_file_to_string(Path, StringOut)
        // Wait, Tau-prolog doesn't easily let us add predicates programmatically.
        // It's better to create a tau-prolog module or evaluate a string of Prolog defining them.
        // Make the library path resolution work conditionally for Node vs Browser
        let stdlibPath = '';
        if (typeof process !== 'undefined' && process.env) {
            const path = require('path');
            stdlibPath = path.resolve(__dirname, '../../stdlib/lib');
        }
        const hooks = `
            :- use_module(library(js)).

            :- dynamic(library_path/1).
            ${stdlibPath ? `:- asserta(library_path('${stdlibPath}')).` : ''}

            js_read_file_to_string(Path, StringOut) :-
                prop(js, readFileToString, Func),
                apply(Func, [Path], ReturnVal),
                StringOut = ReturnVal.

            js_call_predicate([Obj, Method | Args], Result) :-
                prop(js, Obj, JsObj),
                prop(JsObj, Method, Func),
                apply(Func, Args, Result).
        `;
        // Bind global JS objects
        globalThis.global = globalThis;
        let readFileStr = (p) => { throw new Error("File reading not supported in this environment"); };
        if (typeof process !== 'undefined' && process.env) {
            const fs = require('fs');
            readFileStr = (p) => fs.readFileSync(p, 'utf8');
        }
        globalThis.readFileToString = readFileStr;
        globalThis.__import__ = (p) => {
            // very basic mock of importing
        };
        globalThis._petta_format_error = (fmt, args) => {
            return fmt; // Just return format unparsed for now
        };
        globalThis._petta_format = (fmt, args) => {
            console.log(fmt, args);
            return true;
        };
        const morkSpace = new extensions_1.MorkSpace();
        globalThis.mork = {
            addAtom: (atom) => morkSpace.addAtom(atom),
            removeAtom: (atom) => morkSpace.removeAtom(atom),
            match: (pattern) => morkSpace.match(pattern)
        };
        const faissSpace = new extensions_1.FaissSpace();
        globalThis.faiss = {
            create: (dim) => faissSpace.create(dim),
            add: (id, atom, vector) => faissSpace.add(id, atom, vector),
            search: (id, vector, k) => faissSpace.search(id, vector, k),
            remove: (id, atom) => faissSpace.remove(id, atom),
            embed: (expr, dim) => (0, extensions_1.embed)(expr, dim)
        };
        globalThis.llm = {
            use_gpt: async (model, prompt, maxTokens, effort) => await (0, extensions_1.openrouter_chat)(model, prompt, maxTokens, effort),
            use_openrouter: async (model, prompt, maxTokens, effort) => await (0, extensions_1.openrouter_chat)(model, prompt, maxTokens, effort),
            use_gpt_embedding: async (text) => await (0, extensions_1.openrouter_embed)('text-embedding-3-small', text),
            use_openrouter_embedding: async (model, text) => await (0, extensions_1.openrouter_embed)(model, text)
        };
        // Load hooks
        await new Promise((r, e) => {
            this.session.consult(hooks, {
                success: r,
                error: (err) => e(new Error("Error setting up bindings: " + this.session.format_answer(err)))
            });
        });
    }
    async loadCore() {
        await this.setupBindings();
        return new Promise((resolve, reject) => {
            this.session.consult(core_pl_1.CORE_PL, {
                success: () => resolve(),
                error: (err) => {
                    console.error("Consult error:", this.session.format_answer(err));
                    reject(new Error(this.session.format_answer(err)));
                }
            });
        });
    }
}
exports.SessionManager = SessionManager;
