//pub mod args;
pub mod common;
pub mod movies;

pub const SIZE: usize = 250;
pub const INPUT_FILE: &str = "ressources/movielist.txt";
pub const WATCHED_FILE: &str = "ressources/watched.txt";
pub const WATCHED_TEST: &str = "ressources/watched_test.txt";
pub static mut WATCHED_ARRAY: [bool; 250] = [false; 250];
//static mut movie_list:MovieList;
