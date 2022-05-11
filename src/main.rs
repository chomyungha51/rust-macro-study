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
macro_rules! print_items {
    ($($arg:tt), *) => {
        $(print!("{}", $arg);)*
    };
}

#[macro_export]
macro_rules! println_items {
    ($($arg:tt), *) => {
        $(println!("{ }", $arg);)*
    };
}

#[macro_export]
macro_rules! get_length {
   ($arg:expr) => {
        let mut count = 0;
        for i in $arg {
            count++;
        }
   };
}

fn main() {

    print_items!(1, "two", 3);

    println!();

    println_items!("first line", "second line", "third line" );

}
