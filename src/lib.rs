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

/*enum Colour {
    White = 0,
    Black = 1,
}*/

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
    white_turn: bool,
    colour_of_piece: [u64; 2], //white 0-index and black 1-index
    pieces: [u64; 6],
    to_promote_to: u8, // queen 1, rook 2, bishop 3, knight 4,
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
            white_turn: true,
            to_promote_to: 1,
            colour_of_piece: [
                {
                    let mut white_map: u64 = 0;
                    for i in 0..8 {
                        white_map += 2_u64.pow(i * 8) + 2_u64.pow(i * 8 + 1)
                    }
                    white_map
                },
                {
                    let mut black_map: u64 = 0;
                    for i in 0..8 {
                        black_map += 2_u64.pow(i * 8 + 6) + 2_u64.pow(i * 8 + 7)
                    }
                    black_map
                },
            ],
            pieces: [
                {
                    let mut pawn_map: u64 = 0;
                    for i in 0..8 {
                        pawn_map += 2_u64.pow(i * 8 + 1) + 2_u64.pow(i * 8 + 6);
                    }
                    pawn_map // pawn starting pos
                },
                2_u64.pow(24) + 2_u64.pow(31), // queen starting pos
                2_u64.pow(0) + 2_u64.pow(7) + 2_u64.pow(56) + 2_u64.pow(63), // rook starting pos
                2_u64.pow(16) + 2_u64.pow(23) + 2_u64.pow(40) + 2_u64.pow(47), //bishop starting pos
                2_u64.pow(8) + 2_u64.pow(15) + 2_u64.pow(48) + 2_u64.pow(55), //knight starting pos
                2_u64.pow(32) + 2_u64.pow(39), // kings starting pos.
            ],
        }
    }

    /// If the move is legal it will make the move.
    /// Inputs are the positions, first arg is from square and second arg is to square.
    /// Return true if the move was made.
    /// Else returns false, thus meaning the move was invalid.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> bool {
        if self
            .get_possible_moves(_from)
            .unwrap()
            .iter()
            .any(|_i| _i == _to)
        {
            let (from_file, from_rank) = self.transform_input(_from);
            if self.white_turn {
                if !self.is_white(from_file, from_rank) {
                    return false;
                }
            } else if !self.is_black(from_file, from_rank) {
                return false;
            }
            let (to_file, to_rank) = self.transform_input(_to);
            self.do_move(from_file, from_rank, to_file, to_rank);

            if self.is_colour_in_check(true) | self.is_colour_in_check(false) {
                self.state = GameState::Check;
            }
            self.white_turn = !self.white_turn;
            return true;
        }
        false
    }

    /// Set the piece type that a pawn becames following a promotion.
    /// Input should be broadly accepted. For example if you choose knight promotion:
    /// "knight" "Knight" "KNIGHT" "n" "N" are valid.
    /// Same inputs apply to rook and bishop.  
    /// Every other input will assume queen promotion.
    pub fn set_promotion(&mut self, _piece: &str) {
        self.to_promote_to = match _piece {
            "bishop" => 3,
            "Bishop" => 3,
            "BISHOP" => 3,
            "b" => 3,
            "B" => 3,

            "rook" => 2,
            "Rook" => 2,
            "ROOK" => 2,
            "r" => 2,
            "R" => 2,

            "knight" => 4,
            "Knight" => 4,
            "KNIGHT" => 4,
            "n" => 4,
            "N" => 4,
            _ => 1,
        }
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// Returns true if the specified colour is in check. Otherwise false.
    /// Input is boolean, true if the desired colour to see if in check is white. False if the desired colour to see if in check is black.
    pub fn is_colour_in_check(&self, _is_white: bool) -> bool {
        let mut king_index: u32 = 0; // will be iterated throu to find the value.
        let mut bit_pos: u64;
        loop {
            bit_pos = 2_u64.pow(king_index);
            if self.pieces[5/*Piece::King */] & bit_pos == bit_pos {
                if self.colour_of_piece[if _is_white { 0 } else { 1 }] & bit_pos == bit_pos {
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

    fn is_king_logic_threat(&self, _is_white: bool, x: u32, y: u32) -> bool {
        for i in -1..2 {
            for j in -1..2 {
                if (j == 0) & (i == 0) {
                    continue;
                } else {
                    if (x as i32 + i >= 0)
                        & (x as i32 + i < 8)
                        & (y as i32 + j >= 0)
                        & (y as i32 + j < 8)
                    {
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

    fn is_bishop_logic_threat(&self, _is_white: bool, x: u32, y: u32) -> bool {
        let mut _bit_pos: u64;
        for i in 1..((7 - x).min(7 - y) + 1) {
            // northeast
            _bit_pos = 2_u64.pow((x + i) * 8 + y + i);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[3] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        }

        for i in 1..(x.min(7 - y) + 1) {
            // northwest
            _bit_pos = 2_u64.pow((x - i) * 8 + y + i);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[3] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        }
        for i in 1..((7 - x).min(y) + 1) {
            // southeast
            _bit_pos = 2_u64.pow((x + i) * 8 + y - i);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[3] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        }
        for i in 1..(x.min(y) + 1) {
            // southwest
            _bit_pos = 2_u64.pow((x - i) * 8 + y - i);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[3] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        }

        false
    }

    fn is_knight_logic_threat(&self, _is_white: bool, x: u32, y: u32) -> bool {
        let mut _bit_pos: u64;
        for i in 1..3 {
            // the file and pos,pos
            for j in 1..3 {
                // the rank
                if j != i {
                    let mut exp = if x >= i {
                        (x - i) * 8
                    } else {
                        continue;
                    };
                    exp += if y >= j {
                        y - j
                    } else {
                        continue;
                    };
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white { 1 } else { 0 }]
                        & self.pieces[4]
                        & _bit_pos
                        == _bit_pos
                    {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        for i in 1..3 {
            // the file and pos, neg
            for j in 1..3 {
                // the rank
                if j != i {
                    let mut exp = if x >= i {
                        (x - i) * 8
                    } else {
                        continue;
                    };
                    exp += if y <= 7 - j {
                        y + j
                    } else {
                        continue;
                    };
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white { 1 } else { 0 }]
                        & self.pieces[4]
                        & _bit_pos
                        == _bit_pos
                    {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        for i in 1..3 {
            // the file and neg, pos
            for j in 1..3 {
                // the rank
                if j != i {
                    let mut exp = if x <= 7 - i {
                        (x + i) * 8
                    } else {
                        continue;
                    };
                    exp += if y >= j {
                        y - j
                    } else {
                        continue;
                    };
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white { 1 } else { 0 }]
                        & self.pieces[4]
                        & _bit_pos
                        == _bit_pos
                    {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        for i in 1..3 {
            // the file and neg, neg
            for j in 1..3 {
                // the rank
                if j != i {
                    let mut exp = if x <= 7 - i {
                        (x + i) * 8
                    } else {
                        continue;
                    };
                    exp += if y <= 7 - j {
                        y + j
                    } else {
                        continue;
                    };
                    _bit_pos = 2_u64.pow(exp);
                    if self.colour_of_piece[if _is_white { 1 } else { 0 }]
                        & self.pieces[4]
                        & _bit_pos
                        == _bit_pos
                    {
                        return true;
                    }
                } else {
                    continue;
                }
            }
        }
        false
    }

    fn is_pawn_logic_threat(&self, _is_white: bool, x: u32, y: u32) -> bool {
        let mut _bit_pos: u64;
        if _is_white {
            if y > 0 {
                if x > 0 {
                    _bit_pos = 2_u64.pow((x - 1) * 8 + y - 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
                if x < 7 {
                    _bit_pos = 2_u64.pow((x + 1) * 8 + y - 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
            }
            return false;
        } else {
            if y < 7 {
                if x > 0 {
                    _bit_pos = 2_u64.pow((x - 1) * 8 + y + 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
                if x < 7 {
                    _bit_pos = 2_u64.pow((x + 1) * 8 + y + 1);
                    if self.pieces[0] & self.colour_of_piece[1] & _bit_pos == _bit_pos {
                        return true;
                    }
                }
            }
            return false;
        }
    }

    fn is_rook_logic_threat(&self, _is_white: bool, x: u32, y: u32) -> bool {
        //im sorry but i do some what need to dupe code.
        let mut _bit_pos: u64;
        for i in (x + 1)..8 {
            //east
            _bit_pos = 2_u64.pow(i * 8 + y);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[2] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        }
        for i in (0..x).rev() {
            //west
            _bit_pos = 2_u64.pow(i * 8 + y);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[2] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        } 
        for i in (y + 1)..8 {
            // north
            _bit_pos = 2_u64.pow(x * 8 + i);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[2] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        }
        for i in (0..y).rev() {
            //south
            _bit_pos = 2_u64.pow(x * 8 + i);
            if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                if (self.pieces[1] & _bit_pos == _bit_pos) | (self.pieces[2] & _bit_pos == _bit_pos)
                {
                    return true;
                } else {
                    break;
                }
            } else if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            }
        }
        false
    }

    /// Returns a vector with all the possible moves for that piece on a specific tile.
    /// Return value wrapped in some. If no possible move exist for the piece an empty vector will be returned.
    /// Input is accepted as the square position eg. "A4" would be the square in the A-file at rank-4.
    ///
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
                Piece::King => possible_moves_to_return
                    .append(&mut self.search_king_moves(is_white, file_coord, rank_coord)),
                Piece::Rook => possible_moves_to_return
                    .append(&mut self.search_rook_moves(is_white, file_coord, rank_coord)),
                Piece::Knight => possible_moves_to_return
                    .append(&mut self.search_knight_moves(is_white, file_coord, rank_coord)),
                Piece::Bishop => possible_moves_to_return
                    .append(&mut self.search_bishop_moves(is_white, file_coord, rank_coord)),
                Piece::Queen => possible_moves_to_return
                    .append(&mut self.search_queen_moves(is_white, file_coord, rank_coord)),
                Piece::Pawn => possible_moves_to_return
                    .append(&mut self.search_pawn_moves(is_white, file_coord, rank_coord)),
            };
            return Some(possible_moves_to_return);
        }
        //throw a exception, no piece on that square.
        Some(vec![String::from("blet")])
    }

    fn would_cause_check(
        &mut self,
        _is_white: bool,
        _from_file: u32,
        _from_rank: u32,
        _to_file: u32,
        _to_rank: u32,
    ) -> bool {
        let _remember_colours = self.colour_of_piece;
        let _remember_pieces = self.pieces;
        self.do_move(_from_file, _from_rank, _to_file, _to_rank);
        let will_cause_check = self.is_colour_in_check(_is_white);
        self.colour_of_piece = _remember_colours;
        self.pieces = _remember_pieces;
        will_cause_check
    }

    fn do_move(&mut self, _from_file: u32, _from_rank: u32, _to_file: u32, _to_rank: u32) {
        //assuming input is valid
        let _bit_pos_to = 2_u64.pow(_to_file * 8 + _to_rank);

        //if there is a capture e.g there is a piece on the to square
        // kill it
        for piece in self.pieces.iter_mut() {
            if *piece & _bit_pos_to == _bit_pos_to {
                self.colour_of_piece[0] &= !_bit_pos_to;
                self.colour_of_piece[1] &= !_bit_pos_to;
                *piece &= !_bit_pos_to;
                break;
            }
        }

        let _bit_pos_from = 2_u64.pow(_from_file * 8 + _from_rank);
        for piece in self.pieces.iter_mut() {
            if *piece & _bit_pos_from == _bit_pos_from {
                if self.colour_of_piece[0] & _bit_pos_from == _bit_pos_from {
                    self.colour_of_piece[0] |= _bit_pos_to;
                } else {
                    self.colour_of_piece[1] |= _bit_pos_to;
                }
                self.colour_of_piece[0] &= !_bit_pos_from;
                self.colour_of_piece[1] &= !_bit_pos_from;
                *piece &= !_bit_pos_from;
                *piece |= _bit_pos_to;
                break;
            }
        }
        //look for pawns promoting
        if (_to_rank == 0) | (_to_rank == 7) {
            if self.pieces[0] & _bit_pos_to == _bit_pos_to {
                // its a pawn promoting.
                self.pieces[self.to_promote_to as usize] |= _bit_pos_to;
                self.pieces[0] &= !_bit_pos_to;
            }
        }
    }

    fn search_queen_moves(
        &mut self,
        _is_white: bool,
        _from_file: u32,
        _from_rank: u32,
    ) -> Vec<String> {
        // I dont know why but i cant combine theese into one line. Probavly means its shaky but i dont know why.
        let mut queen_possible_moves: Vec<String> =
            self.search_bishop_moves(_is_white, _from_file, _from_rank);
        queen_possible_moves.append(&mut self.search_rook_moves(_is_white, _from_file, _from_rank));
        queen_possible_moves
    }
    fn search_king_moves(
        &mut self,
        _is_white: bool,
        _from_file: u32,
        _from_rank: u32,
    ) -> Vec<String> {
        let mut king_possible_moves: Vec<String> = vec![];
        for i in -1..2 {
            for j in -1..2 {
                if (j == 0) & (i == 0) {
                    continue;
                } else {
                    let new_file = _from_file as i32 + i;
                    let new_rank = _from_rank as i32 + j;
                    if (new_file >= 0) & (new_file < 8) & (new_rank >= 0) & (new_rank < 8) {
                        let _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                        if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos
                            != _bit_pos
                        {
                            if !self.would_cause_check(
                                _is_white,
                                _from_file,
                                _from_rank,
                                new_file as u32,
                                new_rank as u32,
                            ) {
                                king_possible_moves
                                    .push(self.transform_back(new_file as u32, new_rank as u32));
                            }
                        }
                    }
                }
            }
        }
        king_possible_moves
    }

    fn search_pawn_moves(
        &mut self,
        _is_white: bool,
        _from_file: u32,
        _from_rank: u32,
    ) -> Vec<String> {
        let mut pawn_possible_moves: Vec<String> = vec![];
        let mut new_rank: u32 = if _is_white {
            _from_rank + 1
        } else {
            _from_rank - 1
        };
        let mut new_file: i32;
        let mut _bit_pos: u64;
        for i in -1..2 {
            new_file = _from_file as i32 + i;
            if (new_file >= 0) & (new_file < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank);
                if i != 0 {
                    if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                        // if i want to add enpassant add here
                        if !self.would_cause_check(
                            _is_white,
                            _from_file,
                            _from_rank,
                            new_file as u32,
                            new_rank,
                        ) {
                            pawn_possible_moves
                                .push(self.transform_back(new_file as u32, new_rank));
                        }
                    }
                } else {
                    if (self.colour_of_piece[0] | self.colour_of_piece[1]) & _bit_pos != _bit_pos {
                        if !self.would_cause_check(
                            _is_white,
                            _from_file,
                            _from_rank,
                            new_file as u32,
                            new_rank,
                        ) {
                            pawn_possible_moves
                                .push(self.transform_back(new_file as u32, new_rank));
                        }
                    }
                    if (if _is_white { 1 } else { 6 }) == _from_rank {
                        if _is_white {
                            new_rank += 1
                        } else {
                            new_rank -= 1
                        };
                        _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank);
                        if (self.colour_of_piece[0] | self.colour_of_piece[1]) & _bit_pos
                            != _bit_pos
                        {
                            if !self.would_cause_check(
                                _is_white,
                                _from_file,
                                _from_rank,
                                new_file as u32,
                                new_rank,
                            ) {
                                pawn_possible_moves
                                    .push(self.transform_back(new_file as u32, new_rank));
                            }
                        }
                    }
                }
            }
        }
        pawn_possible_moves
    }

    fn search_knight_moves(
        &mut self,
        _is_white: bool,
        _from_file: u32,
        _from_rank: u32,
    ) -> Vec<String> {
        let mut knight_possible_moves: Vec<String> = vec![];
        let mut _bit_pos: u64;
        for i in -2..3 {
            for j in -2..3 {
                if (i.max(-i) != j.max(-j)) & (j != 0) & (i != 0) {
                    let new_file = _from_file as i32 + i;
                    let new_rank = _from_rank as i32 + j;
                    if (new_file >= 0) & (new_file < 8) & (new_rank >= 0) & (new_rank < 8) {
                        _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                        if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos
                            != _bit_pos
                        {
                            if !self.would_cause_check(
                                _is_white,
                                _from_file,
                                _from_rank,
                                new_file as u32,
                                new_rank as u32,
                            ) {
                                knight_possible_moves
                                    .push(self.transform_back(new_file as u32, new_rank as u32));
                            }
                        }
                    }
                }
            }
        }
        knight_possible_moves
    }

    fn search_bishop_moves(
        &mut self,
        _is_white: bool,
        _from_file: u32,
        _from_rank: u32,
    ) -> Vec<String> {
        let mut bishop_possible_moves: Vec<String> = vec![];
        let mut _bit_pos: u64;
        let mut new_file: i32;
        let mut new_rank: i32;
        for i in 1..8 {
            // a little bit more brute force but i cant be bothered. going NE
            new_file = _from_file as i32 + i;
            new_rank = _from_rank as i32 + i;
            if (new_file < 8) & (new_rank < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(
                        _is_white,
                        _from_file,
                        _from_rank,
                        new_file as u32,
                        new_rank as u32,
                    ) {
                        bishop_possible_moves
                            .push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        for i in 1..8 {
            // sry for code dupe dont think its avoidable, going NW
            new_file = _from_file as i32 - i;
            new_rank = _from_rank as i32 + i;
            if (new_file >= 0) & (new_rank < 8) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(
                        _is_white,
                        _from_file,
                        _from_rank,
                        new_file as u32,
                        new_rank as u32,
                    ) {
                        bishop_possible_moves
                            .push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        for i in 1..8 {
            // going SE
            new_file = _from_file as i32 + i;
            new_rank = _from_rank as i32 - i;
            if (new_file < 8) & (new_rank >= 0) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(
                        _is_white,
                        _from_file,
                        _from_rank,
                        new_file as u32,
                        new_rank as u32,
                    ) {
                        bishop_possible_moves
                            .push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        for i in 1..8 {
            // going SW
            new_file = _from_file as i32 - i;
            new_rank = _from_rank as i32 - i;
            if (new_file >= 0) & (new_rank >= 0) {
                _bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank as u32);
                if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                    break;
                } else {
                    if !self.would_cause_check(
                        _is_white,
                        _from_file,
                        _from_rank,
                        new_file as u32,
                        new_rank as u32,
                    ) {
                        bishop_possible_moves
                            .push(self.transform_back(new_file as u32, new_rank as u32));
                    }
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    break;
                }
            }
        }
        bishop_possible_moves
    }

    fn search_rook_moves(
        &mut self,
        _is_white: bool,
        _from_file: u32,
        _from_rank: u32,
    ) -> Vec<String> {
        let mut rook_possible_moves: Vec<String> = vec![];
        let mut _bit_pos: u64;
        for i in (1 + _from_file)..8 {
            _bit_pos = 2_u64.pow(i * 8 + _from_rank);
            if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, i, _from_rank) {
                    rook_possible_moves.push(self.transform_back(i, _from_rank));
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    //cant move past pieces
                    break;
                }
            }
        }
        for i in (0.._from_file).rev() {
            _bit_pos = 2_u64.pow(i * 8 + _from_rank);
            if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, i, _from_rank) {
                    rook_possible_moves.push(self.transform_back(i, _from_rank));
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    //cant move past pieces
                    break;
                }
            }
        }
        for i in (1 + _from_rank)..8 {
            _bit_pos = 2_u64.pow(_from_file * 8 + i);
            if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, _from_file, i) {
                    rook_possible_moves.push(self.transform_back(_from_file, i));
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    //cant move past pieces
                    break;
                }
            }
        }
        for i in (0.._from_rank).rev() {
            _bit_pos = 2_u64.pow(_from_file * 8 + i);
            if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                break;
            } else {
                if !self.would_cause_check(_is_white, _from_file, _from_rank, _from_file, i) {
                    rook_possible_moves.push(self.transform_back(_from_file, i));
                }
                if self.colour_of_piece[if _is_white { 1 } else { 0 }] & _bit_pos == _bit_pos {
                    //cant move past pieces
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
        let mut to_debug_print = String::new();
        let mut char_append: char = '*';
        for y in (0..8).rev() {
            for x in 0..8 {
                char_append = '*';
                for (index_of_piece, piece) in self.pieces.iter().enumerate() {
                    let bit_pos = 2_u64.pow(x as u32 * 8 + y as u32);
                    if bit_pos & piece == bit_pos {
                        char_append = match index_of_piece {
                            0 => {
                                if self.colour_of_piece[0] & bit_pos == bit_pos {
                                    'P'
                                } else {
                                    'p'
                                }
                            }
                            1 => {
                                if self.colour_of_piece[0] & bit_pos == bit_pos {
                                    'Q'
                                } else {
                                    'q'
                                }
                            }
                            2 => {
                                if self.colour_of_piece[0] & bit_pos == bit_pos {
                                    'R'
                                } else {
                                    'r'
                                }
                            }
                            3 => {
                                if self.colour_of_piece[0] & bit_pos == bit_pos {
                                    'B'
                                } else {
                                    'b'
                                }
                            }
                            4 => {
                                if self.colour_of_piece[0] & bit_pos == bit_pos {
                                    'N'
                                } else {
                                    'n'
                                }
                            }
                            5 => {
                                if self.colour_of_piece[0] & bit_pos == bit_pos {
                                    'K'
                                } else {
                                    'k'
                                }
                            }
                            _ => '*',
                        };
                        break;
                    }
                }
                to_debug_print.push(char_append);
            }
            to_debug_print.push('\n');
        }
        write!(f, "{}", to_debug_print)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

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
        
        println!("{game:?}");
       

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
    #[test]
    fn test_a_game() {
        let mut game = Game::new();
        assert_eq!(format!("{game:?}"),"rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n");
        assert_eq!(false, game.is_colour_in_check(true));
        assert_eq!(false, game.is_colour_in_check(false));

        game.do_move(3, 0, 3, 7); //do_move works.
        assert_eq!(format!("{game:?}"),"rnbQkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNB*KBNR\n");
        assert_eq!(true ,game.is_colour_in_check(false));
        game.do_move(0, 7, 3, 0);
        assert_eq!(true ,game.is_colour_in_check(true));
        
        game = Game::new();
        assert_eq!(format!("{game:?}"),"rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n");
        assert_eq!(Some(vec![]), game.get_possible_moves("A1"));

    }

    #[test]
    fn test_rook_moving_piece (){
        let mut game = Game::new();
        assert_eq!(format!("{game:?}"),"rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n");
        game.pieces = [0,0,0,0,0,0]; //setting board empty to test moves.
        game.colour_of_piece = [0,0];

        //I need to add kings otherwise my is in check function shits itself.
        game.pieces[5] = 1 + 2_u64.pow(7); // ill just stuff em away in the corner. They are in A1 and A8 cuz 1 = 2.pow(0)
        game.colour_of_piece[0] = 1;
        game.colour_of_piece[1] = 2_u64.pow(7);
        assert_eq!(format!("{game:?}"),"k*******\n********\n********\n********\n********\n********\n********\nK*******\n");


        //rook
        game.pieces[2] = 2_u64.pow(3 * 8 + 3); //D4
        
        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 3); //D4
        assert_eq!(format!("{game:?}"),"k*******\n********\n********\n********\n***R****\n********\n********\nK*******\n");
        assert_eq!(Some(vec![String::from("E4"), String::from("F4"), String::from("G4"), String::from("H4"), 
            String::from("C4"), String::from("B4"), String::from("A4"), 
            String::from("D5"), String::from("D6"), String::from("D7"), String::from("D8"),
            String::from("D3"), String::from("D2"), String::from("D1")]), 
            game.get_possible_moves("D4"));

        //What if i add an enemy piece to block it?
        game.pieces[0] = 2_u64.pow(3 * 8 + 6);
        game.colour_of_piece[1] += 2_u64.pow(3 * 8 + 6);
        assert_eq!(format!("{game:?}"),"k*******\n***p****\n********\n********\n***R****\n********\n********\nK*******\n");
        assert_eq!(Some(vec![String::from("E4"), String::from("F4"), String::from("G4"), String::from("H4"), 
            String::from("C4"), String::from("B4"), String::from("A4"), 
            String::from("D5"), String::from("D6"), String::from("D7"), // D8 is removed from possible.
            String::from("D3"), String::from("D2"), String::from("D1")]), 
            game.get_possible_moves("D4"));

        //what if is friendly instead of enemy?
        game.colour_of_piece[1] -= 2_u64.pow(3 * 8 + 6);
        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 6);
        assert_eq!(format!("{game:?}"),"k*******\n***P****\n********\n********\n***R****\n********\n********\nK*******\n");
        assert_eq!(Some(vec![String::from("E4"), String::from("F4"), String::from("G4"), String::from("H4"), 
            String::from("C4"), String::from("B4"), String::from("A4"), 
            String::from("D5"), String::from("D6"), //D7 is removed from possible....
            String::from("D3"), String::from("D2"), String::from("D1")]), 
            game.get_possible_moves("D4"));

    }

    #[test]
    fn test_bishop_moving_piece (){
        let mut game = Game::new();
        assert_eq!(format!("{game:?}"),"rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n");
        game.pieces = [0,0,0,0,0,0]; //setting board empty to test moves.
        game.colour_of_piece = [0,0];

        //I need to add kings otherwise my is in check function shits itself.
        game.pieces[5] = 1 + 2_u64.pow(7); // ill just stuff em away in the corner. They are in A1 and A8 cuz 1 = 2.pow(0)
        game.colour_of_piece[0] = 1;
        game.colour_of_piece[1] = 2_u64.pow(7);
        assert_eq!(format!("{game:?}"),"k*******\n********\n********\n********\n********\n********\n********\nK*******\n");


        //bishop
        game.pieces[3] = 2_u64.pow(3 * 8 + 3); //D4
        
        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 3); //D4
        assert_eq!(format!("{game:?}"),"k*******\n********\n********\n********\n***B****\n********\n********\nK*******\n");
        assert_eq!(Some(vec![String::from("E5"), String::from("F6"), String::from("G7"), String::from("H8"), 
            String::from("C5"), String::from("B6"), String::from("A7"), 
            String::from("E3"), String::from("F2"), String::from("G1"),
            String::from("C3"), String::from("B2")]), 
            game.get_possible_moves("D4"));

        //Lets shift it one step up so we can see how it behaves.
        game.do_move(3, 3, 3, 4);
        assert_eq!(format!("{game:?}"),"k*******\n********\n********\n***B****\n********\n********\n********\nK*******\n");
        assert_eq!(Some(vec![String::from("E6"), String::from("F7"), String::from("G8"), 
            String::from("C6"), String::from("B7"), String::from("A8"), // btw black king on A8 
            String::from("E4"), String::from("F3"), String::from("G2"), String::from("H1"),
            String::from("C4"), String::from("B3"), String::from("A2")]), 
            game.get_possible_moves("D5"));
        assert_eq!(true, game.is_colour_in_check(false)); // since the king is on A8

    }

}
