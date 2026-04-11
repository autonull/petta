import fs from 'fs';
import path from 'path';
import { runCli } from './petta';

async function runTests() {
    const examplesDir = path.join(__dirname, '../../../examples');
    const files = fs.readdirSync(examplesDir).filter(f => f.endsWith('.metta'));

    let failed = 0;
    for (const file of files) {
        if (['repl.metta', 'llm_cities.metta', 'torch.metta', 'greedy_chess.metta', 'git_import2.metta'].includes(file)) {
            continue;
        }
        console.log(`Running test: ${file}`);
        try {
            await runCli([path.join(examplesDir, file)]);
        } catch(e) {
            console.error(`Failed ${file}:`, e);
            failed++;
        }
    }
    if (failed > 0) {
        process.exit(1);
    }
}

runTests();
