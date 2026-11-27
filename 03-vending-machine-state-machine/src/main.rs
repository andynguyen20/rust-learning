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
    fn show(&self) {
        let product = format!("{:?}", self.product);
        let price = format!("${}.{:02}", self.price / 100, self.price % 100);
        println!("{:2}  {:<12}  {:<7}  {:<5}", self.id, product, price, self.stock);
    }
} 


#[derive(Debug)]
struct VendingMachine {
    stock: [Item; 5],
    state: MachineState,
    current_item: Option<Item>,
}

impl VendingMachine {
    fn new() -> Self {
        Self {
            stock: [
                Item::new(1, Product::Chips, 150, 5),
                Item::new(2, Product::Soda, 200, 3),
                Item::new(3, Product::Water, 100, 0),
                Item::new(4, Product::Chocolate, 175, 4),
                Item::new(5, Product::Candy, 125, 6),
            ],
            state: MachineState::Idle,
            current_item: None,
        }
    }
    fn show(&self) {
        println!("{:<2}  {:<10}  {:>7}  {:>5}", "ID", "PRODUCT", "PRICE", "STOCK");
        for item in &self.stock {
            item.show();
        }
        println!("");
    }
    fn select_item(&self, id: i32) {
    }
}

fn main() {
    println!("=============================================");
    println!("              VENDING MACHINE");
    println!("=============================================");
    println!("Type 'help' for available commands.");
    println!("");
    let vending_machine = VendingMachine::new();
    vending_machine.show();
    /*loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        break;
    }*/
}
