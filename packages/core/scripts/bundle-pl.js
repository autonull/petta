"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = __importDefault(require("fs"));
const path_1 = __importDefault(require("path"));
const coreDir = path_1.default.join(__dirname, '../src/prolog');
const files = ['parser.pl', 'translator.pl', 'specializer.pl', 'filereader.pl', 'spaces.pl', 'metta.pl'];
let combinedPl = ":- op(700, xfx, '=@=').\\n:- dynamic(library_path/1).\\n:- dynamic(translator_rule/1).\\n:- dynamic('get-type'/2).\\n:- dynamic(fun/1).\\n:- dynamic(silent/1).\\n:- dynamic(ho_specialization/2).\\n";
for (const file of files) {
    let content = fs_1.default.readFileSync(path_1.default.join(coreDir, file), 'utf8');
    content = content.replace(/:- ensure_loaded\(\[.*?\]\)\./g, '');
    content = content.replace(/:- ensure_loaded\(.*?\)\./g, '');
    content = content.replace(/:- dynamic\(.*?\)\./g, '');
    combinedPl += content + '\n';
}
const outPath = path_1.default.join(__dirname, '../src/core_pl.ts');
fs_1.default.writeFileSync(outPath, `export const CORE_PL = \`${combinedPl.replace(/`/g, '\\`').replace(/\$/g, '\\$')}\`;\n`);
console.log(`Bundled ${files.length} prolog files into core_pl.ts`);
