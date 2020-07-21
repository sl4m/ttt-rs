#![forbid(unsafe_code)]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::must_use_candidate,
    clippy::expect_used
)]

mod ai;
mod board;
mod ext;
mod game;
mod mark;
mod player;
mod run;
mod std_io;
mod ui;

use ai::{Ai, Negamax};
pub use board::Board;
use ext::UsizeExt;
pub use game::Game;
pub use mark::Mark;
pub use player::{Computer, Human, Player};
pub use run::run;
pub(crate) use std_io::ConsoleIo;
pub use std_io::StdIo;
pub use ui::Ui;

#[cfg(test)]
mod test_common;
