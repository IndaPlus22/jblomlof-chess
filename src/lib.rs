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
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    piece_pos : [u8 ; 32], /*
    Each piece will be stored in a u8 call it n, that gives the resulting int. of n/8 the file.
    And n % 8 the rank that the piece is on.
    16 elements in the array because there is in total 16 pieces.
    They will be orderd White king - 0 , White queen - 1, White rook - 2and3, White bishop - 4and5,
    White knight - 6and7, White pawn - 8_till_15, same for black but +16 on index
    */

    piece_dead : u16, // each bit represents if a piece is alive. Altough i only need to worry about
                       // kings, queens, rook, bishops and knights. Pawns can never reach rank 1 (if they are white)
                       // and never reach rank 8 if they are black, thus i can store their alive value in themselves.
                        // a 1 is dead. Same order as piece_pos, except pawns are not in it.
                       }

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            piece_pos: [32, 24, 0, 56, 16, 40, 8, 48, // white rank 1
                        1, 9, 17, 25, 33, 41 ,49, 57, // white pawns at rank 2
                        39, 31, 7, 63, 23, 47, 15, 55, // black at rank 8
                        6, 14, 22, 30, 38, 46, 54, 62], // black pawns at rank 7
            piece_dead : 0,
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        
        Some(self.state)
    }

    /// Set the piece type that a pawn becames following a promotion.
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
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        None
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
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}