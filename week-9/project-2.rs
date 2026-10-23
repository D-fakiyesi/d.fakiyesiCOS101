use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
    ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
    ("Adams Aliyu", "ECO10110101", "Economics", 100),
    ("Shania Bolade", "CSC10328828", "Computer", 200),
    ("Adekunle Gold", "EEE11020202", "Electrical", 200),
    ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),];
    let mut file = File::create("students.txt").expect("create failed");
    for student in students.iter() {
    println!("{} | {} | {} | {}", student.0, student.1, student.2, student.3);
    file.write_all(student.0.as_bytes()).expect("write failed");
    file.write_all(" | ".as_bytes()).expect("write failed");
    file.write_all(student.1.as_bytes()).expect("write failed");
    file.write_all(" | ".as_bytes()).expect("write failed");
    file.write_all(student.2.as_bytes()).expect("write failed");
    file.write_all(" | ".as_bytes()).expect("write failed");
    file.write_all(student.3.to_string().as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");
}
}