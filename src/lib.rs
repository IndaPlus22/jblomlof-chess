use std::{f32::consts::PI, fmt};

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum GameState {
    InProgress,
    Check,
    GameOver,
}
enum Piece {
    Pawn = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    King = 5,
}

enum Colour {
    White = 0,
    Black = 1,
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

/*Oh boy now its time to think.
Since each bit will monitor positions we need to recieve and modify bits by bitwise ops.
using =& and &*/
pub struct Game {
    /* save board, active colour, ... */

    /* The board will be divided like this. It will count from 0 to 63. The square A1 is 0, A2 is 1,
    A3 is 2... B1 is 8, B2 is 9...C1 is 16... D1 is 24... H1 is 56 ... H8 is 63 */
    state: GameState,
    colour_of_piece: [u64; 2], //white 0-index and black 1-index
    pieces: [u64; 6],
    /*piece_pos : [u8 ; 32], /*
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
                        This is definatly not the way to implement this.*/
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            colour_of_piece : [12, 21], //random numbers rn
            pieces : [12,2,1,3,4,5],//random
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

    /*This function looks if the pieces are different colours. Returning false if that's the case. */
    fn is_valid_move(
        &self,
        _moving_piece_type: Piece,
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> bool {
        // if not in possible moves.
        match _moving_piece_type {
            Piece::King => {
                if (_from_file.abs_diff(_to_file) > 1) //somehow i need parenthese or it crys.
            | (_from_rank.abs_diff(_to_rank) > 1)
                {
                    return false;
                }
            }
            Piece::Bishop => {
                if _from_file.abs_diff(_to_file) != _from_rank.abs_diff(_to_rank) {
                    return false;
                }
            }
            Piece::Rook => {
                if (_from_file.abs_diff(_to_file) != 0) & (_from_rank.abs_diff(_to_rank) != 0) {
                    return false;
                }
            }
            Piece::Queen => {
                if ((_from_file.abs_diff(_to_file) != 0) & (_from_rank.abs_diff(_to_rank) != 0)) // rook logic
            & (_from_file.abs_diff(_to_file) != _from_rank.abs_diff(_to_rank))
                {
                    return false;
                }
            }
            Piece::Knight => {
                if !((_from_file.abs_diff(_to_file) == 2) & (_from_rank.abs_diff(_to_rank) == 1))
                    & !((_from_file.abs_diff(_to_file) == 1) & (_from_rank.abs_diff(_to_rank) == 2))
                {
                    return false;
                }
            }
            Piece::Pawn => (),  // TODO, remember colour matters.
            _ => (),
        }
        if self.is_white(_from_file, _from_rank) == self.is_white(_to_file, _to_rank) {
            return false;
        }
        true
    }

    fn is_white(&self, file_coord: u32, rank_coord: u32) -> bool {
        // dont know if to have is_black aswell, depends on how sure the input is.
        let bit_coord: u64 = 2_u64.pow(file_coord * 8 + rank_coord);
        self.colour_of_piece[0/*Colour::White*/] & bit_coord == bit_coord
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
