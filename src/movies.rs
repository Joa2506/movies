use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{common, WATCHED_ARRAY};

#[derive(Serialize, Deserialize)]
pub struct MovieList {
    movies: Vec<Movie>,
    size: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Movie {
    rank: usize,
    name: String,
    year: usize,
    watched: bool,
}
impl Movie {
    pub fn new(rank: usize, name: String, year: usize) -> Self {
        Movie {
            rank: rank,
            name: name,
            year: year,
            watched: false,
        }
    }
    pub fn watch(&mut self) {
        self.watched = true;
    }
    pub fn unwatch(&mut self) {
        self.watched = false;
    }
    pub fn get_year(&self) -> usize {
        self.year
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.rank, self.name, self.year)
    }
    pub fn is_watched(&self) -> bool {
        self.watched
    }
}
impl MovieList {
    pub fn new() -> Self {
        let movies: Vec<Movie> = Vec::new();
        MovieList {
            movies: movies,
            size: 0,
        }
    }
    pub fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
        self.size += 1;
    }

    //Returns the String value of the movies by rank.
    pub fn show_movie_by_rank(&self, rank: usize) -> String {
        if (rank > self.movies.len()) || (rank < 1) {
            "Provide a number between 1 and 250".to_string()
        } else {
            let movie = &self.movies[rank - 1];
            movie.to_string()
        }
    }

    pub fn get_list(&mut self) -> Vec<Movie> {
        return self.movies.clone();
    }
    //Picks a movie based on the rank and sets it to watched
    pub fn pick_movie_by_rank(&mut self, rank: usize) -> String {
        if (rank > 250) || (rank < 1) {
            "Provide a number between 1 and 250".to_string()
        } else {
            self.movies[rank - 1].watch();
            let movie = &self.movies[rank - 1];
            let result = common::write_to_file(movie);
            match result {
                Ok(_t) => movie.to_string(),
                Err(e) => panic!("Problem writing movie to the file {:?}", e),
            }
        }
    }
    // //Todo
    // pub fn check_if_watched(&self, rank:usize) -> bool {
    //     true
    // }

    //Returns the String value of the movies based on name
    pub fn look_up_name(&self, name: String) -> String {
        match self.movies.iter().find(|movie| movie.name == name) {
            Some(movie) => movie.to_string(),
            None => "Could not find movie. Have you written correctly?".to_string(),
        }
    }
    //Will not be needed once read from json is implemented instead of read from txt file
    pub fn update_watched_movies(&mut self) {
        for i in 0..250 {
            unsafe {
                if WATCHED_ARRAY[i] {
                    self.movies[i].watch();
                }
            }
        }
    }
    pub fn watch_movie(&mut self, rank: usize) {
        unsafe {
            WATCHED_ARRAY[rank - 1] = true;
        }
        self.movies[rank - 1].watched = true;
    }
    // pub fn watch_by_name(&self, name:String) -> String {

    //     let mut movie= self.movies.iter().find(|movie| movie.name == name);
    //     let mut movie_watch = &mut movie;
    //     match movie_watch {
    //         Some(m) => {
    //             //TODO. Write to watched list. Only need to write by rank.
    //             m.watch();
    //             "The movie has been watched".to_string()
    //         },
    //         None => "Could not find movie. Have you written correctly?".to_string()
    //     }
    // }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn json_parser(&mut self) {
        let file = File::open("ressources/movies.json").unwrap();
        self.movies = serde_json::from_reader(file).unwrap();
        self.size = self.movies.len();
    }

    pub fn reset_watched_json(&mut self) {
        for i in 1..self.size {
            self.movies[i - 1].unwatch();
        }
    }
}
