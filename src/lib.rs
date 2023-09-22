use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * - Read the Rust documentation, ask questions if you get stuck!
 */

 #[derive(Copy, Clone, Debug)]
 enum Piece { // nested enum for color
    Pawn, Rook
 }

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    white: bool,
    board: [[Option<Piece>; 8]; 8], // [row][col] or [rank][file]
    //...
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            white: true,
            board: [[None; 8]; 8]
            //...
            

        }
    }

    pub fn init_board(&mut self) {
        self.board[0][0] = Some(Piece::Rook);
        self.board[0][7] = Some(Piece::Rook);
        self.board[1] = [Some(Piece::Pawn); 8];
    }

    pub fn p(&self) {
        println!();
        for c in self.board {
            for r in c {
                match r {
                    Some(Piece::Pawn) => print!("P "),
                    Some(Piece::Rook) => print!("R "),
                    None => print!(". ")
                }
            }
            println!();
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
        let file = _position.chars().next().unwrap();
        let rank = _position.chars().nth(1).unwrap();
        let f: usize = file as usize - 49;
        let r: usize = rank as usize - 49;

        let mut moves: Vec<String> = Vec::new();

        let piece = self.board[r][f];
        match piece {
            Some(Piece::Pawn) => {
                print!("pawn here");
                
                


            },
            Some(Piece::Rook) => print!("rook here"),
            None => return None
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
        
        write!(f, "")
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
        game.init_board();

        //game.init_board();
        game.p();
        
        let pos = "52"; // file, rank, index from 1
        let moves = game.get_possible_moves(pos);
        if moves.is_none() {
            println!("none");
        }

        // println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}