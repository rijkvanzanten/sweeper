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

				board.adjacent(row, col, |tile| {
					tile.num_adjacent += 1;
				});
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

	pub fn width(&self) -> usize {
		self.width
	}

	fn adjacent<C>(&mut self, row: usize, col: usize, callback: C)
	where
		C: Fn(&mut Tile),
	{
		let cols = self.width;
		let rows = self.tiles.len() / self.width;

		let top_exists = row > 0;
		let right_exists = col < cols - 1;
		let bottom_exists = row < rows - 1;
		let left_exists = col > 0;

		if top_exists {
			callback(&mut self[row - 1][col]);

			if left_exists {
				callback(&mut self[row - 1][col - 1]);
			}

			if right_exists {
				callback(&mut self[row - 1][col + 1])
			}
		}

		if left_exists {
			callback(&mut self[row][col - 1]);
		}

		if right_exists {
			callback(&mut self[row][col + 1])
		}

		if bottom_exists {
			callback(&mut self[row + 1][col]);

			if left_exists {
				callback(&mut self[row + 1][col - 1]);
			}

			if right_exists {
				callback(&mut self[row + 1][col + 1])
			}
		}
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
