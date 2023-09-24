use std::fmt;


/* TODO
- implement get_covered_squares(pos)



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
    board: [[Option<Piece>; 8]; 8], // [row][col] or [rank][file]
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

                //b[1] = [Some(Piece::new(Color::Black, Role::Pawn)); 8];
                //b[6] = [Some(Piece::new(Color::White, Role::Pawn)); 8];

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

                b[3][3] = Some(Piece::new(Color::Black, Role::Queen));
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
        None
    }

    pub fn get_covered_squares(&self, _position: &str) -> Option<Vec<String>> {
        let rank = _position.chars().next().unwrap();
        let file = _position.chars().nth(1).unwrap();
        let r: usize = rank as usize - 48; // row
        let c: usize = file as usize - 48; // col

        let mut covered: Vec<String> = Vec::new();

        let rook_set: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)]; // down, up, right, left
        let bishop_set: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)]; // downright, upright, downleft, upleft

        // check if there is piece at position
        if let Some(p) = self.board[r][c] {
            // match role to piece at pos
            match p.role {
                Role::Pawn => {
                    println!("pawn at {}", _position);

                },
                Role::Rook => {
                    println!("rook at {}", _position);
                    covered.append(&mut self.sliding_pieces(r, c, rook_set));

                },
                Role::Bishop => {
                    println!("bishop at {}", _position);
                    covered.append(&mut self.sliding_pieces(r, c, bishop_set));

                },
                Role::Queen => {
                    println!("queen at {}", _position);
                    covered.append(&mut self.sliding_pieces(r, c, rook_set));
                    covered.append(&mut self.sliding_pieces(r, c, bishop_set));

                }
                _ => println!("nothing"),
            }
        }
        else {
            return None;
        }

        return Some(covered);
    }

    pub fn sliding_pieces(&self, r: usize, c: usize, set: [(i8, i8); 4]) -> Vec<String> {
        let mut covered: Vec<String> = Vec::new();

        for s in set {
            let mut new_r = r as i8 + s.0;
            let mut new_c = c as i8 + s.1;

            while (new_r >= 0 && new_r <= 7) && (new_c >= 0 && new_c <= 7) {
                if let Some(np) = self.board[new_r as usize][new_c as usize] {
                    println!("something here at {}{}", new_r, new_c);
                    break;
                }
                else {
                    let mut new_pos: String = new_r.to_string();
                    new_pos.push_str(&new_c.to_string());
                    println!("nothing here at {}", new_pos);
                    covered.push(new_pos);
                }

                new_r += s.0;
                new_c += s.1;
            }
        }

        return covered;
    }


}

/// Implement print routine for Game.
impl fmt::Debug for Game {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        let mut output: String = String::new();

        output.push_str("\n");
        for c in self.board {
            for r in c {
                match r {
                    Some(p) => {
                        let c: &str;
                        let r: &str;

                        match p.color {
                            Color::White => c = "w",
                            Color::Black => c = "b",
                        }

                        match p.role {
                            Role::Pawn => r = "P",
                            Role::Rook => r = "R",
                            Role::Knight => r = "N",
                            Role::Bishop => r = "B",
                            Role::Queen => r = "Q",
                            Role::King => r = "K"
                        }

                        output.push_str(c);
                        output.push_str(r);
                        output.push_str(" ");
                    },
                    None => output.push_str(" . "),
                }
            }
            output.push_str("\n");
        }

        write!(c, "{}", output)
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


        let pos = "33"; // rank, file, index from 0
        let moves = game.get_covered_squares(pos);
        match moves {
            Some(m) => {
                for mm in m {
                    println!("{}", mm);
                }
            },
            None => println!("none here"),
        }

        println!();
        println!();
        println!();
        println!();

        

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}