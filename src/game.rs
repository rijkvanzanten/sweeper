use rand::Rng;
use crate::utils::gen_id;

/// Holds the current game state and cells on the board. The cells are stored as a 1d vector to
/// optimize performance, but can be indexed in a 2d fashion
///
/// ```
/// let game = Game::new(100, 100);
///
/// for row of game {
///   for cell of game {
///     assert_eq!(cell.mine )
///   }
/// }
/// ```
pub struct Game {
   id: String,
	width: usize,
	cells: Vec<Cell>,
}

impl Game {
	pub fn new(width: usize, height: usize, mines: usize) -> Game {
		let mut game = Game {
         id: gen_id(),
			width,
			cells: vec![Cell::default(); width * height],
		};

		let mut mines_left = mines;

		while mines_left > 0 {
			let x: usize = rand::thread_rng().gen_range(0..width);
			let y: usize = rand::thread_rng().gen_range(0..height);

			let index = x + y * width;

			let cell = &mut game.cells[index];

			if !cell.mine {
				cell.mine = true;
				mines_left -= 1;
			}
		}

		game
	}

	pub fn rows(&self) -> std::slice::Chunks<Cell> {
		self.cells.chunks(self.width)
	}

   pub fn id(&self) -> &String {
      &self.id
   }
}

#[derive(Clone)]
struct Cell {
	mine: bool,
	num_adjacent: u8,
	state: CellState,
}

impl Default for Cell {
	fn default() -> Self {
		Cell {
			mine: false,
			num_adjacent: 0,
			state: CellState::Default,
		}
	}
}

#[derive(Clone)]
enum CellState {
	Default,
	Flagged,
	Visible,
}
