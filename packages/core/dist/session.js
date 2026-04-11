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
    }
    async setupBindings() {
        // Here we map custom JS predicates so Prolog can call Node.js / TS functions.
        // 1. js_read_file_to_string(Path, StringOut)
        // Wait, Tau-prolog doesn't easily let us add predicates programmatically.
        // It's better to create a tau-prolog module or evaluate a string of Prolog defining them.
        // Make the library path resolution work
        const stdlibPath = path_1.default.resolve(__dirname, '../../stdlib/lib');
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
        globalThis.global = globalThis;
        globalThis.readFileToString = (p) => fs_1.default.readFileSync(p, 'utf8');
        globalThis.__import__ = (p) => {
            // very basic mock of importing
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
        const coreDir = path_1.default.join(__dirname, '../src/prolog');
        const files = ['parser.pl', 'translator.pl', 'specializer.pl', 'filereader.pl', 'spaces.pl', 'metta.pl'];
        return new Promise(async (resolve, reject) => {
            try {
                let combinedPl = ":- op(700, xfx, '=@=').\n:- dynamic(library_path/1).\n:- dynamic(translator_rule/1).\n:- dynamic('get-type'/2).\n:- dynamic(fun/1).\n:- dynamic(silent/1).\n:- dynamic(ho_specialization/2).\n";
                for (const file of files) {
                    let content = fs_1.default.readFileSync(path_1.default.join(coreDir, file), 'utf8');
                    content = content.replace(/:- ensure_loaded\(\[.*?\]\)\./g, '');
                    content = content.replace(/:- ensure_loaded\(.*?\)\./g, '');
                    content = content.replace(/:- dynamic\(.*?\)\./g, '');
                    combinedPl += content + '\n';
                }
                this.session.consult(combinedPl, {
                    success: () => resolve(),
                    error: (err) => reject(new Error(this.session.format_answer(err)))
                });
            }
            catch (e) {
                reject(e);
            }
        });
    }
}
exports.SessionManager = SessionManager;
