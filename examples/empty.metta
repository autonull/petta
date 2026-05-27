(= (y) (empty))

; `(empty)` now evaluates to the symbol `empty` (not a failure sentinel)
!(test (== (y) empty)
       True)
