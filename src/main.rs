use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Filled(Player),
}

struct Game {
    board: [[Cell; 3]; 3],
    current_player: Player,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
        }
    }

    fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Empty => print!(" . "),
                    Cell::Filled(player) => match player {
                        Player::X => print!(" X "),
                        Player::O => print!(" O "),
                    },
                }
            }
            println!();
        }
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if row >= 3 || col >= 3 {
            return Err("Fora dos limites!");
        }
        if self.board[row][col] != Cell::Empty {
            return Err("Célula já preenchida!");
        }
        self.board[row][col] = Cell::Filled(self.current_player);
        Ok(())
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn check_winner(&self) -> Option<Player> {
        for row in self.board.iter() {
            if row[0] == row[1] && row[1] == row[2] {
                if let Cell::Filled(player) = row[0] {
                    return Some(player);
                }
            }
        }

        for col in 0..3 {
            if self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col] {
                if let Cell::Filled(player) = self.board[0][col] {
                    return Some(player);
                }
            }
        }

        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            if let Cell::Filled(player) = self.board[0][0] {
                return Some(player);
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            if let Cell::Filled(player) = self.board[0][2] {
                return Some(player);
            }
        }

        None
    }

    fn is_board_full(&self) -> bool {
        for row in self.board.iter() {
            for cell in row.iter() {
                if *cell == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    fn play(&mut self) {
        loop {
            self.print_board();
            println!("Jogador {:?}, faça sua jogada (linha e coluna): ", self.current_player);
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
            let input: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            
            if input.len() != 2 {
                println!("Entrada inválida! Por favor, insira dois números separados por espaço.");
                continue;
            }
            
            let (row, col) = (input[0], input[1]);

            match self.make_move(row, col) {
                Ok(_) => {
                    if let Some(winner) = self.check_winner() {
                        self.print_board();
                        println!("Jogador {:?} venceu!", winner);
                        break;
                    }
                    if self.is_board_full() {
                        self.print_board();
                        println!("Empate!");
                        break;
                    }
                    self.switch_player();
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
