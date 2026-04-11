:- use_module(library(dcgs)).
:- use_module(library(charsio)).
:- use_module(library(lists)).

%Generate a MeTTa S-expression string from the Prolog list (inverse parsing):
swrite(Term, String) :- phrase(swrite_exp(Term), Codes),
                        atom_chars(String, Codes).
swrite_exp(Var)   --> { var(Var) }, !, "$", { write_term_to_chars(Var, [], A) }, A.
swrite_exp(Num)   --> { number(Num) }, !, { write_term_to_chars(Num, [], Cs) }, Cs.
swrite_exp(Str)   --> { is_list(Str) }, !, "\"", { escape_quotes(Str, Es) }, Es, "\"". % Strings are lists of chars
swrite_exp(Atom)  --> { atom(Atom) }, !, { atom_chars(Atom, Cs) }, Cs.
swrite_exp([H|T]) --> { \+ is_list([H|T]) }, !, "(", {atom_chars(cons, Cons)}, Cons, " ", swrite_exp(H), " ", swrite_exp(T), ")".
swrite_exp([H|T]) --> !, "(", seq([H|T]), ")".
swrite_exp([])    --> !, "()".
swrite_exp(Term)  --> { Term =.. [F|Args] }, "(", {atom_chars(F, FCs)}, FCs, ( { Args == [] } -> [] ; " ", seq(Args) ), ")".
seq([X])    --> swrite_exp(X).
seq([X|Xs]) --> swrite_exp(X), " ", seq(Xs).

escape_quotes([], []).
escape_quotes(['"'|T], ['\\','"'|R]) :- !, escape_quotes(T, R).
escape_quotes([H|T], [H|R]) :- escape_quotes(T, R).

sread(S, T) :- ( atom_chars(A, S),
                 phrase(sexpr(T, [], _), A)
               -> true ; throw(error(syntax_error(S), none)) ).

sexpr(S,E,E)  --> blanks, string_lit(S), blanks, !.
sexpr(T,E0,E) --> blanks, "(", blanks, seq(T,E0,E), blanks, ")", blanks, !.
sexpr(N,E,E)  --> blanks, number(N), ( lookahead_any([' ', '(', ')', '\t', '\n', '\r']) ; \+ [_] ), blanks, !.
sexpr(V,E0,E) --> blanks, var_symbol(V,E0,E), blanks, !.
sexpr(A,E,E)  --> blanks, atom_symbol(A), blanks.

lookahead_any(Terms, S, E) :- S = [Head | _], member(Head,Terms), !, S = E.

seq([X|Xs],E0,E2) --> sexpr(X,E0,E1), blanks, seq(Xs,E1,E2).
seq([],E,E)       --> [].

var_symbol(V,E0,E) --> "$", token(Cs), { ( Cs == ['_'] -> V = _, E = E0 ; memberchk(Cs-V0, E0) -> V = V0, E = E0 ; V = _, E = [Cs-V|E0] ) }.

atom_symbol(A) --> token(Cs), { ( Cs = ['"'|_] -> append(['"'|Body], ['"'], Cs), A = Body
                                                ; atom_chars(R, Cs),
                                                  ( R == 'True' -> A = true
                                                                ; R == 'False' -> A = false
                                                                               ; A = R ))}.

token([C|Cs]) --> [C], { \+ member(C, [' ', '\t', '\r', '\n', '(', ')']) }, token(Cs).
token([]) --> [].

blanks --> [C], { member(C, [' ', '\t', '\r', '\n']) }, !, blanks.
blanks --> [].

string_lit(S) --> "\"", string_chars(S), "\"".
string_chars([]) --> [].
string_chars([C|Cs]) --> [C], { C \= '"', C \= '\\' }, !, string_chars(Cs).
string_chars([C|Cs]) --> "\\", [X], { (X=='n'->C='\n'; X=='t'->C='\t'; X=='r'->C='\r'; C=X) }, string_chars(Cs).

number(N) --> digits(Ds), { Ds \= [], catch(number_chars(N, Ds), _, fail) }.
number(N) --> "-", digits(Ds), { Ds \= [], catch(number_chars(N, ['-'|Ds]), _, fail) }.
number(N) --> digits(D1), ".", digits(D2), { D1 \= [], D2 \= [], append(D1, ['.'|D2], Ds), catch(number_chars(N, Ds), _, fail) }.
number(N) --> "-", digits(D1), ".", digits(D2), { D1 \= [], D2 \= [], append(['-'|D1], ['.'|D2], Ds), catch(number_chars(N, Ds), _, fail) }.

digits([D|Ds]) --> digit(D), digits(Ds).
digits([]) --> [].

digit(D) --> [D], { member(D, ['0','1','2','3','4','5','6','7','8','9']) }.
