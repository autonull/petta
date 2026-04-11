"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = __importDefault(require("fs"));
const path_1 = __importDefault(require("path"));
const petta_1 = require("./petta");
async function runTests() {
    const examplesDir = path_1.default.join(__dirname, '../../../examples');
    const files = fs_1.default.readdirSync(examplesDir).filter(f => f.endsWith('.metta'));
    let failed = 0;
    for (const file of files) {
        if (['repl.metta', 'llm_cities.metta', 'torch.metta', 'greedy_chess.metta', 'git_import2.metta'].includes(file)) {
            continue;
        }
        console.log(`Running test: ${file}`);
        try {
            await (0, petta_1.runCli)([path_1.default.join(examplesDir, file)]);
        }
        catch (e) {
            console.error(`Failed ${file}:`, e);
            failed++;
        }
    }
    if (failed > 0) {
        process.exit(1);
    }
}
runTests();
