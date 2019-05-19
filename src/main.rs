mod board;
use board::Board;

fn main() {
	let board = Board::read("tests/board.txt");
	board.print();
}
