// TODO: add missing_docs later
#![warn(missing_debug_implementations, rust_2018_idioms)]
// TODO: remove later
#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::missing_docs_in_private_items,
    clippy::result_expect_used
)]

mod ai;
mod board;
mod mark;
mod player;
mod std_io;
mod ui;

#[cfg(test)]
mod test_common;
