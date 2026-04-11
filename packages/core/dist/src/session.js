"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.SessionManager = void 0;
const tau_prolog_1 = __importDefault(require("tau-prolog"));
const fs_1 = __importDefault(require("fs"));
const path_1 = __importDefault(require("path"));
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
        this.setupBindings();
    }
    setupBindings() {
        // Here we map custom JS predicates so Prolog can call Node.js / TS functions.
        // 1. js_read_file_to_string(Path, StringOut)
        // Wait, Tau-prolog doesn't easily let us add predicates programmatically.
        // It's better to create a tau-prolog module or evaluate a string of Prolog defining them.
        // Make the library path resolution work
        const stdlibPath = path_1.default.resolve(__dirname, '../../stdlib/lib');
        const hooks = `
            :- use_module(library(js)).

            :- dynamic library_path/1.
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
        // Load hooks
        this.session.consult(hooks, {
            success: () => { },
            error: (err) => console.error("Error setting up bindings:", err)
        });
    }
    async loadCore() {
        const coreDir = path_1.default.join(__dirname, 'prolog');
        // Actually load src/prolog/metta.pl and its dependencies
        const mettaPl = fs_1.default.readFileSync(path_1.default.join(coreDir, 'metta.pl'), 'utf8');
        return new Promise((resolve, reject) => {
            this.session.consult(mettaPl, {
                success: () => resolve(),
                error: (err) => reject(err)
            });
        });
    }
}
exports.SessionManager = SessionManager;
