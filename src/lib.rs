use std::fmt;


/* TODO
- implement pawn movement
- implement get_possible_moves


*/



#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White, Black
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Role {
    Pawn, Rook, Knight, Bishop, Queen, King
}

 #[derive(Copy, Clone, Debug)]
struct Piece {
    color: Color,
    role: Role,
    has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, role: Role) -> Piece {
        Piece {
            color: color,
            role: role,
            has_moved: false,
        }
    }
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    white: bool,
    board: [[Option<Piece>; 8]; 8], // [row][col] or [rank][file], origin at top left
    //...
}

impl Game {
    /// Initialises board
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            white: true,
            board: {
                let mut b = [[None; 8]; 8];

                // b[1] = [Some(Piece::new(Color::Black, Role::Pawn)); 8];
                // b[6] = [Some(Piece::new(Color::White, Role::Pawn)); 8];

                b[0][0] = Some(Piece::new(Color::Black, Role::Rook));
                b[0][7] = Some(Piece::new(Color::Black, Role::Rook));
                b[7][0] = Some(Piece::new(Color::White, Role::Rook));
                b[7][7] = Some(Piece::new(Color::White, Role::Rook));

                b[0][1] = Some(Piece::new(Color::Black, Role::Knight));
                b[0][6] = Some(Piece::new(Color::Black, Role::Knight));
                b[7][1] = Some(Piece::new(Color::White, Role::Knight));
                b[7][6] = Some(Piece::new(Color::White, Role::Knight));

                b[0][2] = Some(Piece::new(Color::Black, Role::Bishop));
                b[0][5] = Some(Piece::new(Color::Black, Role::Bishop));
                b[7][2] = Some(Piece::new(Color::White, Role::Bishop));
                b[7][5] = Some(Piece::new(Color::White, Role::Bishop));

                b[0][3] = Some(Piece::new(Color::Black, Role::Queen));
                b[7][3] = Some(Piece::new(Color::White, Role::Queen));

                b[0][4] = Some(Piece::new(Color::Black, Role::King));
                b[7][4] = Some(Piece::new(Color::White, Role::King));

                b
            }
            //...
            

        }
    }


    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        None
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Implement en passant and castling.
    pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>> {
        let t = self.is_check(Color::White);
        if t {
            println!("check!");
        }
        else {
            println!("not check");
        }
        
        None
    }

    pub fn get_covered_squares(&self, _position: &str) -> Option<Vec<String>> {
        let rank = _position.chars().next().unwrap();
        let file = _position.chars().nth(1).unwrap();
        let r: usize = rank as usize - 48; // row
        let c: usize = file as usize - 48; // col

        let mut covered: Vec<String> = Vec::new();

        // piece movement direction sets
        let rook_set: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)]; // down, up, right, left
        let bishop_set: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)]; // downright, upright, downleft, upleft
        
        let knight_set: [(i8, i8); 8] = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];
        let king_set: [(i8, i8); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)]; // rook_set + bishop_set (needed because non_sliding_pieces takes array of length 8 as argument)

        // check if there is piece at position
        if let Some(p) = self.board[r][c] {
            let color = p.color;

            // match role to piece at position
            match p.role {
                Role::Pawn => {
                    println!("pawn at {}", _position);

                },
                Role::Rook => {
                    println!("rook at {}", _position);
                    covered.append(&mut self.sliding_pieces(r, c, rook_set));

                },
                Role::Knight => {
                    println!("knight at {}", _position);
                    covered.append(&mut self.non_sliding_pieces(r, c, knight_set));

                },
                Role::Bishop => {
                    println!("bishop at {}", _position);
                    covered.append(&mut self.sliding_pieces(r, c, bishop_set));

                },
                Role::Queen => {
                    println!("queen at {}", _position);
                    // queen movement is combination of rook and bishop
                    covered.append(&mut self.sliding_pieces(r, c, rook_set));
                    covered.append(&mut self.sliding_pieces(r, c, bishop_set));

                }
                Role::King => {
                    println!("king at {}", _position);
                    covered.append(&mut self.non_sliding_pieces(r, c, king_set));
                }
            }
        }
        else {
            return None;
        }

        return Some(covered);
    }

    pub fn sliding_pieces(&self, r: usize, c: usize, set: [(i8, i8); 4]) -> Vec<String> {
        let mut covered: Vec<String> = Vec::new();
        let color = self.board[r][c].unwrap().color;

        for s in set {
            let mut new_r = r as i8 + s.0;
            let mut new_c = c as i8 + s.1;

            while (new_r >= 0 && new_r <= 7) && (new_c >= 0 && new_c <= 7) {
                let next = self.board[new_r as usize][new_c as usize];
                match next {
                    Some(piece) => {                        
                        // check if color matches
                        if piece.color != color {
                            let mut new_pos: String = new_r.to_string();
                            new_pos.push_str(&new_c.to_string());
                            println!("capture possible at {}!", new_pos);
                            covered.push(new_pos);
                        }

                        break;
                    },
                    None => {
                        let mut new_pos: String = new_r.to_string();
                        new_pos.push_str(&new_c.to_string());
                        println!("move possible to {}", new_pos);
                        covered.push(new_pos);
                    }
                }

                new_r += s.0;
                new_c += s.1;
            }
        }

        return covered;
    }

    pub fn non_sliding_pieces(&self, r: usize, c: usize, set: [(i8, i8); 8]) -> Vec<String> {
        let mut covered: Vec<String> = Vec::new();
        let color = self.board[r][c].unwrap().color;

        for s in set {
            let mut new_r = r as i8 + s.0;
            let mut new_c = c as i8 + s.1;

            if (new_r >= 0 && new_r <= 7) && (new_c >= 0 && new_c <= 7) {
                let next = self.board[new_r as usize][new_c as usize];
                match next {
                    Some(piece) => {                        
                        // check if color matches
                        if piece.color != color {
                            let mut new_pos: String = new_r.to_string();
                            new_pos.push_str(&new_c.to_string());
                            println!("capture possible at {}!", new_pos);
                            covered.push(new_pos);
                        }
                    },
                    None => {
                        let mut new_pos: String = new_r.to_string();
                        new_pos.push_str(&new_c.to_string());
                        println!("move possible to {}", new_pos);
                        covered.push(new_pos);
                    }
                }

                new_r += s.0;
                new_c += s.1;
            }
        }

        return covered;
    }

    pub fn is_check(&self, current_color: Color) -> bool {
        let mut check = false;
        
        // maybe move/offload?
        let opp_color: Color;
        if current_color == Color::White {
            opp_color = Color::Black;
        }
        else {
            opp_color = Color::White;
        }

        // find opponent king position
        let mut opp_king_pos: String = String::new();
        for (r, row) in self.board.iter().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                if let Some(p) = piece {
                    if p.color == opp_color && p.role == Role::King {
                        opp_king_pos.push_str(&r.to_string());
                        opp_king_pos.push_str(&c.to_string());
                    }
                }
            }
        }

        // loop thru all pieces
        let mut current_pos: String = String::new();
        for (r, row) in self.board.iter().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                if let Some(p) = piece {
                    if p.color == current_color {
                        current_pos.clear();
                        current_pos.push_str(&r.to_string());
                        current_pos.push_str(&c.to_string());
                        let covered = self.get_covered_squares(&current_pos).unwrap();
                        if covered.contains(&opp_king_pos) {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }
}

/// Implement print routine for Game.
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        let mut output: String = String::new();

        output.push_str("\n");
        for row in self.board {
            for piece in row {
                match piece {
                    Some(p) => {
                        let color: &str;
                        let role: &str;

                        match p.color {
                            Color::White => color = "w",
                            Color::Black => color = "b",
                        }

                        match p.role {
                            Role::Pawn => role = "P",
                            Role::Rook => role = "R",
                            Role::Knight => role = "N",
                            Role::Bishop => role = "B",
                            Role::Queen => role = "Q",
                            Role::King => role = "K"
                        }

                        output.push_str(color);
                        output.push_str(role);
                        output.push_str(" ");
                    },
                    None => output.push_str(" . "),
                }
            }
            output.push_str("\n");
        }

        write!(f, "{}", output)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation

    // cargo test -- --nocapture --test-threads=1

    #[test]
    fn game_in_progress_after_init() {

        let mut game = Game::new();

        
        println!("{:?}", game);


        let pos = "01"; // rank, file, index from 0
        let moves = game.get_covered_squares(pos);
        game.get_possible_moves(pos);
        
        /*
        match moves {
            Some(m) => {
                for mm in m {
                    println!("{}", mm);
                }
            },
            None => println!("none here"),
        }
        */

        println!();
        println!();
        println!();
        println!();

        

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}