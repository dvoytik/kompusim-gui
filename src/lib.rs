#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::KompusimApp;
mod instr_decoder;
mod instr_list;
mod load_demo;
