enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

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
        self.new_num();
        while self.running {
            self.print_board();
            self.input();
        }
    }

    fn print_board(&mut self) {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen???
        for row in self.board {
            for col in row {
                if col != 0 { print!("\x1b[38;5;16m"); }
                if col == 2 { print!("\x1b[48;5;213m"); }
                if col == 4 { print!("\x1b[48;5;176m"); }
                if col == 8 { print!("\x1b[48;5;139m"); }
                if col == 16 { print!("\x1b[48;5;102m"); }
                if col == 32 { print!("\x1b[48;5;65m"); }
                if col == 64 { print!("\x1b[48;5;28m"); }
                if col == 128 { print!("\x1b[48;5;22m"); }
                if col == 256 { print!("\x1b[48;5;196m"); }
                if col == 512 { print!("\x1b[48;5;160m"); }
                if col == 1024 { print!("\x1b[48;5;124m"); }
                if col == 2048 { print!("\x1b[48;5;88m"); }
                if col >= 4096 { print!("\x1b[48;5;52m"); }
                print!("{col}\t");
                print!("\x1b[0m");
            }
            println!();
        }
    }

    fn new_num(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let new_location = [rng.gen_range(0..4), rng.gen_range(0..4)];

            if self.board[new_location[0]][new_location[1]] == 0 {
                self.board[new_location[0]][new_location[1]] = rng.gen_range(1..3) * 2;
                break;
            }
        }
    }

    fn move_board(&mut self, direction: Direction) {
        let mut combined = [false, false, false, false];
        let mut moved = false;

        if matches!(direction, Direction::LEFT) {
            // Move
            for _i in 0..2 {
                for row in 0..4 {
                    for col in (1..4).rev() {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row][col - 1] != 0 {
                            continue;
                        }

                        self.board[row][col - 1] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
            // Combine
            for row in 0..4 {
                for col in (1..4).rev() {
                    if combined[row] {
                        combined[row] = false;
                        continue;
                    }
                    if self.board[row][col] == 0 {
                        continue;
                    }
                    if self.board[row][col] != self.board[row][col - 1] {
                        continue;
                    }

                    self.board[row][col - 1] *= 2;
                    self.board[row][col] = 0;
                    combined[row] = true;
                    moved = true;
                }
            }
            // Move
            for _i in 0..2 {
                for row in 0..4 {
                    for col in (1..4).rev() {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row][col - 1] != 0 {
                            continue;
                        }

                        self.board[row][col - 1] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
        } else if matches!(direction, Direction::RIGHT) {
            // Move
            for _i in 0..2 {
                for row in 0..4 {
                    for col in 0..3 {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row][col + 1] != 0 {
                            continue;
                        }

                        self.board[row][col + 1] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
            // Combine
            for row in 0..4 {
                for col in 0..3 {
                    if combined[row] {
                        combined[row] = false;
                        continue;
                    }
                    if self.board[row][col] == 0 {
                        continue;
                    }
                    if self.board[row][col] != self.board[row][col + 1] {
                        continue;
                    }

                    self.board[row][col + 1] *= 2;
                    self.board[row][col] = 0;
                    combined[row] = true;
                    moved = true;
                }
            }
            // Move
            for _i in 0..2 {
                for row in 0..4 {
                    for col in 0..3 {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row][col + 1] != 0 {
                            continue;
                        }

                        self.board[row][col + 1] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
        } else if matches!(direction, Direction::UP) {
            // Move
            for _i in 0..2 {
                for row in (1..4).rev() {
                    for col in 0..4 {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row - 1][col] != 0 {
                            continue;
                        }

                        self.board[row - 1][col] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
            // Combine
            for row in (1..4).rev() {
                for col in 0..4 {
                    // Combine
                    if combined[col] {
                        combined[col] = false;
                        continue;
                    }
                    if self.board[row][col] == 0 {
                        continue;
                    }
                    if self.board[row][col] != self.board[row - 1][col] {
                        continue;
                    }

                    self.board[row - 1][col] *= 2;
                    self.board[row][col] = 0;
                    combined[col] = true;
                    moved = true;
                }
            }
            // Move
            for _i in 0..2 {
                for row in (1..4).rev() {
                    for col in 0..4 {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row - 1][col] != 0 {
                            continue;
                        }

                        self.board[row - 1][col] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
        } else if matches!(direction, Direction::DOWN) {
            // Move
            for _i in 0..2 {
                for row in 0..3 {
                    for col in 0..4 {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row + 1][col] != 0 {
                            continue;
                        }

                        self.board[row + 1][col] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
            // Combine
            for row in 0..3 {
                for col in 0..4 {
                    // Combine
                    if combined[col] {
                        combined[col] = false;
                        continue;
                    }
                    if self.board[row][col] == 0 {
                        continue;
                    }
                    if self.board[row][col] != self.board[row + 1][col] {
                        continue;
                    }

                    self.board[row + 1][col] *= 2;
                    self.board[row][col] = 0;
                    combined[col] = true;
                    moved = true;
                }
            }
            // Move
            for _i in 0..2 {
                for row in 0..3 {
                    for col in 0..4 {
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row + 1][col] != 0 {
                            continue;
                        }

                        self.board[row + 1][col] = self.board[row][col];
                        self.board[row][col] = 0;
                        moved = true;
                    }
                }
            }
        }
        if moved {
            self.new_num();
        }
    }

    fn input(&mut self) {
        use console::Term;
        let term = Term::stdout();
        let user_input = term.read_char().unwrap();
        if user_input == 'q' {
            return self.running = false;
        } else if user_input == 'w' {
            self.move_board(Direction::UP)
        } else if user_input == 's' {
            self.move_board(Direction::DOWN)
        } else if user_input == 'a' {
            self.move_board(Direction::LEFT)
        } else if user_input == 'd' {
            self.move_board(Direction::RIGHT)
        }
    }
}


fn main() {
    let mut game = Game::new();
    game.run();
}
