#![allow(dead_code, unused_imports, unused_variables)]
extern crate rand;

use movies::{common, movies::{MovieList, Movie}};
use rand::Rng;

fn main() {
    let mut movies:MovieList = movies::movies::MovieList::new();
    common::init(&mut movies);
    
    let mut rng = rand::thread_rng();
    let n:usize = rng.gen_range(1..251) as usize;

    println!("{}", movies.pick_movie_by_rank_test(n));
    
    //println!("{}", movies.look_up_name("The Shawshank Redemption".to_string()));

}
