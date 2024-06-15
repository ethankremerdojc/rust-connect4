use std::io;

const CELLS_WIDTH: usize = 7;
const CELLS_HEIGHT: usize = 6;

#[derive(Default, Debug, PartialEq)]
enum CellState {
  #[default]
  Empty,
  Red,
  Black
}

#[derive(Default, Debug)]
enum Player {
    #[default]
    PlayerOne,
    PlayerTwo
}

impl Player {
    fn color(&self) -> CellState {
        match self {
            Player::PlayerOne => { CellState::Red },
            Player::PlayerTwo => { CellState::Black }
        }
    }
}

#[derive(Default, Debug)]
struct Board {
  cells: [[CellState; CELLS_WIDTH]; CELLS_HEIGHT],
  current_player: Player
}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize
}

#[derive(Debug)]
enum Direction {
    Up,
    UpLeft,
    UpRight,
    Right
}

impl Direction {
    fn all_directions() -> [Direction; 4] {
        [Direction::Up, Direction::UpLeft, Direction::UpRight, Direction::Right]
    }

    fn can_be_applied_to(&self, pos:Position) -> bool {
        match self {
            Direction::Up => {
                if pos.y == CELLS_HEIGHT - 1 {
                    false
                } else {
                    true
                }
            },
            Direction::UpLeft => {
                if pos.y == CELLS_HEIGHT - 1 {
                    false
                } else if pos.x == 0 {
                    false
                } else {
                    true
                }
            },
            Direction::UpRight => {
                if pos.y == CELLS_HEIGHT - 1 {
                    false
                } else if pos.x == CELLS_WIDTH - 1 {
                    false
                } else {
                    true
                }
            },
            Direction::Right => {
                if pos.x == CELLS_WIDTH - 1 {
                    false
                } else {
                    true
                }
            }
        }
    }

    fn apply_to(&self, pos: Position) -> Position {
        match self {
            Direction::Up => {
                Position{
                    x: pos.x,
                    y: pos.y + 1
                }
            },
            Direction::UpLeft => {
                Position{
                    x: pos.x - 1,
                    y: pos.y + 1
                }
            },
            Direction::UpRight => {
                Position{
                    x: pos.x + 1,
                    y: pos.y + 1
                }
            },
            Direction::Right => {
                Position{
                    x: pos.x + 1,
                    y: pos.y
                }
            }
        }
    }
}

impl Board {

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::PlayerOne => Player::PlayerTwo,
            Player::PlayerTwo => Player::PlayerOne
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, piece: CellState) {
        self.cells[row][col] = piece;
    }

    fn check_for_win(&self) -> bool { 
        let mut longest_chain: usize = 0;

        for (row_index, row) in self.cells.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {

                if cell == &CellState::Empty { continue; };

                let temp_longest_chain: usize = self.get_greatest_chain_from_cell(row_index, col_index);
                if temp_longest_chain > longest_chain {
                    longest_chain = temp_longest_chain
                }
            }
        }

        // let current_color: CellState = self.current_player.color();
        // println!("Longest Chain Length For {current_color:?}: {longest_chain:?}");
        if longest_chain == 4 { true } else { false }
    }

    fn get_greatest_chain_from_cell(&self, row_index: usize, col_index: usize) -> usize {
        let directions = Direction::all_directions();
        let cell_position = Position { x: col_index, y: row_index};

        let current_cell = &self.cells[cell_position.y][cell_position.x];
        if current_cell == &CellState::Empty { return 0 };
        if current_cell != &self.current_player.color() { return 0 };

        let mut longest_chain: usize = 1;

        for direction in directions {

            let mut temp_position: Position = Position{ x: col_index, y: row_index};
            let mut chain_length = 1;

            loop {
                if ! direction.can_be_applied_to(temp_position) { break };
                temp_position = direction.apply_to(temp_position);

                let new_cell = &self.cells[temp_position.y][temp_position.x];
                if new_cell != &self.current_player.color() { break };
                
                chain_length += 1;
            }

            if chain_length > longest_chain { longest_chain = chain_length }
        }

        longest_chain
    }

    fn render(&self) {
        for row in self.cells.iter().rev() {
            let mut row_contents: String = String::new();

            for cell in row {
                match cell {
                    CellState::Empty => { row_contents += "|   " },
                    CellState::Black => { row_contents += "| O " },
                    CellState::Red => { row_contents += "| @ " },
                }
            }

            row_contents += "|";
            println!("{row_contents}");
        }
    }

    fn check_if_valid_row(&self, &col_number: &usize) -> bool {
        for row in &self.cells {
            if row[col_number] == CellState::Empty { return true }
        }

        false
    }

    fn get_valid_user_input_number(&self) -> usize {
        let mut chosen_col: usize;

        loop {
            let user_input = get_user_input();
            match &*user_input { // instead of &*user_input you can do user_input.as_str() 
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" => {
                        chosen_col = user_input.parse().unwrap();
                        break;
                    },
                    _ => {
                        println!("Invalid selection, try again.");
                        continue;
                    }
            };
        }

        chosen_col -= 1;
        chosen_col
    }

    fn get_player_chosen_row(&self) -> usize {
        let mut chosen_col;

        loop {
            chosen_col = self.get_valid_user_input_number();
            let chosen_is_valid = self.check_if_valid_row(&chosen_col);

            if chosen_is_valid {
                break
            }
            
            println!("Invalid selection (Row is full.)")
        }

        chosen_col
    }

    fn place_piece(&mut self, col_number: usize, piece: CellState) {
        let mut placed: bool = false;
        
        for (index, row) in self.cells.iter().enumerate() {
            if row[col_number] == CellState::Empty {
                self.set_cell(index, col_number, piece);
                placed = true;
                break;
            }
        }

        assert!(placed);
    }
}

pub fn connect_4() {
    let mut board = Board::default();

    loop {
        board.render();
        let current_piece = board.current_player.color();
        println!("{current_piece:?}, choose a row number. (1-7)");

        let chosen_col: usize = board.get_player_chosen_row();
        board.place_piece(chosen_col, current_piece);

        let player_has_won: bool = board.check_for_win();

        if player_has_won {
            board.render();
            println!("{:?} wins!", board.current_player.color());
            break;
        }

        board.switch_player();
    }
}