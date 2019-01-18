// args0.rs
fn main() {
    // for arg in std::env::args() {
    //     println!("'{}'", arg);
    // }

    // alternatively, and skipping the executable name
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        println!("'{}'", arg);
    }
}
