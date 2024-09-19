// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro2 {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro2!();
}

