import pl from 'tau-prolog';
import fs from 'fs';
import path from 'path';
import { openrouter_chat, openrouter_embed, MorkSpace, FaissSpace, embed as faiss_embed } from '@petta/extensions';

// Require core modules
require('tau-prolog/modules/lists')(pl);
require('tau-prolog/modules/random')(pl);
require('tau-prolog/modules/format')(pl);
require('tau-prolog/modules/os')(pl);
require('tau-prolog/modules/js')(pl);

export class SessionManager {
    session: any;

    constructor() {
        this.session = pl.create(10000);

    }

    private async setupBindings() {
        // Here we map custom JS predicates so Prolog can call Node.js / TS functions.

        // 1. js_read_file_to_string(Path, StringOut)
        // Wait, Tau-prolog doesn't easily let us add predicates programmatically.
        // It's better to create a tau-prolog module or evaluate a string of Prolog defining them.

        // Make the library path resolution work
        const stdlibPath = path.resolve(__dirname, '../../stdlib/lib');

        const hooks = `
            :- use_module(library(js)).

            :- dynamic(library_path/1).
            :- asserta(library_path('${stdlibPath}')).

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
                        (globalThis as any).global = globalThis;
        (globalThis as any).readFileToString = (p: string) => fs.readFileSync(p, 'utf8');
        (globalThis as any).__import__ = (p: string) => {
             // very basic mock of importing
        };
        (globalThis as any)._petta_format_error = (fmt: string, args: any[]) => {
            return fmt; // Just return format unparsed for now
        };
        (globalThis as any)._petta_format = (fmt: string, args: any[]) => {
            console.log(fmt, args);
            return true;
        };

        const morkSpace = new MorkSpace();
        (globalThis as any).mork = {
            addAtom: (atom: any) => morkSpace.addAtom(atom),
            removeAtom: (atom: any) => morkSpace.removeAtom(atom),
            match: (pattern: any) => morkSpace.match(pattern)
        };

        const faissSpace = new FaissSpace();
        (globalThis as any).faiss = {
            create: (dim: number) => faissSpace.create(dim),
            add: (id: number, atom: any, vector: number[]) => faissSpace.add(id, atom, vector),
            search: (id: number, vector: number[], k: number) => faissSpace.search(id, vector, k),
            remove: (id: number, atom: any) => faissSpace.remove(id, atom),
            embed: (expr: any, dim: number) => faiss_embed(expr, dim)
        };

        (globalThis as any).llm = {
            use_gpt: async (model: string, prompt: string, maxTokens: number, effort: string) => await openrouter_chat(model, prompt, maxTokens, effort),
            use_openrouter: async (model: string, prompt: string, maxTokens: number, effort: string) => await openrouter_chat(model, prompt, maxTokens, effort),
            use_gpt_embedding: async (text: string) => await openrouter_embed('text-embedding-3-small', text),
            use_openrouter_embedding: async (model: string, text: string) => await openrouter_embed(model, text)
        };

        // Load hooks
        await new Promise<void>((r, e) => {
            this.session.consult(hooks, {
                success: r,
                error: (err: any) => e(new Error("Error setting up bindings: " + this.session.format_answer(err)))
            });
        });
    }

    public async loadCore() {
        await this.setupBindings();
        const coreDir = path.join(__dirname, '../src/prolog');

        const files = ['parser.pl', 'translator.pl', 'specializer.pl', 'filereader.pl', 'spaces.pl', 'metta.pl'];

        return new Promise<void>(async (resolve, reject) => {
            try {
                let combinedPl = ":- op(700, xfx, '=@=').\n:- dynamic(library_path/1).\n:- dynamic(translator_rule/1).\n:- dynamic('get-type'/2).\n:- dynamic(fun/1).\n:- dynamic(silent/1).\n:- dynamic(ho_specialization/2).\n";
                for (const file of files) {
                    let content = fs.readFileSync(path.join(coreDir, file), 'utf8');
                    content = content.replace(/:- ensure_loaded\(\[.*?\]\)\./g, '');
                    content = content.replace(/:- ensure_loaded\(.*?\)\./g, '');
                    content = content.replace(/:- dynamic\(.*?\)\./g, '');
                    combinedPl += content + '\n';
                }

                this.session.consult(combinedPl, {
                    success: () => resolve(),
                    error: (err: any) => reject(new Error(this.session.format_answer(err)))
                });
            } catch(e) {
                reject(e);
            }
        });
    }
}
