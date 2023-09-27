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
    turn: Color,
    board: [[Option<Piece>; 8]; 8], // [row][col] or [rank][file], origin at top left
    //...
}

impl Game {
    /// Initialises board
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            turn: Color::White,
            board: {
                let mut b = [[None; 8]; 8];

                b[1] = [Some(Piece::new(Color::Black, Role::Pawn)); 8];
                b[6] = [Some(Piece::new(Color::White, Role::Pawn)); 8];
                
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
                
                // test pieces
                //b[1][4] = Some(Piece::new(Color::Black, Role::Rook)); // test piece
                //b[5][5] = Some(Piece::new(Color::Black, Role::Pawn)); // test piece

                b
            }
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        let frank = _from.chars().next().unwrap();
        let ffile = _from.chars().nth(1).unwrap();
        let fr: usize = frank as usize - 48; // row
        let fc: usize = ffile as usize - 48; // col

        let trank = _to.chars().next().unwrap();
        let tfile = _to.chars().nth(1).unwrap();
        let tr: usize = trank as usize - 48; // row
        let tc: usize = tfile as usize - 48; // col

        // check if piece exists on square
        if let Some(mut piece) = self.board[fr][fc] {
            // check if piece is correct color (offload to gui?)
            if piece.color == self.turn {
                let covered: Vec<String> = self.get_covered_squares(_from).unwrap();

                // check move pseudolegality
                if covered.contains(&_to.to_string()) {
                    println!("try move from {} to {}", _from, _to);

                    piece.has_moved = true;

                    // move piece
                    let temp_piece = self.board[tr][tc]; // store piece thats on _to in case there is one
                    self.board[tr][tc] = Some(piece);
                    self.board[fr][fc] = None;

                    // check move legality
                    if self.is_check(piece.color) {
                        // move illegal
                        println!("move illegal");

                        // move pieces back
                        piece.has_moved = false;
                        self.board[fr][fc] = Some(piece);
                        self.board[tr][tc] = temp_piece;

                        return None;
                    }
                    else {
                        // move legal
                        println!("move legal");

                        // check if other player under check
                        let ischeck: bool;

                        // update turn and check for check
                        if self.turn == Color::White {
                            self.turn = Color::Black;
                            ischeck = self.is_check(self.turn);
                            println!("black's turn");
                        }
                        else {
                            self.turn = Color::White;
                            ischeck = self.is_check(self.turn);
                            println!("white's turn");
                        }

                        if ischeck {
                            self.state = GameState::Check;
                            println!("check!!");

                            let ismate = self.is_mate(self.turn);
                            if ismate {
                                self.state = GameState::GameOver;
                                println!("mate!!! game over");
                            }
                        }
                        else {
                            self.state = GameState::InProgress;
                            println!("inprogress");
                        }

                        return Some(self.state);
                    }
                }
                else {
                    return None;
                }
            }
            else {
                return None
            }            
        }
        else {
            return None;
        }
    }

    pub fn is_mate(&mut self, color: Color) -> bool {
        let mut mate: bool;

        // loop thru pieces
        let mut pos: String = String::new();
        for r in 0..7 {
            for c in 0..7 {
                if let Some(mut piece) = self.board[r][c] {
                    if piece.color == color {
                        pos.clear();
                        pos.push_str(&r.to_string());
                        pos.push_str(&c.to_string());

                        let covered = self.get_covered_squares(&pos).unwrap();
                        for cov in covered {
                            let trank = cov.chars().next().unwrap();
                            let tfile = cov.chars().nth(1).unwrap();
                            let tr: usize = trank as usize - 48; // row
                            let tc: usize = tfile as usize - 48; // col
                            // move piece
                            let temp_piece = self.board[tr][tc]; // store piece thats on _to in case there is one
                            self.board[tr][tc] = Some(piece);
                            self.board[r][c] = None;

                            // check if resolving move exists, if no resolving move exists, we have mate
                            if !self.is_check(piece.color) {
                                // if is_check is false, then resolving move exists

                                // move pieces back
                                self.board[r][c] = Some(piece);
                                self.board[tr][tc] = temp_piece;
                                return false;
                            }

                            // move pieces back
                            self.board[r][c] = Some(piece);
                            self.board[tr][tc] = temp_piece;
                        }
                    }
                }
            }
        }

        // if all loops have been gone thru, then no resolving move must exist, thus mate
        return true;
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

        // piece movement directions
        let pawn_set: [(i8, i8); 3] = [(1, 0), (1, 1), (1, -1)]; // valid for black pawns, negative values for white pawns

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
                    //println!("pawn at {}", _position);
                    covered.append(&mut self.pawn_pieces(r, c, pawn_set))
                },
                Role::Rook => {
                    //println!("rook at {}", _position);
                    covered.append(&mut self.sliding_pieces(r, c, rook_set));
                },
                Role::Knight => {
                    //println!("knight at {}", _position);
                    covered.append(&mut self.non_sliding_pieces(r, c, knight_set));
                },
                Role::Bishop => {
                    //println!("bishop at {}", _position);
                    covered.append(&mut self.sliding_pieces(r, c, bishop_set));
                },
                Role::Queen => {
                    //println!("queen at {}", _position);
                    // queen movement is combination of rook and bishop
                    covered.append(&mut self.sliding_pieces(r, c, rook_set));
                    covered.append(&mut self.sliding_pieces(r, c, bishop_set));
                },
                Role::King => {
                    //println!("king at {}", _position);
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
                            //println!("capture possible at {}!", new_pos);
                            covered.push(new_pos);
                        }

                        break;
                    },
                    None => {
                        let mut new_pos: String = new_r.to_string();
                        new_pos.push_str(&new_c.to_string());
                        //println!("move possible to {}", new_pos);
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
            let new_r = r as i8 + s.0;
            let new_c = c as i8 + s.1;

            if (new_r >= 0 && new_r <= 7) && (new_c >= 0 && new_c <= 7) {
                let next = self.board[new_r as usize][new_c as usize];
                match next {
                    Some(piece) => {                        
                        // check if color matches
                        if piece.color != color {
                            let mut new_pos: String = new_r.to_string();
                            new_pos.push_str(&new_c.to_string());
                            //println!("capture possible at {}!", new_pos);
                            covered.push(new_pos);
                        }
                    },
                    None => {
                        let mut new_pos: String = new_r.to_string();
                        new_pos.push_str(&new_c.to_string());
                        //println!("move possible to {}", new_pos);
                        covered.push(new_pos);
                    }
                }
            }
        }

        return covered;
    }

    pub fn pawn_pieces(&self, r: usize, c: usize, set: [(i8, i8); 3]) -> Vec<String> {
        let mut covered: Vec<String> = Vec::new();
        let current_piece = self.board[r][c].unwrap();
        let color = current_piece.color;

        for (i, s) in set.iter().enumerate() {
            let mut s0 = s.0;
            let mut s1 = s.1;

            if color == Color::White {
                s0 *= -1;
                s1 *= -1;
            }

            let mut new_r = r as i8 + s0;
            let mut new_c = c as i8 + s1;
            
            if (new_r >= 0 && new_r <= 7) && (new_c >= 0 && new_c <= 7) {
                if i == 0 {
                    let mut next = self.board[new_r as usize][new_c as usize];
                    match next {
                        Some(piece) => {},
                        None => {
                            let mut new_pos: String = new_r.to_string();
                            new_pos.push_str(&new_c.to_string());
                            //println!("move possible to {}", new_pos);
                            covered.push(new_pos);

                            // allow move over 2 squares if piece has not yet moved
                            if !current_piece.has_moved {
                                new_r += s0;
                                new_c += s1;
                                next = self.board[new_r as usize][new_c as usize];

                                match next {
                                    Some(piece) => {},
                                    None => {
                                        let mut new_pos: String = new_r.to_string();
                                        new_pos.push_str(&new_c.to_string());
                                        //println!("move possible to {}", new_pos);
                                        covered.push(new_pos);
                                    }
                                }

                                // check for has_moved in make_move?
                            }
                        }
                    }
                }
                else { // capture case
                    let next = self.board[new_r as usize][new_c as usize];
                    match next {
                        Some(piece) => {                        
                            // check if color matches
                            if piece.color != color {
                                let mut new_pos: String = new_r.to_string();
                                new_pos.push_str(&new_c.to_string());
                                //println!("capture possible at {}!", new_pos);
                                covered.push(new_pos);
                            }
                        },
                        None => {}
                    }
                }
            }

        }

        return covered;
    }

    // checks if a color is under check
    pub fn is_check(&self, color: Color) -> bool {
        // maybe move/offload?
        let opp_color: Color;
        if color == Color::White {
            opp_color = Color::Black;
        }
        else {
            opp_color = Color::White;
        }

        // find player king position
        let mut king_pos: String = String::new();
        for (r, row) in self.board.iter().enumerate() {
            for (c, square) in row.iter().enumerate() {
                if let Some(piece) = square {
                    if piece.color == color && piece.role == Role::King {
                        king_pos.push_str(&r.to_string());
                        king_pos.push_str(&c.to_string());
                    }
                }
            }
        }

        // loop thru all opponent pieces
        let mut current_pos: String = String::new();
        for (r, row) in self.board.iter().enumerate() {
            for (c, square) in row.iter().enumerate() {
                if let Some(piece) = square {
                    if piece.color == opp_color {
                        current_pos.clear();
                        current_pos.push_str(&r.to_string());
                        current_pos.push_str(&c.to_string());
                        let covered = self.get_covered_squares(&current_pos).unwrap();
                        if covered.contains(&king_pos) {
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
            for square in row {
                match square {
                    Some(piece) => {
                        let color: &str;
                        let role: &str;

                        match piece.color {
                            Color::White => color = "w",
                            Color::Black => color = "b",
                        }

                        match piece.role {
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
    use std::io;

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
        
        /*println!("{:?}", game);

        let pos = "64"; // rank, file, index from 0
        game.make_move(pos, "55");

        println!("{:?}", game);

        game.make_move(pos, "54");

        println!("{:?}", game);

        game.get_covered_squares("54");*/
            
        let stdin = io::stdin();
        for i in 0..30 {
            println!("{:?}", game);

            let mut from = String::new();
            stdin.read_line(&mut from).expect("error");
            from.pop();

            game.get_covered_squares(&from);

            let mut to = String::new();
            stdin.read_line(&mut to).expect("error");
            to.pop();

            game.make_move(&from, &to);
        }

        println!();
        println!();
        println!();
        println!();

        
        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}



/* TODO
- get_possible_moves
- move method that makes move without checking legality ??


*/

