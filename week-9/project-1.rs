use std::fs::File;
use std::io::Write;

fn main() {
   let lager = [
   "33 Export","Desperados","Goldberg","Gulder","Heineken","Star",];
   
   let stout = ["Legend","Turbo King","Williams"];
   
   let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz",];
   
   let mut file = File::create("breweries.txt").expect("create failed");
   file.write_all("LAGER\n".as_bytes()).expect("write failed");
    for drink in lager.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
    file.write_all("STOUT\n".as_bytes()).expect("write failed");
    for drink in stout.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
    file.write_all("NON-ALCOHOLIC\n".as_bytes()).expect("write failed");
for drink in non_alcoholic.iter() {
    file.write_all(drink.as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");
}
}