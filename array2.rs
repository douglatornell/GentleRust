// array2.rs
fn sum(values: &[i32]) -> i32 {
    // &[i32] is pronounced "slice of i32"
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i];
    }
    res
}

fn main() {
    let arr = [10, 20, 30, 40];
    // passing arr by reference with &arr passes arr as a slice
    // & is pronounced "borrow"
    let res = sum(&arr);
    println!("sum {}", res);
}
