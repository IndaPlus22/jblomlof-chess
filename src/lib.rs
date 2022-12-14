use std::{fmt, vec};

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

/* IMPORTANT:
 * - Document well!
 * - Write well and clean code!
 */
pub struct Game {
    /* save board, active colour, ... */

    /* The board will be divided like this. It will count from 0 to 63. The square A1 is 0, A2 is 1,
    A3 is 2... B1 is 8, B2 is 9...C1 is 16... D1 is 24... H1 is 56 ... H8 is 63 */
    state: GameState,
    white_turn: bool,
    colour_of_piece: [u64; 2], //white 0-index and black 1-index
    pieces: [u64; 6],
    to_promote_to: u8, // queen 1, rook 2, bishop 3, knight 4,
    en_passant_at: u8, //will tell which square enpassant is possible e.g if the move is E7->E5 then it'll mark E6
                       //I can be smart. I know enpassants are only possible on rank-3(y==2) and rank-6(y==5) thus i can save on memory
                       //and only use 0-15 as squares and 16+ as not possible.
                       //this one will count the opposite way, eg 3 will indicate D3 and 11 will indicate D6
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            white_turn: true,
            en_passant_at: 16,
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

    ///Returns a long string of the board.
    /// It will represent the board with lines.
    /// Each line will make one rank and go from left to right.
    /// Black pieces will be represented by lowercase characters and White pieces by uppercase characters
    /// Each piece will be represented by one character.
    /// Kings -> K/k
    /// Queen -> Q/q
    /// Pawn -> P/p
    /// Bishop -> B/b
    /// Knight -> N/n
    /// Rook -> R/r
    /// Empty spots will be represented using *
    /// For example the starting board will be
    /// "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR"
    /// Note: \n means newline
    pub fn get_board(&self) -> String {
        let mut board_state = String::new();
        let mut char_append: char;
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
                board_state.push(char_append);
            }
            board_state.push('\n');
        }
        board_state.pop();
        board_state
    }

    //Returns true if it is white to move. Returns false otherwise meaning its black to move
    pub fn is_white_turn(&self) -> bool {
        self.white_turn
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

            //Why am i repeating computation task?
            // I should do
            let white_check_or_mate = self.colour_in_check_or_mate(true);
            let black_check_or_mate = self.colour_in_check_or_mate(false);

            if (white_check_or_mate == 2) | (black_check_or_mate == 2) {
                self.state = GameState::GameOver;
            } else if (white_check_or_mate == 1) | (black_check_or_mate == 1) {
                self.state = GameState::Check;
            } else {
                self.state = GameState::InProgress;
            }
            self.en_passant_at = 16; // 16 means no enpassant

            //was it a enpassant move coming?
            let bit_pos = 2_u64.pow(to_file * 8 + to_rank);
            match self.get_that_piece_type(bit_pos) {
                Piece::Pawn => {
                    if from_rank.abs_diff(to_rank) == 2 {
                        if from_rank < to_rank {
                            // this means a white pawn was moving.
                            self.en_passant_at = from_file as u8;
                        } else {
                            self.en_passant_at = from_file as u8 + 8;
                        }
                    }
                }
                _ => (),
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
    /// Values avaiable are inProgress, Check, GameOver(which is checkmate right now)
    pub fn get_game_state(&mut self) -> GameState {
        //Ugly but hopefully works, i just repeat the check for mate thing.
        let white_check_or_mate = self.colour_in_check_or_mate(true);
        let black_check_or_mate = self.colour_in_check_or_mate(false);

        if (white_check_or_mate == 2) | (black_check_or_mate == 2) {
            self.state = GameState::GameOver;
        } else if (white_check_or_mate == 1) | (black_check_or_mate == 1) {
            self.state = GameState::Check;
        } else {
            self.state = GameState::InProgress;
        }
        self.state
    }

    /// Returns u32.
    /// Returns 1 if the specified colour is in check. returns 0 if the specified colour is not in check and returns 2 if the specified colour is in checkmate
    /// Input is boolean, true if the desired colour to see if in check is white. False if the desired colour to see if in check is black.
    pub fn colour_in_check_or_mate(&mut self, _is_white: bool) -> u32 {
        // returning 0 if not check nor mate, 1 if check, 2 if mate
        let (x, y) = self.get_king_pos(_is_white);
        if self.helper_colour_in_check(_is_white, x, y) {
            // some sort of check
            if self.no_valid_moves_for_colour(_is_white) {
                return 2;
            }

            return 1;
        }
        return 0;
    }

    /// Returns a option<vector> with all the possible valid moves for that piece on a specific tile.
    /// Return value wrapped in some. If no possible move exist for the piece an empty vector will be returned.
    /// Input is accepted as the square position eg. "A4" would be the square in the A-file at rank-4.
    /// If no piece exist on the input square it returns none.
    ///
    /// (Not done) (en passent done) (optional) Don't forget to include en passent and castling.
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
        None
    }

    fn no_valid_moves_for_colour(&mut self, _is_white: bool) -> bool {
        let mut _bit_pos: u64;
        for _bit_index in 0..64 {
            _bit_pos = 2_u64.pow(_bit_index);
            if self.colour_of_piece[if _is_white { 0 } else { 1 }] & _bit_pos == _bit_pos {
                if self
                    .get_possible_moves(&self.transform_back(_bit_index / 8, _bit_index % 8))
                    .unwrap()
                    .len()
                    > 0
                {
                    return false;
                }
            }
        }
        true
    }

    fn helper_colour_in_check(&self, _is_white: bool, _x: u32, _y: u32) -> bool {
        self.is_bishop_logic_threat(_is_white, _x, _y)
            | self.is_rook_logic_threat(_is_white, _x, _y)
            | self.is_knight_logic_threat(_is_white, _x, _y)
            | self.is_pawn_logic_threat(_is_white, _x, _y)
            | self.is_king_logic_threat(_is_white, _x, _y)
    }

    fn get_king_pos(&self, _is_white: bool) -> (u32, u32) {
        let mut king_index: u32 = 0; // will be iterated throu to find the value.
        let mut bit_pos: u64;
        loop {
            bit_pos = 2_u64.pow(king_index);
            if self.pieces[5] & bit_pos == bit_pos {
                if self.colour_of_piece[if _is_white { 0 } else { 1 }] & bit_pos == bit_pos {
                    //found the correct coloured king
                    return (king_index / 8, king_index % 8);
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
        let (x, y) = self.get_king_pos(_is_white);
        let will_cause_check = self.helper_colour_in_check(_is_white, x, y);
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

        //look for en_passant
        if (self.en_passant_at < 16)
            & (2_u64.pow(
                (self.en_passant_at % 8) as u32 * 8 + if self.en_passant_at < 8 { 2 } else { 5 },
            ) & _bit_pos_to
                == _bit_pos_to)
        {
            match self.get_that_piece_type(2_u64.pow(_from_file * 8 + _from_rank)) {
                Piece::Pawn => {
                    if self.is_white(_from_file, _from_rank) {
                        // the attacking piece is white thus kill black
                        self.colour_of_piece[1] &= !2_u64.pow(_to_file * 8 + 4);
                        self.pieces[0] &= !2_u64.pow(_to_file * 8 + 4); // 4 because the pawn always jumps to rank5
                    } else {
                        // kill white
                        self.colour_of_piece[0] &= !2_u64.pow(_to_file * 8 + 3);
                        self.pieces[0] &= !2_u64.pow(_to_file * 8 + 3); //3 since white pawns will get enpassanted on rank 4
                    }
                }
                _ => (),
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
        let new_rank: u32 = if _is_white {
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
                    } else if (self.en_passant_at < 16)
                        & (_bit_pos
                            == _bit_pos
                                & 2_u64.pow(
                                    (self.en_passant_at as u32 % 8) * 8
                                        + if self.en_passant_at <= 7 { 2 } else { 5 },
                                ))
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
                        let _temp_bit_pos = 2_u64.pow(new_file as u32 * 8 + new_rank);
                        let _double_move_new_rank;
                        if _is_white {
                            _double_move_new_rank = new_rank + 1;
                        } else {
                            _double_move_new_rank = new_rank - 1;
                        };
                        _bit_pos = 2_u64.pow(new_file as u32 * 8 + _double_move_new_rank);

                        if ((self.colour_of_piece[0] | self.colour_of_piece[1]) & _bit_pos
                            != _bit_pos)
                            & ((self.colour_of_piece[0] | self.colour_of_piece[1]) & _temp_bit_pos
                                != _temp_bit_pos)
                        {
                            if !self.would_cause_check(
                                _is_white,
                                _from_file,
                                _from_rank,
                                new_file as u32,
                                _double_move_new_rank,
                            ) {
                                pawn_possible_moves.push(
                                    self.transform_back(new_file as u32, _double_move_new_rank),
                                );
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

    fn transform_input(&self, input_pos: &str) -> (u32, u32) {
        let mut chars_iter = input_pos.chars();
        (
            chars_iter
                .next()
                .unwrap()
                .to_digit(18 /*Any number > 17 works */)
                .unwrap()
                - 10,
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
        let mut char_append: char;
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
        let mut game = Game::new();

        println!("{game:?}");

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
    #[test]
    fn test_a_game() {
        let mut game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );
        assert_eq!(0, game.colour_in_check_or_mate(true));
        assert_eq!(0, game.colour_in_check_or_mate(false));

        game.do_move(3, 0, 3, 7); //do_move works.
        assert_eq!(
            format!("{game:?}"),
            "rnbQkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNB*KBNR\n"
        );
        assert_eq!(1, game.colour_in_check_or_mate(false));
        game.do_move(0, 7, 3, 0);
        assert_eq!(1, game.colour_in_check_or_mate(true));

        game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );
        assert_eq!(Some(vec![]), game.get_possible_moves("A1"));
    }

    #[test]
    fn test_rook_moving_piece() {
        let mut game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );
        game.pieces = [0, 0, 0, 0, 0, 0]; //setting board empty to test moves.
        game.colour_of_piece = [0, 0];

        //I need to add kings otherwise my is in check function shits itself.
        game.pieces[5] = 1 + 2_u64.pow(7); // ill just stuff em away in the corner. They are in A1 and A8 cuz 1 = 2.pow(0)
        game.colour_of_piece[0] = 1;
        game.colour_of_piece[1] = 2_u64.pow(7);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n********\n********\n********\nK*******\n"
        );

        //rook
        game.pieces[2] = 2_u64.pow(3 * 8 + 3); //D4

        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 3); //D4
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n***R****\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("E4"),
                String::from("F4"),
                String::from("G4"),
                String::from("H4"),
                String::from("C4"),
                String::from("B4"),
                String::from("A4"),
                String::from("D5"),
                String::from("D6"),
                String::from("D7"),
                String::from("D8"),
                String::from("D3"),
                String::from("D2"),
                String::from("D1")
            ]),
            game.get_possible_moves("D4")
        );

        //What if i add an enemy piece to block it?
        game.pieces[0] = 2_u64.pow(3 * 8 + 6);
        game.colour_of_piece[1] += 2_u64.pow(3 * 8 + 6);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n***p****\n********\n********\n***R****\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("E4"),
                String::from("F4"),
                String::from("G4"),
                String::from("H4"),
                String::from("C4"),
                String::from("B4"),
                String::from("A4"),
                String::from("D5"),
                String::from("D6"),
                String::from("D7"), // D8 is removed from possible.
                String::from("D3"),
                String::from("D2"),
                String::from("D1")
            ]),
            game.get_possible_moves("D4")
        );

        //what if is friendly instead of enemy?
        game.colour_of_piece[1] -= 2_u64.pow(3 * 8 + 6);
        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 6);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n***P****\n********\n********\n***R****\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("E4"),
                String::from("F4"),
                String::from("G4"),
                String::from("H4"),
                String::from("C4"),
                String::from("B4"),
                String::from("A4"),
                String::from("D5"),
                String::from("D6"), //D7 is removed from possible....
                String::from("D3"),
                String::from("D2"),
                String::from("D1")
            ]),
            game.get_possible_moves("D4")
        );
    }

    #[test]
    fn test_bishop_moving_piece() {
        let mut game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );
        game.pieces = [0, 0, 0, 0, 0, 0]; //setting board empty to test moves.
        game.colour_of_piece = [0, 0];

        //I need to add kings otherwise my is in check function shits itself.
        game.pieces[5] = 1 + 2_u64.pow(7); // ill just stuff em away in the corner. They are in A1 and A8 cuz 1 = 2.pow(0)
        game.colour_of_piece[0] = 1;
        game.colour_of_piece[1] = 2_u64.pow(7);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n********\n********\n********\nK*******\n"
        );

        //bishop
        game.pieces[3] = 2_u64.pow(3 * 8 + 3); //D4

        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 3); //D4
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n***B****\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("E5"),
                String::from("F6"),
                String::from("G7"),
                String::from("H8"),
                String::from("C5"),
                String::from("B6"),
                String::from("A7"),
                String::from("E3"),
                String::from("F2"),
                String::from("G1"),
                String::from("C3"),
                String::from("B2")
            ]),
            game.get_possible_moves("D4")
        );
        assert_eq!(0, game.colour_in_check_or_mate(false));

        //Lets shift it one step up so we can see how it behaves.
        game.do_move(3, 3, 3, 4);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n***B****\n********\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("E6"),
                String::from("F7"),
                String::from("G8"),
                String::from("C6"),
                String::from("B7"),
                String::from("A8"), // btw black king on A8
                String::from("E4"),
                String::from("F3"),
                String::from("G2"),
                String::from("H1"),
                String::from("C4"),
                String::from("B3"),
                String::from("A2")
            ]),
            game.get_possible_moves("D5")
        );
        assert_eq!(1, game.colour_in_check_or_mate(false)); // since the king is on A8

        //Lets move the kings such that the are not on the edge
        //To see if they block movement.
        game.do_move(0, 0, 1, 2);
        game.do_move(0, 7, 1, 6);
        assert_eq!(
            format!("{game:?}"),
            "********\n*k******\n********\n***B****\n********\n*K******\n********\n********\n"
        );
        assert_eq!(
            Some(vec![
                String::from("E6"),
                String::from("F7"),
                String::from("G8"),
                String::from("C6"),
                String::from("B7"), // btw black king on B7 thus A8 not valid move
                String::from("E4"),
                String::from("F3"),
                String::from("G2"),
                String::from("H1"),
                String::from("C4")
            ]),
            game.get_possible_moves("D5")
        );
        assert_eq!(1, game.colour_in_check_or_mate(false)); // since the king is on B7
    }

    #[test]
    fn test_queen_moving_piece() {
        let mut game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );
        game.pieces = [0, 0, 0, 0, 0, 0]; //setting board empty to test moves.
        game.colour_of_piece = [0, 0];

        //I need to add kings otherwise my is in check function shits itself.
        game.pieces[5] = 1 + 2_u64.pow(7); // ill just stuff em away in the corner. They are in A1 and A8 cuz 1 = 2.pow(0)
        game.colour_of_piece[0] = 1;
        game.colour_of_piece[1] = 2_u64.pow(7);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n********\n********\n********\nK*******\n"
        );

        //Queen
        //no need to do a big test since its basically just rook moves and bishop moves combined.
        game.pieces[1] = 2_u64.pow(3 * 8 + 3); //D4
        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 3); //D4
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n***Q****\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("E5"),
                String::from("F6"),
                String::from("G7"),
                String::from("H8"),
                String::from("C5"),
                String::from("B6"),
                String::from("A7"),
                String::from("E3"),
                String::from("F2"),
                String::from("G1"),
                String::from("C3"),
                String::from("B2"),
                String::from("E4"),
                String::from("F4"),
                String::from("G4"),
                String::from("H4"),
                String::from("C4"),
                String::from("B4"),
                String::from("A4"),
                String::from("D5"),
                String::from("D6"),
                String::from("D7"),
                String::from("D8"),
                String::from("D3"),
                String::from("D2"),
                String::from("D1")
            ]),
            game.get_possible_moves("D4")
        ); //Exactly bishop + rook moves.
        assert_eq!(0, game.colour_in_check_or_mate(false));
    }

    #[test]
    fn test_knight_moving_piece() {
        let mut game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );
        game.pieces = [0, 0, 0, 0, 0, 0]; //setting board empty to test moves.
        game.colour_of_piece = [0, 0];

        //I need to add kings otherwise my is in check function shits itself.
        game.pieces[5] = 1 + 2_u64.pow(7); // ill just stuff em away in the corner. They are in A1 and A8 cuz 1 = 2.pow(0)
        game.colour_of_piece[0] = 1;
        game.colour_of_piece[1] = 2_u64.pow(7);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n********\n********\n********\nK*******\n"
        );

        //adding a knight to D4
        game.pieces[4] = 2_u64.pow(3 * 8 + 3); //D4
        game.colour_of_piece[0] += 2_u64.pow(3 * 8 + 3); //D4
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n***N****\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("B3"),
                String::from("B5"),
                String::from("C2"),
                String::from("C6"),
                String::from("E2"),
                String::from("E6"),
                String::from("F3"),
                String::from("F5"),
            ]),
            game.get_possible_moves("d4") //small letters work as input. Altough will always return uppercase
        );
        assert_eq!(0, game.colour_in_check_or_mate(false));

        //Lets test if it still works with a piece there.
        game.do_move(0, 7, 5, 4); // the black king to F5
        assert_eq!(
            format!("{game:?}"),
            "********\n********\n********\n*****k**\n***N****\n********\n********\nK*******\n"
        );
        assert_eq!(
            Some(vec![
                String::from("B3"),
                String::from("B5"),
                String::from("C2"),
                String::from("C6"),
                String::from("E2"),
                String::from("E6"),
                String::from("F3"),
                String::from("F5"),
            ]),
            game.get_possible_moves("D4")
        );
        assert_eq!(1, game.colour_in_check_or_mate(false));

        //now put the white king on a tile the knight could jump to.
        game.do_move(0, 0, 1, 2); //B3
        assert_eq!(
            format!("{game:?}"),
            "********\n********\n********\n*****k**\n***N****\n*K******\n********\n********\n"
        );
        assert_eq!(
            Some(vec![
                //String::from("B3"), //cant jump to a piece of same colour
                String::from("B5"),
                String::from("C2"),
                String::from("C6"),
                String::from("E2"),
                String::from("E6"),
                String::from("F3"),
                String::from("F5"),
            ]),
            game.get_possible_moves("D4")
        );
        assert_eq!(1, game.colour_in_check_or_mate(false));
    }

    #[test]
    fn test_mate() {
        let mut game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );

        game.do_move(3, 0, 3, 7);
        assert_eq!(
            format!("{game:?}"),
            "rnbQkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNB*KBNR\n"
        );
        assert_eq!(1, game.colour_in_check_or_mate(false)); // only check since black king can capture white queen.
        assert_eq!(0, game.colour_in_check_or_mate(true));
        game.do_move(0, 0, 2, 7);
        assert_eq!(
            format!("{game:?}"),
            "rnRQkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\n*NB*KBNR\n"
        );
        assert_eq!(2, game.colour_in_check_or_mate(false)); //mate since rook is defening queen
        assert_eq!(0, game.colour_in_check_or_mate(true));

        //now repeat for opposite colour
        game = Game::new();
        assert_eq!(
            format!("{game:?}"),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR\n"
        );

        game.do_move(3, 7, 3, 0);
        assert_eq!(
            format!("{game:?}"),
            "rnb*kbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBqKBNR\n"
        );
        assert_eq!(1, game.colour_in_check_or_mate(true)); // only check since black king can capture white queen.
        assert_eq!(0, game.colour_in_check_or_mate(false));
        game.do_move(2, 7, 2, 1);
        assert_eq!(
            format!("{game:?}"),
            "rn**kbnr\npppppppp\n********\n********\n********\n********\nPPbPPPPP\nRNBqKBNR\n"
        );
        assert_eq!(2, game.colour_in_check_or_mate(true)); //mate since bishop is defening queen
        assert_eq!(0, game.colour_in_check_or_mate(false));
    }

    #[test]
    fn testing_a_real_game_using_only_public_func() {
        let mut game = Game::new();
        assert_eq!(
            game.get_board(),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR"
        );
        assert_eq!(true, game.make_move("E2", "E4"));
        assert_eq!(
            game.get_board(),
            "rnbqkbnr\npppppppp\n********\n********\n****P***\n********\nPPPP*PPP\nRNBQKBNR"
        );
        assert_eq!(true, game.make_move("E7", "E5"));
        assert_eq!(
            game.get_board(),
            "rnbqkbnr\npppp*ppp\n********\n****p***\n****P***\n********\nPPPP*PPP\nRNBQKBNR"
        );

        //trying a move thats illegal
        assert_eq!(false, game.make_move("D7", "D5")); //its white turn not black
        assert_eq!(
            game.get_board(),
            "rnbqkbnr\npppp*ppp\n********\n****p***\n****P***\n********\nPPPP*PPP\nRNBQKBNR"
        );
        //lets try a white piece doing an illegal move
        assert_eq!(false, game.make_move("A1", "A3"));
        assert_eq!(
            game.get_board(),
            "rnbqkbnr\npppp*ppp\n********\n****p***\n****P***\n********\nPPPP*PPP\nRNBQKBNR"
        );

        //back to real moves.
        assert_eq!(true, game.make_move("G1", "F3"));
        assert_eq!(
            game.get_board(),
            "rnbqkbnr\npppp*ppp\n********\n****p***\n****P***\n*****N**\nPPPP*PPP\nRNBQKB*R"
        );
        assert_eq!(true, game.make_move("F8", "C5"));
        assert_eq!(
            game.get_board(),
            "rnbqk*nr\npppp*ppp\n********\n**b*p***\n****P***\n*****N**\nPPPP*PPP\nRNBQKB*R"
        );
        //and so on.
        assert_eq!(true, game.make_move("F3", "E5"));
        assert_eq!(true, game.make_move("D8", "H4"));
        assert_eq!(true, game.make_move("E5", "D3"));
        assert_eq!(true, game.make_move("H4", "F2"));
        assert_eq!(1, game.colour_in_check_or_mate(true));
        assert_eq!(0, game.colour_in_check_or_mate(false));
        assert_eq!(game.get_game_state(), GameState::Check);
        assert_eq!(
            Some(vec![String::from("F2")]),
            game.get_possible_moves("D3")
        );
        assert_eq!(true, game.make_move("D3", "F2"));
        assert_eq!(game.get_game_state(), GameState::InProgress);

        assert_eq!(true, game.make_move("E8", "F8"));
        assert_eq!(true, game.make_move("E4", "E5"));
        assert_eq!(true, game.make_move("D7", "D5"));
        assert_eq!(game.en_passant_at, 11);
        assert_eq!(true, game.make_move("E5", "D6")); // enpassant thus proving it worked.
        assert_eq!(true, game.make_move("C5", "B4"));
        assert_eq!(true, game.make_move("D6", "D7"));
        assert_eq!(true, game.make_move("G8", "H6"));
        assert_eq!(true, game.make_move("D7", "D8"));
        assert_eq!(game.get_game_state(), GameState::GameOver);
        assert_eq!(
            game.get_board(),
            "rnbQ*k*r\nppp**ppp\n*******n\n********\n*b******\n********\nPPPP*NPP\nRNBQKB*R"
        );
        //this shows promotion works. altough not yet shown game.set_promotion()

        //i'll simply show it in another function.
    }

    #[test]
    fn test_set_promotion() {
        let mut game = Game::new();
        game.pieces = [0, 0, 0, 0, 0, 0];
        game.colour_of_piece = [0, 0];
        game.pieces[5] = 1 + 2_u64.pow(7); // ill just stuff em away in the corner. They are in A1 and A8 cuz 1 = 2.pow(0)
        game.colour_of_piece[0] = 1;
        game.colour_of_piece[1] = 2_u64.pow(7);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n********\n********\n********\n********\n********\n********\nK*******\n"
        );

        game.pieces[0] = 2_u64.pow(62); //H7
        game.colour_of_piece[0] += 2_u64.pow(62);
        assert_eq!(
            format!("{game:?}"),
            "k*******\n*******P\n********\n********\n********\n********\n********\nK*******\n"
        );
        game.set_promotion("rook");
        assert_eq!(game.is_white_turn(), true);
        assert_eq!(true, game.make_move("H7", "H8"));
        assert_eq!(
            format!("{game:?}"),
            "k******R\n********\n********\n********\n********\n********\n********\nK*******\n"
        );
        assert_eq!(1, game.colour_in_check_or_mate(false));
    }

    #[test]
    fn test_bug() {
        let mut game = Game::new();
        assert_eq!(
            game.get_board(),
            "rnbqkbnr\npppppppp\n********\n********\n********\n********\nPPPPPPPP\nRNBQKBNR"
        );
        assert_eq!(true, game.make_move("E2", "E4"));
        assert_eq!(true, game.make_move("F7", "F5"));
        assert_eq!(true, game.make_move("D1", "E2"));
        assert_eq!(true, game.make_move("G7", "G5"));
        assert_eq!(true, game.make_move("E2", "H5"));
        assert_eq!(2, game.colour_in_check_or_mate(false));
        //assert_eq!(1, game.colour_in_check_or_mate(true));
        // assert_eq!(1, game.colour_in_check_or_mate(false));

        let white_check_or_mate = game.colour_in_check_or_mate(true);
        let black_check_or_mate = game.colour_in_check_or_mate(false);
        assert_eq!(
            (white_check_or_mate == 2) | (black_check_or_mate == 2),
            true
        );
        assert_eq!(game.get_game_state(), GameState::GameOver);
    }
}
