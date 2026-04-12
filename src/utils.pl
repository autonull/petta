:- use_module(library(lists)).

goals_list_to_conj([], true) :- !.
goals_list_to_conj([G], G) :- !.
goals_list_to_conj([G|Gs], (G,R)) :- goals_list_to_conj(Gs, R).

memberchk_eq(V, [H|_]) :- V == H, !.
memberchk_eq(V, [_|T]) :- memberchk_eq(V, T).

maybe_print_compiled_clause(_, _, _) :- (current_prolog_flag(argv, Args), memberchk(silent, Args) -> ! ; fail).
maybe_print_compiled_clause(Label, FormTerm, Clause) :-
    swrite(FormTerm, FormStr),
    format("\e[33m-->  ~w  -->~n\e[36m~w~n\e[33m--> prolog clause -->~n\e[32m", [Label, FormStr]),
    portray_clause(current_output, Clause),
    format("\e[33m^^^^^^^^^^^^^^^^^^^^^~n\e[0m").

format_metta_sexpr(Term) :- swrite(Term, STerm),
    format("\e[33m--> metta sexpr -->~n\e[36m~w~n\e[33m^^^^^^^^^^^^^^^^^^^~n\e[0m", [STerm]).

format_metta_function(FormStr) :-
    format("\e[33m--> metta function -->~n\e[36m~w~n\e[33m^^^^^^^^^^^^^^^^^^^^^^~n\e[0m", [FormStr]).

format_metta_runnable(FormStr) :-
    format("\e[33m--> metta runnable  -->~n\e[36m!~w~n\e[33m^^^^^^^^^^^^^^^^^^^^^^^~n\e[0m", [FormStr]).

find_typechain(Fun, TypeChain) :- catch(match('&self', [':', Fun, TypeChain], TypeChain, TypeChain), _, fail).

get_typechains(Fun, TypeChains) :- findall(TC, find_typechain(Fun, TC), TypeChains).

invalid_pattern(Term) :- var(Term), !.
invalid_pattern(Term) :- cyclic_term(Term).

term_vars_to_list(Term, Vars) :- term_variables(Term, Vars).

exclude_var(Term, Excluded) :- term_variables(Term, Vars), exclude(==(Term), Vars, Excluded).