fn main() {
    println!("Hello, world!");
    let version = env!("CARGO_PKG_VERSION");
    println!("Program version: {}", version);
}