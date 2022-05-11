/*
MacroFagSpec

block : {...}
expr : expression
ident : variable or function name
item : module, function, typeAlias, struct, enum, union, trait, staticItem...
stmt : statement
tt : token tree
ty : type
vis : pub
...

MacroRepOp
* : any number of repetition
+ : any number but at least one
? : optional at most one

*/

#[macro_export]
macro_rules! custom_println_macro {
    ($arg:int) => { println!("{}", $arg);};
    (expr) => (expr);
}

#[macro_export]
macro_rules! custom_vec_macro {
    () => ();
    () => ();
    () => ();
}

#[macro_export]
macro_rules! to_str {
    () => ();
    () => ();
}

fn main() {

    custom_println_macro!();
    custom_vec_macro!();
    to_str!();
}
