use std::fs;

use crate::{INPUT_FILE, movies::{Movie, self, MovieList}};

pub fn init(movies: &mut MovieList) {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    //let mut split;// = content.split("\\");



    for line in content.lines() {
        //split = line.split("\\"); 
        let vec:Vec<&str> = line.split("\\").collect();

        let rank = vec[0].parse::<usize>().unwrap();
        let year = vec[2].parse::<usize>().unwrap();

        let movie: Movie = movies::Movie::new(rank, vec[1].to_string(), year);
        movies.add_movie(movie);

    }
    println!("{}", movies.size());
}

// fn line_to_movie(line: &str) -> Movie {

// }

// pub fn find_movie (number: usize) {
//     let content = fs::read_to_string(INePUT_FILE).expect("Unable to read file");
//     println!("{}", content);
// }