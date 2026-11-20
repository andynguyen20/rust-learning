use std::io;

fn main() {
    let mut board = [[' '; 3]; 3];
    let win_array: [[usize; 3]; 8] = [[2, 4, 6], [0, 4, 8], [3, 4, 5], [1, 4, 7], [0, 3, 6], [0, 1, 2], [2, 5, 8], [6, 7, 8]];
    let mut player = 'X';
    loop {
        if is_full(&board) {
            print_board(&board);
            println!("Tie game");
            break;
        }
        print_board(&board);
        println!("Player {player}, choose a cell (1-9): ");
        let mut player_move = String::new();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        let mut square: usize = player_move
            .trim()
            .parse()
            .expect("Invalid integer");

        square -= 1;

        if !place(&mut board, player, square) {
            continue;
        }

        if check_winner(&board, player, &win_array) == player {
            print_board(&board);
            println!("Player {player} wins");
            break;
        }

        player = if player == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    for i in 0..=2 {
        for j in 0..=2 {
            if j != 2 {
                print!(" {} |", board[i][j]);
            } else {
                println!(" {} ", board[i][j]);
                if i != 2 {
                    println!("---+---+---");
                }
            }
        }
    }
}

fn is_full(board: &[[char; 3]; 3]) -> bool {
    for i in 0..=2 {
        for j in 0..=2 {
            if board[i][j] == ' ' {
                return false;
            }        
        }
    }
    true
}

fn place(board: &mut[[char; 3]; 3], player: char, square: usize) -> bool {
    let row = square / 3;
    let column = square % 3;
    match player {
        'X' => {
            if !free_square(&board, square) {
                println!("place: invalid square, please choose a free square");
                return false;
            }
            board[row][column] = 'X';
            true
        }
        'O' => {
            if !free_square(&board, square) {
                println!("place: invalid square, please choose a free square");
                return false;
            }
            board[row][column] = 'O';
            true
        }
        _ => {
            panic!("place: invalid player");
        }
    }
}

fn free_square(board: &[[char; 3]; 3], square: usize) -> bool {
    let row = square / 3;
    let column = square % 3;
    if board[row][column] == ' ' {
        true
    } else {
        false
    }
}

fn check_winner(board: &[[char; 3]; 3], player: char, win_array: &[[usize; 3]; 8]) -> char {
    match player {
        'X' => {
            if check_winner_array(&board, player, &win_array) {
                return 'X';
            }
            ' '
        }
        'O' => {
            if check_winner_array(&board, player, &win_array) {
                return 'O';
            }
            ' '
        }
        _ => {
            panic!("check_winner: invalid player: {player}");
        }
    }
}

fn check_winner_array(board: &[[char; 3]; 3], player: char, win_array: &[[usize; 3]; 8]) -> bool {
    for i in 0..=7 {
        let mut count = 0;
        for j in 0..=2 {
            if check_player_square(&board, win_array[i][j], player) {
                count += 1;
            }
        }
        if count == 3 {
            return true;
        }
    }
    false
}

fn check_player_square(board: &[[char; 3]; 3], square: usize, player: char) -> bool {
    let row = square / 3;
    let column = square % 3;
    match player {
        'X' => {
            if board[row][column] == 'X' {
                return true;
            }
            false
        }
        'O' => {
            if board[row][column] == 'O' {
                return true;
            }
            false
        }
        _ => {
            panic!("check_player_square: invalid player: {player}");
        }
    }
}