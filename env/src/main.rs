fn main() {
    for var in std::env::vars() {
        println!("{}={}", var.0, var.1);
    }
}
