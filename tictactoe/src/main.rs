use console::Term;

pub struct Game {
    running: bool,
    board: [[char; 3]; 3],
    cursor: [usize; 2],
    turn: char,
}

impl Game {
    pub fn new() -> Self {
        Self {
            running: true,
            board: [[' ', ' ', ' ', ], [' ', ' ', ' ', ], [' ', ' ', ' ', ]],
            cursor: [0, 0],
            turn: 'X',
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.print_board();
            self.input();
            if self.has_won() {
                self.print_board();
                if self.turn == 'X' {
                    self.turn = 'O';
                } else {
                    self.turn = 'X';
                }
                println!("{} has won!", self.turn);
                self.running = false;
            }
        }
    }

    fn print_board(&mut self) {
        print!("\x1B[2J\x1B[1;1H"); // Clears the screen.... !!!
        for (row_index, row) in self.board.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                if row_index == self.cursor[0] as usize && col_index == self.cursor[1] as usize {
                    print!("[");
                } else {
                    print!(" ");
                }

                print!("{col}");

                if row_index == self.cursor[0] as usize && col_index == self.cursor[1] as usize {
                    print!("]");
                } else {
                    print!(" ");
                }

                if col_index < 2 {
                    print!("|")
                }
            }
            println!();
        }
    }

    fn has_won(&mut self) -> bool {
        for index in 0..3 {
            if self.board[index][1] != ' ' &&
                self.board[index][0] == self.board[index][1] &&
                self.board[index][1] == self.board[index][2] { // Vertical
                return true;
            }
            if self.board[1][index] != ' ' &&
                self.board[0][index] == self.board[1][index] &&
                self.board[1][index] == self.board[2][index] { // Horizontal
                return true;
            }
            if self.board[1][1] != ' ' &&
                ((self.board[0][0] == self.board[1][1] &&
                    self.board[1][1] == self.board[2][2]) ||
                    self.board[0][2] == self.board[1][1] &&
                        self.board[1][1] == self.board[2][0]) {
                return true;
            }
        }
        return false;
    }

    fn enter(&mut self) {
        if self.board[self.cursor[0]][self.cursor[1]] != ' ' {
            return;
        }

        self.board[self.cursor[0]][self.cursor[1]] = self.turn;
        if self.turn == 'X' {
            self.turn = 'O';
        } else {
            self.turn = 'X';
        }
    }

    fn input(&mut self) {
        use console::Term;
        let term = Term::stdout();
        let user_input = term.read_char().unwrap();
        if user_input == 'q' {
            return self.running = false;
        } else if user_input == 'w' {
            if self.cursor[0] > 0 {
                self.cursor[0] -= 1;
            }
        } else if user_input == 's' {
            if self.cursor[0] < 3 {
                self.cursor[0] += 1;
            }
        } else if user_input == 'a' {
            if self.cursor[1] > 0 {
                self.cursor[1] -= 1;
            }
        } else if user_input == 'd' {
            if self.cursor[1] < 3 {
                self.cursor[1] += 1;
            }
        } else if user_input == '\n' {
            self.enter();
        } else {
            println!("{:?}", user_input);
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}