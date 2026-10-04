fn main(){
    println!(" RESTAURANT MENU ");
    println!("P - Poundo Yam / Edikangiko Soup   -3200");
    println!("F - Fried Rice / Chicken  -3000");
    println!("A - Amala / Ewedu Soup   -2500");
    println!("E - Eba / Egusi Soup   -2000");
    println!("W - White Rice / Stew   -2500");

    let mut food = String::new();
    println!("Enter your food choice: ");
    std::io::stdin().read_line(&mut food).expect("Failed to read line");

    let food = food.trim();

    let price;

    if food == "P" {
        price = 3200;
    } else if food == "F" {
        price = 3000;
    } else if food == "A" {
        price = 2500;
    } else if food == "E" {
        price = 2000;
    } else if food == "W" {
        price = 2500;
    } else {
        println!("Invalid food choice");
        return;
    }

    let mut quantity = String::new();

    println!("Enter quantity: ");
    std::io::stdin().read_line(&mut quantity).expect("Failed to read line");

    let quantity: f64 = quantity.trim().parse().expect("Please enter a valid number");

    let total = price * quantity;

    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;

        println!("Discount: {}" , discount);
        println!("Final Total: {}", final_total);
    } else {
        println!("No discount.");
        println!("Final Total: {}", total);
    }
    }