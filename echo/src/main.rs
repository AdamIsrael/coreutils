use std::env;

/// Echo the arguments
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut iter = args.iter();
    let mut sep = "";

    // Throw out the name of the binary
    iter.next();

    for argument in iter {
        print!("{sep}{argument}");
        if sep.is_empty() {
            sep = " ";
        }
    }
    println!();
}
