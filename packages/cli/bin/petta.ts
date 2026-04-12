import path from 'path';
import { SessionManager } from '@petta/core';

export async function runCli(args: string[]) {
    if (args.length === 0) {
        console.error("Usage: petta <file.metta>");
        return;
    }

    const sessionManager = new SessionManager();
    await sessionManager.loadCore();

    const file = args[0];
    const absolutePath = path.resolve(file);

    return new Promise<void>((resolve, reject) => {
        sessionManager.session.query(`load_metta_file('${absolutePath}', Results), maplist(swrite, Results, ResultsR), maplist(format('~w~n'), ResultsR).`);
        const formatAnswer = (x: any): string => {
            if (x && x.id === 'throw') {
                return sessionManager.session.format_answer(x);
            }
            return String(x);
        };

        sessionManager.session.answer((x: any) => {
            if (x === false) {
                // If the file executed correctly, but the result is false
                resolve();
            } else if (x && x.id === 'throw') {
                reject(formatAnswer(x));
            } else {
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
