!(import! &self "ts_import_file.ts")

!(test (repr (js-call (ts_import_file.greet "PeTTa User"))) "Hello, PeTTa User from TypeScript!")
!(test (js-call (ts_import_file.add 10 20)) 30)
