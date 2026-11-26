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
    let chips = create_item(01, Product::Chips, 150, 5);
    let soda = create_item(02, Product::Soda, 200, 3);
    let water = create_item(03, Product::Water, 100, 0);
    let chocolate = create_item(04, Product::Chocolate, 175, 4);
    let candy = create_item(05, Product::Candy, 125, 6);
    println!("{:#?}", chips);
    println!("{:#?}", soda);
    println!("{:#?}", water);
    println!("{:#?}", chocolate);
    println!("{:#?}", candy);
}

fn create_item(id: i32, product: Product, price: i32, stock: i32) -> Item {
    Item {
        id,
        product,
        price,
        stock,
    }
}