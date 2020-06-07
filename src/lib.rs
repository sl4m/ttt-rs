#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::must_use_candidate,
    clippy::result_expect_used
)]

mod ai;
pub mod board;
pub mod game;
pub mod mark;
pub mod player;
mod run;
pub mod std_io;
pub mod strings;
pub mod ui;

pub use crate::run::run;

#[cfg(test)]
mod test_common;
