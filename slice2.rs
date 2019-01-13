// slice2.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());
    // println!("last value {}", last.unwrap());  panic!!
    println!("last value {}", *last.unwrap_or(&-1));

    // Note difference between de-referencing and not
    // let foo: () = *last.unwrap_or(&-1);
    // let foo: () = last.unwrap_or(&-1);
}
