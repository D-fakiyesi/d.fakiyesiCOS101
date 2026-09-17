use std::io;
fn main() {
    let mut age = String::new();
    ;let mut exp = String::new();
    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let Age:i32 =age.trim().parse().expect("Not a valid number");

    println!("How long have you been in the field?: ");
    io::stdin().read_line(&mut exp).expect("Not a valid string");
    let Experience:i32 =exp.trim().parse().expect("Not a valid number");

    if Age >= 40 && Experience >= 10{
        println!("Experienced");
        println!("Your incentive is 1_560_000");
    }else if Age >= 30 && Experience >= 10{
            println!("Experienced");
            println!("Your incentive is 1_450_000");
        }else if Age <= 28 && Age >= 22 && Experience >= 5{
            println!("Experienced");
            println!("Your incentive is 1_300_000");
        }else{
            println!("Not experienced");
            println!("Your incentive is 100_000");
        }
    }
