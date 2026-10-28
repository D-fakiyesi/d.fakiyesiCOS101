fn display(v: Vec<i32>) -> Vec<i32> {
    println!("{:?}", v);
    v
}

fn main() {
    let v = vec![10, 20, 30];
    let v2 = display(v);
    println!("{:?}", v2);
}