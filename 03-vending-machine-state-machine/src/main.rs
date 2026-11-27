use std::io;

#[derive(Debug)]
enum MachineState {
    Idle,
    AwaitingPayment {
        item_id: i32,
        balance: i32,
    },
    Dispensing {
        item_id: i32,
        balance: i32,
    },
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
        }
    }
    fn show(&self) {
        println!("{:<2}  {:<10}  {:>7}  {:>5}", "ID", "PRODUCT", "PRICE", "STOCK");
        for item in &self.stock {
            item.show();
        }
        println!("");
    }
    fn inspect(&self, mut id: usize) {
        id -= 1;
        println!("Product: {:?}", self.stock[id].product);
        println!("Price: ${}.{:02}", self.stock[id].price / 100, self.stock[id].price % 100);
        println!("Stock: {}", self.stock[id].stock);
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
    vending_machine.inspect(1);
    help();
    /*loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        break;
    }*/
}

fn help() {
    println!("");
    println!("================ AVAILABLE COMMANDS ================");
    println!("");
    println!("Customer Commands:");
    println!("  show                   Display all available products");
    println!("  inspect <id>           Display product details");
    println!("  select <id>            Select a product");
    println!("  insert <cents>         Insert a coin");
    println!("  status                 Display current machine status");
    println!("  dispense               Dispense the selected product");
    println!("  cancel                 Cancel purchase and refund balance");
    println!("");
    println!("Maintenance Commands:");
    println!("  maintenance            Enter maintenance mode");
    println!("  restock <id> <qty>     Restock a product");
    println!("  setprice <id> <cents>  Update a product's price");
    println!("  resume                 Exit maintenance mode");
    println!("");
    println!("General Commands:");
    println!("  help                   Display available commands");
    println!("  quit                   Exit the vending machine");
    println!("");
    println!("Accepted Coins:");
    println!("  5, 10, 25, 100, 200 cents");
    println!("");
    println!("====================================================");
}