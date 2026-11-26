use std::io;

#[derive(Debug)]
enum MachineState {
    Idle,
    AwaitingPayment,
    Dispensing,
    Maintenance,
}

#[derive(Debug)]
enum Product {
    Chips,
    Soda,
    Water,
    Chocolate,
    Candy,
}

#[derive(Debug)]
struct Item {
    id: i32,
    product: Product,
    price: i32,
    stock: i32,
}

impl Item {
    fn new(id: i32, product: Product, price: i32, stock: i32) -> Self {
        Self {
            id,
            product,
            price,
            stock,
        }
    }
}

#[derive(Debug)]
struct VendingMachine {
    stock: [Item; 5],
}

impl VendingMachine {
    fn new() -> Self {
        Self {
            stock: [
                Item::new(01, Product::Chips, 150, 5),
                Item::new(02, Product::Soda, 200, 3),
                Item::new(03, Product::Water, 100, 0),
                Item::new(04, Product::Chocolate, 175, 4),
                Item::new(05, Product::Candy, 125, 6),
            ]
        }
    }
}

fn main() {
    println!("=============================================");
    println!("              VENDING MACHINE");
    println!("=============================================");
    println!("Type 'help' for available commands.");
    println!("");
    /*loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        break;
    }*/
    let vending_machine = VendingMachine::new();
    println!("{:#?}", vending_machine);
}
