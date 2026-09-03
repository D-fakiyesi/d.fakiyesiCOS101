fn main(){
    let p:f64 = 520_000_000;
    let r:f64 = 0.1;
    let t:f64 = 5.0;
    let a = p * (1.0 + r).powf(t);
    // compound interest formula
    let ci = a - p;
    println!("Compound Interest is {}", ci);
}