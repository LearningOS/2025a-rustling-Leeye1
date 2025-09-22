// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    #[macro_export]
    //把这个宏导出到 crate 的根命名空间（crate root），
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

//在包根不可以直接些super因为已经没有父模块了

fn main() {
    my_macro!();
}
