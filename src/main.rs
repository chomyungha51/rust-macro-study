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



/*

1.

error: macro expansion ignores token `v` and any following
  --> src\main.rs:65:9
   |
65 |         v

=========================================

2.

error: expected expression, found statement (`let`)
  --> src\main.rs:64:9
   |
64 |         let mut v = Vec::new();

=========================================

3.

error[E0658]: `let` expressions in this position are unstable
  --> src\main.rs:64:9
   |
64 |         let mut v = Vec::new();

=========================================

4.

error[E0308]: mismatched types
  --> src\main.rs:64:9
   |
64 |         let mut v = Vec::new();
   |         ^^^^^^^^^^^^^^^^^^^^^^ expected struct `Vec`, found `bool`

=========================================

5.
error[E0308]: mismatched types
  --> src\main.rs:73:13
   |
73 |             v
   |             ^ expected `u32`, found array `[{integer}; 5]`


*/


#[macro_export]
macro_rules! custom_vec {

    // emtpy vector
    () => {
        let mut v = Vec::new();
        v
    };

    // put all arguments
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    };

    // repeat the element count times
    ([$element:expr;$count:expr]) => {
        {
            let mut v = Vec::new();
            for i in 1..$count {
                v.push($x)
            }
            v
        }
    };
}

fn main() {

    print_items!(1, "two", 3);

    println!();

    println_items!("first line", "second line", "third line" );

    let v1:Vec<i32> = custom_vec!();
    let v2:Vec<u32> = custom_vec!(1,2,3,4,5);
    let v3:Vec<u32> = custom_vec!([1;5]);

}
