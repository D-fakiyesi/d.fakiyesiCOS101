use std::io;

fn main() {
    let mut applicants: Vec<(&str, u32)> = Vec::new();

    println!("How many applicants are being interviewed?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap_or(0);

    for i in 0..count {
        println!("Enter applicant {} name:", i + 1);
        let mut name_input = String::new();
        io::stdin().read_line(&mut name_input).unwrap();
        let name = name_input.trim().to_string();

        println!("Enter years of programming experience for {}:", name);
        let mut years_input = String::new();
        io::stdin().read_line(&mut years_input).unwrap();
        let years: u32 = years_input.trim().parse().unwrap_or(0);

        applicants.push((Box::leak(name.into_boxed_str()), years));
    }

    let mut highest_years = 0;
    let mut best_candidate = "";

    for i in 0..applicants.len() {
        let person = applicants[i];
        if person.1 > highest_years {
            highest_years = person.1;
            best_candidate = person.0;
        }
    }

    println!("\nThe candidate with the highest experience is: {}", best_candidate);
    println!("Years of experience: {}", highest_years);
}
