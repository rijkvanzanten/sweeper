use rand::Rng;
use serde::Serialize;

/// Game board. Holds every tile and it's state
#[derive(Serialize)]
pub struct Board {
	width: usize,
	tiles: Vec<Tile>,
}

impl Board {
	pub fn new(width: usize, height: usize, mines: usize) -> Board {
		let mut board = Board {
			width,
			tiles: vec![Tile::default(); width * height],
		};

		let mut mines_left = mines;

		while mines_left > 0 {
			let row: usize = rand::thread_rng().gen_range(0..height);
			let col: usize = rand::thread_rng().gen_range(0..width);

			let tile = &mut board[row][col];

			if !tile.mine {
				tile.mine = true;
				mines_left -= 1;

				for (adjacent_row, adjacent_col) in board.adjacent(row, col) {
					board[adjacent_row][adjacent_col].num_adjacent += 1;
				}
			}
		}

		board
	}

	pub fn rows(&self) -> Vec<Vec<Tile>> {
		self.tiles
			.chunks(self.width)
			.map(|chunk| chunk.to_vec())
			.collect::<Vec<Vec<Tile>>>()
	}

	/// Silently ignores attempts at flagging revealed tiles
	pub fn toggle_flag(&mut self, row: usize, col: usize) {
		let tile = &mut self[row][col];

		if let TileState::Flagged = tile.state {
			tile.state = TileState::Default;
		} else if let TileState::Default = tile.state {
			tile.state = TileState::Flagged;
		}
	}

	pub fn reveal(&mut self, row: usize, col: usize) {
		let tile = &mut self[row][col];

		if tile.mine {
			self.game_over()
		} else if let TileState::Default = tile.state {
			tile.state = TileState::Revealed;

			if tile.num_adjacent == 0 {
				for (adjacent_row, adjacent_col) in self.adjacent(row, col) {
					self.reveal(adjacent_row, adjacent_col);
				}
			}
		}
	}

	fn game_over(&mut self) {
		for tile in self.tiles.iter_mut() {
			tile.state = TileState::Revealed;
		}
	}

	pub fn width(&self) -> usize {
		self.width
	}

	fn adjacent(&mut self, row: usize, col: usize) -> Vec<(usize, usize)> {
		let mut coords: Vec<(usize, usize)> = vec![];

		let cols = self.width;
		let rows = self.tiles.len() / self.width;

		let top_exists = row > 0;
		let right_exists = col < cols - 1;
		let bottom_exists = row < rows - 1;
		let left_exists = col > 0;

		if top_exists {
			coords.push((row - 1, col));

			if left_exists {
				coords.push((row - 1, col - 1));
			}

			if right_exists {
				coords.push((row - 1, col + 1));
			}
		}

		if left_exists {
			coords.push((row, col - 1));
		}

		if right_exists {
			coords.push((row, col + 1));
		}

		if bottom_exists {
			coords.push((row + 1, col));

			if left_exists {
				coords.push((row + 1, col - 1));
			}

			if right_exists {
				coords.push((row + 1, col + 1));
			}
		}

		coords
	}
}

impl std::ops::Index<usize> for Board {
	type Output = [Tile];
	fn index(&self, row: usize) -> &[Tile] {
		let start = row * self.width;
		&self.tiles[start..start + self.width]
	}
}

impl std::ops::IndexMut<usize> for Board {
	fn index_mut(&mut self, row: usize) -> &mut [Tile] {
		let start = row * self.width;
		&mut self.tiles[start..start + self.width]
	}
}

#[derive(Clone, Serialize)]
pub struct Tile {
	mine: bool,
	num_adjacent: u8,
	state: TileState,
}

impl Default for Tile {
	fn default() -> Self {
		Tile {
			mine: false,
			num_adjacent: 0,
			state: TileState::Default,
		}
	}
}

#[derive(Clone, Serialize)]
pub enum TileState {
	Default,
	Flagged,
	Revealed,
}
