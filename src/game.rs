use rand::Rng;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Game {
	board: Board,
}

impl Game {
	pub fn new(size: u8, mines: u16) -> Game {
		let board = create_board(size, mines);
		Game { board }
	}
}

pub type Board = Vec<Vec<Cell>>;

#[derive(Serialize, Debug)]
pub struct Cell {
	state: CellState,
	mine: bool,
}

#[derive(Serialize, Debug)]
pub enum CellState {
	Default,
	Revealed(Option<u8>),
	Flagged,
}

fn create_board(size: u8, mines: u16) -> Vec<Vec<Cell>> {
	let mut board: Vec<Vec<Cell>> = Vec::new();

	for _ in 0..size {
		let mut row: Vec<Cell> = Vec::new();

		for _ in 0..size {
			row.push(Cell {
				state: CellState::Default,
				mine: false,
			});
		}

		board.push(row);
	}

	let mut mines_left = mines;

	while mines_left > 0 {
		let x: usize = rand::thread_rng().gen_range(0..size).into();
		let y: usize = rand::thread_rng().gen_range(0..size).into();

		let cell = &mut board[y][x];

		if !cell.mine {
			cell.mine = true;
			mines_left -= 1;
		}
	}

	board
}
