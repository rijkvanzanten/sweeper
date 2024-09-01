mod index;
pub use index::get_index;

mod new;
pub use new::post_new;

mod game;
pub use game::get_game;

mod flag;
pub use flag::post_flag;

mod reveal;
pub use reveal::post_reveal;