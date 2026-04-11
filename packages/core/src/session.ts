import pl from 'tau-prolog';
import fs from 'fs';
import path from 'path';

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
        this.setupBindings();
    }

    private setupBindings() {
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

        // Load hooks
        this.session.consult(hooks, {
            success: () => {},
            error: (err: any) => console.error("Error setting up bindings:", err)
        });
    }

    public async loadCore() {
        const coreDir = path.join(__dirname, '../src/prolog');
        // Actually load src/prolog/metta.pl and its dependencies
        const mettaPl = fs.readFileSync(path.join(coreDir, 'metta.pl'), 'utf8');

        return new Promise<void>((resolve, reject) => {
            this.session.consult(mettaPl, {
                success: () => resolve(),
                error: (err: any) => reject(err)
            });
        });
    }
}
