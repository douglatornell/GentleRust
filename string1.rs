// string1.rs
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly";  // string slice
    let s = text.to_string();  // allocated string

    dump(text);
    dump(&s);
}
