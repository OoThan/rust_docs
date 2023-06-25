// This is a simple marco named `say_hello`
macro_rules! say_hello {
    // `()` indicated that the marco takes no arguments
    () => {
        // The marco expand into the contents of this block.
        println!("Hello from marco fn")
    };
}

fn main() {
    // This call will expand into `println!("Hello")`
    say_hello!()
}
