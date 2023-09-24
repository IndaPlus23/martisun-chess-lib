use std::fmt;

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
    Pawn, Rook
}


/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * - Read the Rust documentation, ask questions if you get stuck!
 */

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
                b[1] = [Some(Piece::new(Color::Black, Role::Pawn)); 8];
                b[6] = [Some(Piece::new(Color::White, Role::Pawn)); 8];

                b[0][0] = Some(Piece::new(Color::Black, Role::Rook));
                b[0][7] = Some(Piece::new(Color::Black, Role::Rook));
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
    pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<(usize, usize)>> {
        let file = _position.chars().next().unwrap();
        let rank = _position.chars().nth(1).unwrap();
        let f: usize = file as usize - 49;
        let r: usize = rank as usize - 49;

        let pos = (r, f);

        let mut moves: Vec<(usize, usize)> = Vec::new();

        
        let piece = self.board[pos.0][pos.1];
        if let Some(p) = piece {
            match p.role {
                Role::Pawn => {

                    println!("p");
                },
                Role::Rook => {
                    let mut i = 0;

                    let nextpos = self.board[pos.0][pos.1 + 1];


                    // move right
                    loop {
                        
                    }





                    println!("r");
                },
                //_ => println!("nothing"),
            }
        }
        else {
            return None;
        }


        return Some(moves);
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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


        let pos = "53"; // file, rank, index from 1
        let moves = game.get_possible_moves(pos);
        if moves.is_none() {
            println!("none");
        }

        println!();
        println!();
        println!();
        println!();

        

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}