; Try to import a TS file that conflicts with a global variable, like console
; Since it throws, we can't easily catch it in normal flow without breaking tests...
; We could catch the import failure directly.
!(test (catch (import! &self "console.ts")) (Error "Cannot import module 'console': A global variable with this name already exists."))
