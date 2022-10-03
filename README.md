Hello Welcome!
This is a very complicated piece of shit hehe.
I havent implemented castling because i dont want to.
The features this build includes are:
Full piece movement including Enpassant (But excluding castling)
Checks and checkmate checks. (Pun intended)
well thats about it.

Ok for functionallity we have some functions

(official docs)

Game::new() :
Creates a game obj(?) and sets starting position for all pieces.
e.g. do let game = Game::new()

Game::get_board() : 
Returns a long string of the board.
It will represent the board with lines. Each line will make one rank and go from left to right.
Black pieces will be represented by lowercase characters and White pieces by uppercase characters
Each piece will be represented by one character. Kings -> K/k. Queen -> Q/q. Pawn -> P/p. Bishop -> B/b. Knight -> N/n. Rook -> R/r. Empty spots will be represented using *
For example the starting board will be in this file i cant write * or ir cries so instead ill do X
 \"rnbqkbnr\npppppppp\nXXXXXXXX\nXXXXXXXX\nXXXXXXXX\nXXXXXXXX\nPPPPPPPP\nRNBQKBNR" 
    Note: \n means newline if you cant see the newline character thats on me and dont worry.

Game::is_white_turn() : 
Returns true if it is white to move. Returns false otherwise meaning its black to move

Game::get_game_state() :
Return the GameState
(One of the following values
GameState::InProgress -> meaning game is normal and no one in check or mate
GameState::Check -> one is in check
GameState::GameOver -> one is in mate.
)

Game::colour_in_check_or_mate(is_colour_white) :
takes one bool arg. input true if you want to know if White is in check or mate,
input false if you want to know if black is in mate or check
Returns an u32. If the value returned is 0 then the colour/player is not in check nor mate
If the returned value is 1 then the player is in check
If the returned value is 2 then the player is in mate thus lost.

Game::make_move(from, to) : 
Will take two (2) &str as input. First one from square and the seconds one to square. Written as "fileRank"
e.g "A4" is a square.
If its legal and valid for the piece standing on the from square to move to the "to square" it will do so and return true
otherwise return false and dont do anything.

Game::get_possible_moves(square) : 
Takes a square in type &str as input in "fileRank" returns a vector wrapped in some containing all valid moves. Theese moves are strings following the "fileRank" rule. Returns None if there was no piece on the input square.

Game::set_promotion(piece) :
this sets what type of piece a pawn becomes when promoting. at standard its queen.
Input should be a &str with the name of the piece. One letter acronym is ok aswell.
I.e if you want to set promotion to knight then the valid input is
"knight" "Knight" "KNIGHT" "N" "n" - same for other pieces.
Will assume is queen if input is badly written.

Game


