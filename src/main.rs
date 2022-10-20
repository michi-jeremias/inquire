fn main() {

    let mut x = 42;
    let x_ref: &mut i32 = &mut x;
    // x = 134;
    println!("x_ref = {}", x_ref);
}
