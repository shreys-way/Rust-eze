//variables
fn main() {
    let mut x = 10; //mut allows variable to be mutable
    println!("x is {}", x);

    x = 20;
    println!("x is {}", x);

    let mut x: u8 = 245;
    x = x + 1; 
    println!("x is {}", x);

    let x: f32 = 10.2132132132109; //floats
    println!("x is {}", x);

    let x: f64 = 10.2132132132109; //floats
    println!("x is {}", x);


    // arithmetic operations 
    let a = 10;
    let b = 3.3;
    // can't divide two different data types so casting is used
    println!("a/b is: {}", a as f64 /b); 

    //print statements
    print!("a/b is: {:08.3}\na is {}", a as f64 /b, a as f64); 
    println!("a/b is: {:08.3}\na is {}", a as f64 /b, a as f64); 
    println!("a is {0} \nb is {1}\na again is {0}", a, b);


}