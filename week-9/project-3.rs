use std::fs::File;
use std::io::Write;

fn main() {
    let ministers = vec![ "Aigbogun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etieye",
    ];
    let ministries = vec!["Internal Affairs",
    "Justice",
    "Defense",
    "Power & Steel",
    "Petroleum",
    ];
    let zones = vec!["South West",
    "North East",
    "South South",
    "South West",
    "South East",];
    let mut file = File::create("efcc.txt").expect("create failed");
    for i in 0..ministers.len() {
        file.write_all(ministers[i].as_bytes()).expect("write failed");
        file.write_all(" | ".as_bytes()).expect("write failed");
        file.write_all(ministries[i].as_bytes()).expect("write failed");
        file.write_all(" | ".as_bytes()).expect("write failed");
        file.write_all(zones[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
    
}