// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.




macro_rules! my_macro {//macro 只有在它之后的代码可见
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}