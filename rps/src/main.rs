fn input(text: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{text}: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}

fn get_computer_move() -> &'static str {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..3);
    let moves = ["r", "p", "s"];

    return moves[random_num];
}

fn main() {
    let mut running = true;
    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;

    while running {
        let mut user_move;
        loop {
            user_move = input("Enter your move");

            if user_move == "r" || user_move == "p" || user_move == "s" {
                break;
            }
            println!("{user_move} is not a valid input");
        }
        let computer_move = get_computer_move();

        if user_move == computer_move {
            println!("You have tied. :|");
            ties += 1;
        } else if (user_move == "r" && computer_move == "s") ||
            (user_move == "p" && computer_move == "r") ||
            (user_move == "s" && computer_move == "p") {
            wins += 1;
            println!("You have won. :)");
        } else if (computer_move == "r" && user_move == "s") ||
            (computer_move == "p" && user_move == "r") ||
            (computer_move == "s" && user_move == "p") {
            println!("You have lost. :(");
            losses += 1;
        }
        println!("Wins: {wins}");
        println!("Losses: {losses}");
        println!("Ties: {ties}");

        loop {
            let user_input = input("Would you like to play again? (y/n)");

            if user_input == "y" {
                break;
            }
            if user_input == "n" {
                running = false;
                break;
            }

            println!("{user_input} is not a valid input");
        }
    }
}