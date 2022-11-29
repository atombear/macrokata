////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
fn show_output() {
    println!("I should appear as the output.")
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `show_output!()` macro.
macro_rules! show_output {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        show_output()
    };
}
////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    show_output!()
}
