//variables
fn main() {
    let mut x = 10; //mut allows variable to be mutable
    println!("x is {}", x);

    x = 20;
    println!("x is {}", x);

    let mut x: u8 = 245;
    x = x + 1; 
    println!("x is {}", x);

    let x: f32 = 10.09;
    println!("x is {}", x);
}