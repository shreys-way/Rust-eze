//variables
fn main() {
    let mut x = 10; //mut allows variable to be mutable
    println!("x is {}", x);

    x = 20;
    println!("x is {}", x);

    let mut x: u8 = 255;
    // x = x + 1 won't work
    println!("x is {}", x);


}