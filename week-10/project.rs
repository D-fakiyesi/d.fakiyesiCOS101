struct Laptop {
    brand: String,
    price: u32,
    quantity_available: u32,
    quantity_purchased: u32,
}

impl Laptop {
    fn cost(&self)->u32 {
        self.quantity_purchased * self.price
    }

    fn display(&self) {
        println!(
            "Brand: {}\nPrice: {}\nCurrent Stock: {}\nQuantity Purchased: {}\nCost: {}",
            self.brand,
            self.price,
            self.quantity_available,
            self.quantity_purchased,
            self.cost()
        );
    }
}

fn main() {
    let laptop1 = Laptop {
        brand:String::from("HP"),
        price: 650000,
        quantity_available: 10,
        quantity_purchased: 3,
    };
    let laptop2 = Laptop {
        brand:String::from("IBM"),
        price: 755000,
        quantity_available: 6,
        quantity_purchased: 3,
    };
    let laptop3 = Laptop {
        brand:String::from("Toshiba"),
        price: 550000,
        quantity_available: 10,
        quantity_purchased: 3,
    };
    let laptop4 = Laptop {
        brand:String::from("Dell"),
        price: 850000,
        quantity_available: 4,
        quantity_purchased: 3
    };
    let total = laptop1.cost()
    + laptop2.cost()
    + laptop3.cost()
    + laptop4.cost();
    
    laptop1.display();
    laptop2.display();
    laptop3.display();
    laptop4.display();
    
    println!("Grand Total: {}", total);
}