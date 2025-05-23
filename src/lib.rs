#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod tools;
pub use tools::song_details_to_file_path;