use std::io;

fn main() {
    let mut buffer = String::new();
    loop {
        println!("Text Buffer Editor - type 'help' for commands");

        let mut input = String::new();
        let mut arg: &str = "";

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let command = read_first_word(&input);
        
        match command {
            "append" => {
                println!("append");
                let first_space = find_space_index(&input, 1);
                if first_space == 0 {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let append_word = read_next_word(&input[first_space+1..]);
                buffer.push_str(append_word);
            }
            "show" => {
                println!("{buffer}");
            }
            "words" => {
                print_buffer_words(&buffer);
            }
            "slice" => {
                println!("slice");
                let first_space = find_space_index(&input, 1);
                if first_space == 0 {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let second_space = find_space_index(&input, 2);
                if second_space == 0 {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let first_index: usize = input[first_space+1..second_space].trim().parse().expect("Invalid slice command");
                if first_index < 0 || first_index > buffer.chars().count() {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let second_index: usize = input[second_space+1..].trim().parse().expect("Invalid slice command");
                if second_index < 0 || second_index > buffer.chars().count() {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                println!("{}", &buffer[first_index..second_index]);
            }
            "replace" => {
                println!("replace");
                let first_space = find_space_index(&input, 1);
                if first_space == 0 {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let second_space = find_space_index(&input, 2);
                if second_space == 0 {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let delete_word = read_first_word(&input[first_space+1..second_space+1]);
                let replace_word = read_next_word(&input[second_space+1..]);
                let mut temp = String::new();
                let (start_slice, end_slice) = find_word(&buffer, delete_word);
                if start_slice == 0 && end_slice == 0 {
                    println!("Error: Invalid indices or word not found");
                    continue;
                }
                temp.push_str(&buffer[..start_slice]);
                temp.push_str(replace_word);
                temp.push_str(&buffer[end_slice..]);
                buffer = temp.to_string();
            }
            "delete" => {
                println!("delete");
                let first_space = find_space_index(&input, 1);
                if first_space == 0 {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let second_space = find_space_index(&input, 2);
                if second_space == 0 {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let first_index: usize = input[first_space+1..second_space].trim().parse().expect("Invalid slice command");
                if first_index < 0 || first_index > buffer.chars().count() {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let second_index: usize = input[second_space+1..].trim().parse().expect("Invalid slice command");
                if second_index < 0 || second_index > buffer.chars().count() {
                    println!("Error: Make sure to input the command properly!");
                    continue;
                }
                let mut temp = String::new();
                temp.push_str(&buffer[..first_index]);
                temp.push_str(&buffer[second_index..]);
                buffer = temp.to_string();
            }
            "clear" => {
                buffer = String::new();
            }
            "help" => {
                print_help();
            }
            "quit" => {
                break;
            }
            _ => {
                println!("other");
            }
        }
    }
}

fn read_first_word(input: &str) -> &str {
    let input_bytes = input.as_bytes();

    for (index, &elements) in input_bytes.iter().enumerate() {
        if elements == b' ' || elements == b'\r' || elements == b'\n' {
            return &input[0..index];
        }
    }
    ""
}

fn read_next_word(input: &str) -> &str {
    let input_bytes = input.as_bytes();

    for (index, &elements) in input_bytes.iter().enumerate() {
        if elements == b'\r' || elements == b'\n' {
            return &input[0..index];
        }
    }
    ""
}

fn find_space_index(input: &str, space_count: i32) -> usize {
    let input_bytes = input.as_bytes();
    let mut count = 1;

    for (index, &elements) in input_bytes.iter().enumerate() {
        if elements == b' ' && count == space_count {
            return index;
        }
        if elements == b' ' && count != space_count {
            count += 1;
        }
    }
    0
}

fn print_buffer_words(input: &str) {
    let input_bytes = input.as_bytes();

    let mut count = 1;
    print!("{count}. ");
    for (index, &elements) in input_bytes.iter().enumerate() {
        if elements == b' '{
            count += 1;
            println!("");
            print!("{count}. ");
        } else {
            print!("{}", elements as char);
        }
    }
    println!("");
}

fn print_help() {
    println!("==================== TEXT BUFFER EDITOR ====================");
    println!("");
    println!("Usage: ");
    println!("  append <text>          Append text to the buffer");
    println!("  show                   Display the current buffer");
    println!("  words                  Display each word on a numbered line");
    println!("  slice <start> <end>    Display a substring by byte range");
    println!("  replace <old> <new>    Replace the first occurrence of text");
    println!("  delete <start> <end>   Delete text within a byte range");
    println!("  clear                  Empty the buffer");
    println!("  help                   Display available commands");
    println!("  quit                   Exit the program");
    println!("");
}

fn find_word(input: &str, word: &str) -> (usize, usize) {
    let input_bytes = input.as_bytes();
    let word_bytes = word.as_bytes();


    for (index, &elements) in input_bytes.iter().enumerate() {
        if ((index + word_bytes.len()) <= input_bytes.len()) && (&input[index..index + word.len()] == word) {
            return (index, index + word.len());
        }
    }
    (0, 0)
}