!(import! &self "ts_advanced_import.ts")

; Basic method call
!(test (js-call (ts_advanced_import.multiply 5 6)) 30)

; We don't have a way to easily access deeply nested object properties natively through js-call
; but let's test if we can grab a function from a sub-object
!(test (repr (js-call (ts_advanced_import.nested.call_nested))) "nested call success!")

; Trying to overwrite console should fail (in Tau-Prolog, js_call throws)
; Note: Since we are in the TS port, import is evaluated, and will throw before evaluating the test!
; So we can't easily catch it from MeTTa directly if it crashes the compiler/eval loop, but let's verify
; we avoid breaking `console` globally.
