#![allow(dead_code, unused_imports, unused_variables)]
extern crate rand;

use clap::Args;
use movies::{
    common::{self, check_if_watched, init_from_json, json_writer},
    movies::{Movie, MovieList},
    WATCHED_ARRAY,
};
use rand::Rng;
use std::env;

fn main() {
    //let args: Vec<String> = env::args().collect();

    //println!("The program name is {}", args[0]);

    let mut movies: MovieList = movies::movies::MovieList::new();
    //common::init(&mut movies);
    init_from_json(&mut movies);
    let mut rng = rand::thread_rng();
    let mut n: usize = rng.gen_range(1..251) as usize;

    //Make sure that number isn't already watched
    while check_if_watched(&mut movies, n) {
        println!("{} was watched", movies.show_movie_by_rank(n));
        n = rng.gen_range(1..251) as usize;
    }

    //movies.watch_movie(n);
    println!("{}", movies.pick_movie_by_rank(n));
    //movies.reset_watched_json();
    json_writer(movies.get_list());

    //TODO:
    /*Init from json file instead of text file. This will make it possible to remove WATCHED_ARRAY and will make it simpler to
    keep track of which movies have been watched since it's already stored in the json file
    */
}
