use std::io;

fn main() {
    let mut buffer = String::new();
    loop {
        println!("Text Buffer Editor - type 'help' for commands");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let command = read_first_word(&input);
        
        if command == " " {
            println!("Invalid command - type 'help' for commands");
            continue;
        } 

        println!("I made it here");
        break;
    }
}

fn read_first_word(input: &str) -> &str {
    let input_bytes = input.as_bytes();

    for (index, &elements) in input_bytes.iter().enumerate() {
        if elements == b' ' {
            return &input[0..index];
        }
    }
    " "
}
