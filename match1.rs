// match1.rs
fn main() {
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        println!("'{}'", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find("п");
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }

    // rustier
    let multilingual = "Hi! ¡Hola!";  // no match case
    match multilingual.find("п") {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        },
        None => println!("couldn't find the greeting, Товарищ")
    }
    // only car about match case
    let multilingual = "Hi! ¡Hola! привет!";
    if let Some(idx) = multilingual.find("п") {
        println!("Russian hi {}", &multilingual[idx..]);
    }

    let s = "¡";
    println!("{}", &s[0..1]);  // runtime error due to 1st byte of multibyte character
}
