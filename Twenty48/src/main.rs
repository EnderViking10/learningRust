struct Game {
    running: bool,
    board: [[u32; 4]; 4],
}

impl Game {
    pub fn new() -> Self {
        Self {
            running: true,
            board: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], ],
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.print_board();
            self.running = false;
        }
    }

    fn print_board(&mut self) {
        for row in self.board {
            for col in row {
                print!("{col}\t");
            }
            println!();
        }
    }

    fn input(&mut self) {
        use console::Term;
        let term = Term::stdout();
        let user_input = term.read_char().unwrap();
        if user_input == 'q' {
            return self.running = false;
        } else if user_input == 'w' {
        } else if user_input == 's' {
        } else if user_input == 'a' {
        } else if user_input == 'd' {
        } else {
            println!("{:?}", user_input);
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
