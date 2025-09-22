// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


#[rustfmt::skip]
macro_rules! my_macro {
    //macro_rules! 要求多个规则之间必须用分号或逗号隔开
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
