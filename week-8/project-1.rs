use std::io;

fn main() {
    let aps_table = vec![
        ("APS 1-2", "Intern", "-", "Paralegal", "Placement"),
        ("APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"),
        ("APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"),
        ("EL1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        ("EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"),
        ("SES", "CEO", "Dean", "Partner", "Principal"),
    ];

    println!("APS Level Checker");
    println!("------------------");

    println!("Enter your profession (Public Servant, Office Admin, Academic, Lawyer, Teacher):");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).unwrap();
    let profession = profession.trim();

    println!("Enter your years of experience:");
    let mut years_input = String::new();
    io::stdin().read_line(&mut years_input).unwrap();
    let years: u32 = years_input.trim().parse().unwrap_or(0);

    let result = check_aps_level(&aps_table[..], profession, years);

    println!("Your APS Level is: {}", result);
}

fn check_aps_level(table: &[(&str, &str, &str, &str, &str)], profession: &str, years: u32) -> &'static str {
    for row in table {
        let aps = row.0;
        let ps = row.1;
        let admin = row.2;
        let academic = row.3;
        let lawyer = row.4;
        let teacher = row.4;

        let role = if profession == "Public Servant" {
            ps
        } else if profession == "Office Admin" {
            admin
        } else if profession == "Academic" {
            academic
        } else if profession == "Lawyer" {
            lawyer
        } else if profession == "Teacher" {
            teacher
        } else {
            "-"
        };

        if role == "-" {
            continue;
        }

        if aps.contains('-') {
            let parts: Vec<&str> = aps.split(' ').collect();
            let range = parts[1];

            let nums: Vec<&str> = range.split('-').collect();
            let low: u32 = nums[0].parse().unwrap_or(0);
            let high: u32 = nums[1].parse().unwrap_or(0);

            if years >= low && years <= high {
                return aps;
            }
        }

        if aps == "SES" && years >= 14 {
            return "SES";
        }
    }

    "Not Found"
}
