use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter a number: ");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let A: f32 = a.trim().parse().expect("Not a valid number");

    println!("Enter another number: ");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let B: f32 = b.trim().parse().expect("Not a valid number");

    println!("Enter your final number: ");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let C: f32 = c.trim().parse().expect("Not a valid number");

    let d = B * B - 4.0 * A * C;

    if d > 0.0 {
        println!("Two distinct roots");
    } else if d == 0.0 {
        println!("One real solution");
    } else {
        println!("Imaginary roots");
    }
}

