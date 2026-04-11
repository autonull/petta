; consult is used internally. To mimic the old style we import it:
!(import_prolog_function consult)

(= (import_prolog_functions_from_file $prologfile $importfunctions)
  (let* (($prologfile (exists_file))
         ($prologfile (consult))
         ($function (superpose $importfunctions)))
    (import_prolog_function $function)))

; use_module doesn't work perfectly in JS standard environments
; but tau-prolog does implement it for some things, so we leave the interface:
!(import_prolog_function use_module)

(= (import_prolog_functions_from_module $prologfile $importfunctions)
  (let* (($prologfile (exists_file))
         ($prologfile (use_module))
         ($function (superpose $importfunctions)))
    (import_prolog_function $function)))

!(import_prolog_functions_from_file (library lib_import.pl) (static-import! git-import! use-module!))
