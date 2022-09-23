use std::{f32::consts::PI, fmt, vec};

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
 * - Write well and clean code!
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
    to_promote_to : u8, // queen 0, rook 1, bishop 2, knight 3,
}

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
//}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            colour_of_piece: [12, 21],   //random numbers rn
            pieces: [12, 2, 1, 3, 4, 5], //random
            to_promote_to : 0,
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

    pub fn is_colour_in_check (&self, _is_white : bool) -> bool {
        let mut king_index : u32 = 0; // will be iterated throu to find the value.
        let mut bit_pos : u64;
        loop {
            bit_pos = 2_u64.pow(king_index);
            if self.pieces[5/*Piece::King */] & bit_pos == bit_pos {
                if self.colour_of_piece[if _is_white {0} else {1}] & bit_pos == bit_pos {
                    //found the correct coloured king
                    let (x, y) = (king_index / 8, king_index % 8);
                    return self.is_bishop_logic_threat(_is_white, x, y)
                    | self.is_rook_logic_threat(_is_white, x, y) 
                    | self.is_knight_logic_threat(_is_white, x, y)
                    | self.is_pawn_logic_threat(_is_white, x, y)
                    | self.is_king_logic_threat(_is_white, x, y);
                } 
            }
            king_index += 1;
        }
    }
    
    fn is_king_logic_threat(&self, _is_white : bool, x : u32, y : u32) -> bool {
        for i in -1..2{
            for j in -1..2 {
                if (j == 0) & (i == 0){
                    continue;
                } else {
                    if (x as i32 + i >= 0) & (x as i32 + i < 8) & (y as i32 + j >= 0) & (y as i32 + j < 8) {
                        let _bit_pos = 2_u64.pow((x as i32 + i) as u32 * 8 + (y as i32 + j) as u32);
                        if self.pieces[5] & _bit_pos == _bit_pos {
                            return true;
                        }
                    } 
                    
                }
            }
        }
        false
    }

    fn is_bishop_logic_threat(&self, _is_white : bool, x : u32, y : u32) -> bool {
        let mut _bit_pos : u64;
        for i in 1..((7-x).min(7-y) + 1) { // northeast
            _bit_pos = 2_u64.pow((x+i) * 8 + y + i);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[3] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            }
        }
        
        for i in 1..(x.min(7-y) + 1) { // northwest
            _bit_pos = 2_u64.pow((x-i) * 8 + y + i);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[3] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            }
        }
        for i in 1..((7-x).min(y) + 1)  { // southeast
            _bit_pos = 2_u64.pow((x+i) * 8 + y - i);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[3] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            }
        }
        for i in 1..(x.min(y) + 1) { // southwest
            _bit_pos = 2_u64.pow((x-i) * 8 + y - i);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[3] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            }
        }
        
        false
    }

    fn is_knight_logic_threat(&self, _is_white : bool, x :u32, y : u32) -> bool {
        let mut _bit_pos : u64;
        for i in 1..3 { // the file and pos,pos
            for j in 1..3 { // the rank
                if j != i {
                    let mut exp = if x >= i {(x - i) * 8} else {continue;};
                    exp += if y >= j {y - j} else {continue;};
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white {1} else {0}] & self.pieces[4] & _bit_pos == _bit_pos {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        for i in 1..3 { // the file and pos, neg
            for j in 1..3 { // the rank
                if j != i {
                    let mut exp = if x >= i {( x - i) * 8} else {continue;};
                    exp += if y <= 7 - j {y + j} else {continue;};
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white {1} else {0}] & self.pieces[4] & _bit_pos == _bit_pos {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        for i in 1..3 { // the file and neg, pos
            for j in 1..3 { // the rank
                if j != i {
                    let mut exp = if x <= 7 - i {( x + i) * 8} else {continue;};
                    exp += if y >= j {y - j} else {continue;};
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white {1} else {0}] & self.pieces[4] & _bit_pos == _bit_pos {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        for i in 1..3 { // the file and neg, neg
            for j in 1..3 { // the rank
                if j != i {
                    let mut exp = if x <=7 - i {( x + i) * 8} else {continue;};
                    exp += if y <= 7 - j {y + j} else {continue;};
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white {1} else {0}] & self.pieces[4] & _bit_pos == _bit_pos {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        false
    }
    
    fn is_pawn_logic_threat(&self, _is_white : bool, x : u32, y : u32) -> bool {
        let mut _bit_pos : u64;
        if _is_white {
            if y > 0 {
                if x > 0 {
                    _bit_pos = 2_u64.pow((x-1) * 8 + y - 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
                if x < 7 {
                    _bit_pos = 2_u64.pow((x+1) * 8 + y - 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
            }
        return false;
        } else {
            if y < 7 {
                if x > 0 {
                    _bit_pos = 2_u64.pow((x-1) * 8 + y + 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
                if x < 7 {
                    _bit_pos = 2_u64.pow((x+1) * 8 + y + 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
            }
        return false;
        }
    }
    fn is_rook_logic_threat(&self, _is_white : bool, x : u32, y : u32) -> bool {
        //im sorry but i do some what need to dupe code.
        let mut _bit_pos :u64;
        for i in (x+1)..8 { //east
            _bit_pos = 2_u64.pow(i * 8 + y);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[2] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            }
        }
        for i in (0..x).rev() { //west
            _bit_pos = 2_u64.pow(i * 8 + y);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[2] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break; 
            }
        }
        for i in (y+1)..8 { // north
            _bit_pos = 2_u64.pow(x * 8 + i);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[2] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break; 
            }
        }
        for i in (0..y).rev() { //south
            _bit_pos = 2_u64.pow(x * 8 + i);
            if (self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos) {
                if (self.pieces[1] & _bit_pos == _bit_pos) |  (self.pieces[2] & _bit_pos == _bit_pos) {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break; 
            }
        }
        false
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&mut self, _postion: &str) -> Option<Vec<String>> {
        let (file_coord, rank_coord): (u32, u32) = self.transform_input(_postion);
        let bit_pos = 2_u64.pow(file_coord * 8 + rank_coord);
        if self.is_black(file_coord, rank_coord) | self.is_white(file_coord, rank_coord) {
            let mut possible_moves_to_return: Vec<String> = vec![];
            let piece_type: Piece = self.get_that_piece_type(bit_pos);
            let is_white = self.is_white(file_coord, rank_coord);
            
            match piece_type {
                Piece::King => possible_moves_to_return.append(&mut self.search_king_moves(is_white, file_coord, rank_coord)),
                Piece::Rook => possible_moves_to_return.append(&mut self.search_king_moves(is_white, file_coord, rank_coord)),
                Piece::Knight => possible_moves_to_return.append(&mut self.search_knight_moves(is_white, file_coord, rank_coord)),
                Piece::Bishop => possible_moves_to_return.append(&mut self.search_bishop_moves(is_white, file_coord, rank_coord)),
                Piece::Queen => possible_moves_to_return.append(&mut self.search_queen_moves(is_white, file_coord, rank_coord)),
                Piece::Pawn => possible_moves_to_return.append(&mut self.search_pawn_moves(is_white, file_coord, rank_coord)) // just in the mean time
            };
        return Some(possible_moves_to_return);
        }
        //throw a exception, no piece on that square.
        None
    }

    fn would_cause_check(&mut self, _is_white : bool,_from_file : u32, _from_rank : u32, _to_file : u32, _to_rank : u32) -> bool {
        let _remember_colours = self.colour_of_piece;
        let _remember_pieces = self.pieces;
        self.do_move(_from_file, _from_rank, _to_file, _to_rank);
        let will_cause_check = self.is_colour_in_check(_is_white);
        self.colour_of_piece = _remember_colours;
        self.pieces = _remember_pieces;
        will_cause_check
        
    }

    fn do_move(&mut self, _from_file : u32, _from_rank : u32, _to_file : u32, _to_rank : u32) { //assuming input is valid
        let mut _bit_pos = 2_u64.pow(_to_file * 8 + _to_rank);
        for mut piece in self.pieces {
            if piece & _bit_pos == _bit_pos {
                self.colour_of_piece[0] &= !_bit_pos;
                self.colour_of_piece[1] &= !_bit_pos;
                piece &= !_bit_pos;
                break;
            } 
        }
        _bit_pos = 2_u64.pow(_from_file * 8 + _from_rank);
        for mut piece in self.pieces {
            if piece & _bit_pos == _bit_pos {
                if self.colour_of_piece[0] & _bit_pos == _bit_pos {
                    self.colour_of_piece[0] |= 2_u64.pow(_to_file * 8 + _to_rank);
                } else {
                    self.colour_of_piece[1] |= 2_u64.pow(_to_file * 8 + _to_rank);
                }
                self.colour_of_piece[0] &= !_bit_pos;
                self.colour_of_piece[1] &= !_bit_pos;
                piece &= !_bit_pos;
                piece |= 2_u64.pow(_to_file * 8 + _to_rank);
                break;
            } 
        
        }
    }

    fn search_queen_moves(&mut self, _is_white : bool, _from_file : u32, _from_rank : u32) -> Vec<String> { 
        // I dont know why but i cant combine theese into one line. Probavly means its shaky but i dont know why. 
        let mut queen_possible_moves : Vec<String> = self.search_bishop_moves(_is_white, _from_file, _from_rank);
        queen_possible_moves.append(&mut self.search_rook_moves(_is_white, _from_file, _from_rank));
        queen_possible_moves
    }
    fn search_king_moves(&mut self, _is_white : bool, _from_file:u32, _from_rank : u32) -> Vec<String>{
        let mut king_possible_moves : Vec<String> = vec![];
        for i in -1..2{
            for j in -1..2 {
                if (j == 0) & (i == 0){
                    continue;
                } else {
                    let new_file = _from_file as i32 + i;
                    let new_rank = _from_rank as i32 + j;
                    if (new_file >= 0) & (new_file < 8) & (new_rank >= 0) & (new_rank < 8) {
                        let _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                        if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos != _bit_pos {
                            if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank as u32) {
                                king_possible_moves.push(self.transform_back(new_file as u32, new_rank as u32));
                            }
                        }
                    }
                }     
            }
        }
        king_possible_moves
    }

    fn search_pawn_moves(&mut self, _is_white : bool, _from_file : u32, _from_rank : u32) -> Vec<String> {
        let mut pawn_possible_moves : Vec<String> = vec![];
        let mut new_rank : u32 = if _is_white {_from_rank + 1} else {_from_rank - 1};
        let mut new_file : i32;
        let mut _bit_pos : u64;
        for i in -1..2 {
            new_file = _from_file as i32 + i;
            if (new_file >= 0) & (new_file < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank);
                if i != 0{
                    if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos { // if i want to add enpassant add here
                        if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank) {
                            pawn_possible_moves.push(self.transform_back(new_file as u32, new_rank));
                        }
                    }
                } else {
                    if (self.colour_of_piece[0] | self.colour_of_piece[1]) & _bit_pos != _bit_pos {
                        if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank) {
                            pawn_possible_moves.push(self.transform_back(new_file as u32, new_rank));
                        }
                    }
                    if if _is_white {1} else {6} == _from_rank {
                        if _is_white {new_rank += 1} else {new_rank -= 1};
                        _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank);
                        if (self.colour_of_piece[0] | self.colour_of_piece[1]) & _bit_pos != _bit_pos {
                            if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank) {
                                pawn_possible_moves.push(self.transform_back(new_file as u32, new_rank));
                            }
                        }
                    }
                }
            }
        }
        pawn_possible_moves
    }
   
    fn search_knight_moves(&mut self, _is_white : bool, _from_file : u32, _from_rank :u32) -> Vec<String> {
        let mut knight_possible_moves : Vec<String> = vec![];
        let mut _bit_pos : u64;
        for i in -2..3 {
            for j in -2..3 {
                if (i.max(-i) != j.max(-j)) | (j != 0) | (i!= 0) {
                    let new_file = _from_file as i32 + i;
                    let new_rank = _from_rank as i32 + j;
                    if (new_file >= 0) & (new_file < 8) & (new_rank >= 0) & (new_rank < 8) {
                        _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                        if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos != _bit_pos{
                            if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank as u32) {
                                knight_possible_moves.push(self.transform_back(new_file as u32, new_rank as u32));
                            }
                        }
                    } 
                }
            }
        }
        knight_possible_moves
    }

    fn search_bishop_moves(&mut self, _is_white : bool, _from_file : u32, _from_rank : u32) -> Vec<String> {
        let mut bishop_possible_moves : Vec<String> = vec![];
        let mut _bit_pos : u64;
        let mut new_file : i32;
        let mut new_rank : i32;
        for i in 1..8 { // a little bit more brute force but i cant be bothered. going NE
            new_file = _from_file as i32 + i;
            new_rank = _from_rank as i32 + i;
            if (new_file < 8) & (new_rank < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank as u32) {
                        bishop_possible_moves.push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        for i in 1..8 { // sry for code dupe dont think its avoidable, going NW
            new_file = _from_file as i32 - i;
            new_rank = _from_rank as i32 + i;
            if (new_file < 8) & (new_rank < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank as u32) {
                        bishop_possible_moves.push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        for i in 1..8 { // going SE
            new_file = _from_file as i32 + i;
            new_rank = _from_rank as i32 - i;
            if (new_file < 8) & (new_rank < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank as u32) {
                        bishop_possible_moves.push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        for i in 1..8 { // going SW
            new_file = _from_file as i32 - i;
            new_rank = _from_rank as i32 - i;
            if (new_file < 8) & (new_rank < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(_is_white, _from_file, _from_rank, new_file as u32, new_rank as u32) {
                        bishop_possible_moves.push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        bishop_possible_moves
    }

    fn search_rook_moves(&mut self, _is_white : bool, _from_file : u32, _from_rank: u32) -> Vec<String> {
        let mut rook_possible_moves : Vec<String> = vec![];
        let mut _bit_pos: u64;
        for i in (1 + _from_file)..8 {
            _bit_pos = 2_u64.pow(i * 8 + _from_rank);
            if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, i, _from_rank) {
                    rook_possible_moves.push(self.transform_back(i, _from_rank));
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos { //cant move past pieces
                    break;
                }
            }
        }
        for i in (0.._from_file).rev() {
            _bit_pos = 2_u64.pow(i * 8 + _from_rank);
            if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, i, _from_rank) {
                    rook_possible_moves.push(self.transform_back(i, _from_rank));
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos { //cant move past pieces
                    break;
                }
            }
        }
        for i in (1 + _from_rank)..8 {
            _bit_pos = 2_u64.pow(_from_file * 8 + i);
            if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, _from_file, i) {
                    rook_possible_moves.push(self.transform_back(_from_file, i));
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos { //cant move past pieces
                    break;
                }
            }
        }
        for i in (0.._from_rank).rev() {
            _bit_pos = 2_u64.pow(_from_file * 8 + i);
            if self.colour_of_piece[if _is_white {0} else {1}] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, _from_file, i) {
                    rook_possible_moves.push(self.transform_back(_from_file, i));
                }
                if self.colour_of_piece[if _is_white {1} else {0}] & _bit_pos == _bit_pos { //cant move past pieces
                    break;
                }
            }
        }
        rook_possible_moves
    }

    fn get_that_piece_type(&self, _bit_pos: u64) -> Piece {
        if _bit_pos == _bit_pos & self.pieces[0] {
            return Piece::Pawn;
        } else if _bit_pos == _bit_pos & self.pieces[1] {
            return Piece::Queen;
        } else if _bit_pos == _bit_pos & self.pieces[2] {
            return Piece::Rook;
        } else if _bit_pos == _bit_pos & self.pieces[3] {
            return Piece::Bishop;
        } else if _bit_pos == _bit_pos & self.pieces[4] {
            return Piece::Knight;
        } else {
            return Piece::King;
        }
    }

  /*  /*This function looks if the pieces are different colours. Returning false if that's the case. */
    fn is_valid_move(
        &self,
        _moving_piece_type: &Piece,
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> (bool, bool) {
        // I need to tell if its not valid, or if its valid aswell aswell if next move will be invalid.
        // the first bool means the current move is valid or not
        // the second one implies the next move will be valid.
        //eg. true,true means keep coming, true false means this move but no more false,false means stop.

        match _moving_piece_type {
            Piece::King => {
                if (_from_file.abs_diff(_to_file) > 1) //somehow i need parenthese or it crys.
            | (_from_rank.abs_diff(_to_rank) > 1)
                {
                    return (false, false);
                }
            }
            Piece::Bishop => {
                if !self.is_valid_move_for_bishop(_from_file, _from_rank, _to_file, _to_rank) {
                    return (false, false);
                }
            }
            Piece::Rook => {
                if !self.is_valid_move_for_rook(_from_file, _from_rank, _to_file, _to_rank) {
                    return (false, false);
                }
            }
            Piece::Queen => {
                if ((_from_file.abs_diff(_to_file) != 0) & (_from_rank.abs_diff(_to_rank) != 0)) // rook logic
            & (_from_file.abs_diff(_to_file) != _from_rank.abs_diff(_to_rank))
                // bishop logic
                {
                    return (false, false);
                }
            }
            Piece::Knight => {
                if !((_from_file.abs_diff(_to_file) == 2) & (_from_rank.abs_diff(_to_rank) == 1))
                    & !((_from_file.abs_diff(_to_file) == 1) & (_from_rank.abs_diff(_to_rank) == 2))
                {
                    return (false, false);
                }
            }
            Piece::Pawn => {
                if self.is_white(_from_file, _from_rank) {
                    if _to_rank >= _from_rank {
                        return (false, false);
                    }
                    if _from_rank == 6 {
                        if ((_from_file.abs_diff(_to_file) != 0) | (_from_rank.abs_diff(_to_rank) > 2)) // this checks for a normal move.
                    & (!self.is_black(_to_file, _to_rank) | (_from_file.abs_diff(_to_file) != 1)| (_from_rank.abs_diff(_to_rank) != 1))
                        {
                            return (false, false);
                        }
                    } else {
                        if ((_from_file.abs_diff(_to_file) != 0) | (_from_rank.abs_diff(_to_rank) > 1)) //Im sorry for code dupe, but right now i cant figure out a better way to do it without it.
                    & (!self.is_black(_to_file, _to_rank) | (_from_file.abs_diff(_to_file) != 1)| (_from_rank.abs_diff(_to_rank) != 1))
                        {
                            return (false, false);
                        }
                    }
                } else {
                    //is black
                    //Sorry for even more code dupe, i dont think making a func is the right call.
                    if _to_rank <= _from_rank {
                        return (false, false);
                    }
                    if _from_rank == 1 {
                        if ((_from_file.abs_diff(_to_file) != 0)
                            | (_from_rank.abs_diff(_to_rank) > 2))
                            & (!self.is_black(_to_file, _to_rank)
                                | (_from_file.abs_diff(_to_file) != 1)
                                | (_from_rank.abs_diff(_to_rank) != 1))
                        {
                            return (false, false);
                        }
                    } else {
                        if ((_from_file.abs_diff(_to_file) != 0)
                            | (_from_rank.abs_diff(_to_rank) > 1))
                            & (!self.is_black(_to_file, _to_rank)
                                | (_from_file.abs_diff(_to_file) != 1)
                                | (_from_rank.abs_diff(_to_rank) != 1))
                        {
                            return (false, false);
                        }
                    }
                }
            }
        }

        //time to check if the moving piece will land on a another piece.
        if self.is_white(_from_file, _from_rank) {
            // is the moving piece black or white
            if self.is_white(_to_file, _to_rank) {
                return (false, false);
            } else if self.is_black(_to_file, _to_rank) {
                return (true, false);
            }
        } else if self.is_black(_to_file, _to_rank) {
            return (false, false);
        } else if self.is_white(_from_file, _from_rank) {
            return (true, false);
        }

        (true, true)
    }

    fn is_valid_move_for_rook(
        &self,
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> bool {
        (_from_file.abs_diff(_to_file) == 0) | (_from_rank.abs_diff(_to_rank) != 0)
    }

    fn is_valid_move_for_bishop(
        &self,
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> bool {
        _from_file.abs_diff(_to_file) == _from_rank.abs_diff(_to_rank)
    }

    fn is_valid_move_for_king(
        &self,
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> bool {
        (_from_file.abs_diff(_to_file) <= 1) & (_from_rank.abs_diff(_to_rank) <= 1)
    }

    fn is_valid_move_for_knight(
        &self,
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> bool {
        ((_from_file.abs_diff(_to_file) == 2) & (_from_rank.abs_diff(_to_rank) == 1))
            | ((_from_file.abs_diff(_to_file) == 1) & (_from_rank.abs_diff(_to_rank) == 2))
    }

    fn is_valid_move_for_pawn(
        &self,
        _colour: u32, //0 for white, 1 for black
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> bool {
        if _colour == 0 {
            //White piece moving
            if _to_rank > _from_rank {
                if _from_file.abs_diff(_to_file) == 0 {
                    if _from_rank == 1 {}
                } else if _from_file.abs_diff(_to_file) == 1{

                } else 
            }
        } else {
        }
    }
*/
    fn transform_input(&self, input_pos: &str) -> (u32, u32) {
        let mut chars_iter = input_pos.chars();
        (
            chars_iter.next().unwrap().to_digit(17).unwrap() - 10,
            chars_iter.next().unwrap().to_digit(10).unwrap() - 1,
        )
    }

    fn transform_back(&self, file_input: u32, rank_input: u32) -> String {
        return ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
            .get(file_input as usize)
            .expect("Error on file_rank")
            .to_string()
            + &(rank_input + 1).to_string();
    }

    fn is_white(&self, file_coord: u32, rank_coord: u32) -> bool {
        //simply compare the bits in colour_of_pieces
        let bit_coord: u64 = 2_u64.pow(file_coord * 8 + rank_coord);
        self.colour_of_piece[0/*Colour::White*/] & bit_coord == bit_coord
    }

    fn is_black(&self, file_coord: u32, rank_coord: u32) -> bool {
        // I need this to check if black piece moves to black piece
        let bit_coord: u64 = 2_u64.pow(file_coord * 8 + rank_coord);
        self.colour_of_piece[1/*Colour::Black*/] & bit_coord == bit_coord
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
