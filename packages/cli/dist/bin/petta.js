"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.runCli = runCli;
const path_1 = __importDefault(require("path"));
const core_1 = require("@petta/core");
async function runCli(args) {
    if (args.length === 0) {
        console.error("Usage: petta <file.metta>");
        return;
    }
    const sessionManager = new core_1.SessionManager();
    await sessionManager.loadCore();
    const file = args[0];
    const absolutePath = path_1.default.resolve(file);
    return new Promise((resolve, reject) => {
        sessionManager.session.query(`load_metta_file('${absolutePath}', Results), maplist(swrite, Results, ResultsR), maplist(format('~w~n'), ResultsR).`);
        const formatAnswer = (x) => {
            if (x && x.id === 'throw') {
                return sessionManager.session.format_answer(x);
            }
            return String(x);
        };
        sessionManager.session.answer((x) => {
            if (x === false) {
                // If the file executed correctly, but the result is false
                resolve();
            }
            else if (x && x.id === 'throw') {
                reject(formatAnswer(x));
            }
            else {
                resolve();
            }
        });
    });
}
if (require.main === module) {
    const args = process.argv.slice(2);
    runCli(args).catch(err => {
        console.error(err);
        process.exit(1);
    });
}
