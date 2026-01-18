#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod graph;
mod skills;
pub use app::GraphApp;
pub use skills::parse_lua;
