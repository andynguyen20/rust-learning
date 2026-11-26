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
        Item {
            id,
            product,
            price,
            stock,
        }
    }
}

struct VendingMachine {
    stock: [Item; 5],
}

impl VendingMachine {
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
    let chips = Item::new(01, Product::Chips, 150, 5);
    let soda = Item::new(02, Product::Soda, 200, 3);
    let water = Item::new(03, Product::Water, 100, 0);
    let chocolate = Item::new(04, Product::Chocolate, 175, 4);
    let candy = Item::new(05, Product::Candy, 125, 6);
    println!("{:#?}", chips);
    println!("{:#?}", soda);
    println!("{:#?}", water);
    println!("{:#?}", chocolate);
    println!("{:#?}", candy);
}
