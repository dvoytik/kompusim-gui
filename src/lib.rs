#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::KompusimApp;
mod decode_instr;
mod instr_list;
