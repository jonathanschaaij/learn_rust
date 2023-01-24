use std::io;

fn main() {
    println!("Starting Tic-Tac-Toe");

    let mut board: [i8; 9] = [0; 9]; // initialize an empty board with 9 zeros; 1 for player 1 and 2 for player 2
    let mut turn = 1;
    while check_winner(&board) == 0 {
        print_board(board);
        board[ask_move()] = turn;
        turn = if turn == 1 { 2 } else { 1 };
    }

    // Game has ended
    print_board(board);
    if check_winner(&board) == 0 {
        println!("It's a tie!");
    } else {
        println!("The winner is Player {}", check_winner(&board));
    }
}

fn ask_move() -> usize {
    println!("Where do you want to place your next piece");

    let mut userinput = String::new();
    io::stdin()
        .read_line(&mut userinput)
        .expect("Failed to read line");
    let board_index: usize = userinput
        .trim()
        .parse()
        .expect("Please pick a number between one and 9");
    board_index
}

fn check_winner(board: &[i8; 9]) -> i8 {
    // Functions returns the winner based on the state of a Tic-Tac-Toe board
    // -1 is a tie
    // 0 means game is not over/ no winner,
    // 1 and 2 means the respective palyer has won

    // All possible three-in-a-rows
    let win_options = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    let mut winner = -1; // by default assume a tie;

    for win_option in win_options {
        let first = board[win_option[0]];
        if winner < 0 && first == 0 {
            winner = 0;
            continue;
        }
        if board[win_option[1]] == 0 {
            winner = 0;
            continue;
        } else if board[win_option[1]] != first {
            continue;
        }
        if board[win_option[2]] == 0 {
            winner = 0;
            continue;
        } else if board[win_option[2]] != first {
            continue;
        }
        winner = first;
        break;
    }
    winner
}

fn print_board(board: [i8; 9]) {
    println!("\n{}|{}|{}", board[0], board[1], board[2]);
    println!("------");
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("------");
    println!("{}|{}|{}\n", board[6], board[7], board[8]);
}
