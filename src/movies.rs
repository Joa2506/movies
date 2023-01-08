use crate::common;


pub struct MovieList {
    movies: Vec<Movie>,
    size: usize,
}


pub struct Movie {
    rank: usize,
    name: String,
    year: usize,
    watched: bool,
}


impl Movie {
    pub fn new(rank:usize, name: String, year:usize) -> Self {
        Movie { rank: rank, name: name, year: year, watched: false }
        
    }
    pub fn watch(&mut self)  {
        self.watched = true;
    }
    pub fn get_year(&self) -> usize {
        self.year
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.rank, self.name, self.year)
    }
}

impl MovieList {
    pub fn new() -> Self {
        let movies: Vec<Movie> = Vec::new();
 //       let watched: Vec<Movie> = Vec::new();
        MovieList { movies:movies, size:0 }
    }
    pub fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
        self.size += 1;
    }
    // pub fn get_movie_by_rank(&self, rank: usize) -> Option<&Movie> {
    //     self.movies.iter().find(|movie| movie.rank == rank)
    // }
    //Since the list is already sorted after rank all you need is to index the 
    // pub fn get_movie_by_name(&self, name:String) -> Option<& Movie> {
    //     self.movies.iter().find(|movie| movie.name == name)
    // }

    //Returns the String value of the movies by rank.
    pub fn pick_movie_by_rank(&self, rank:usize) -> String {
        if (rank > self.movies.len()) || (rank < 1) {
            "Provide a number between 1 and 250".to_string()
        }
        else {
            let movie= &self.movies[rank-1];
            movie.to_string()
        }
    }

    pub fn pick_movie_by_rank_test(&self, rank:usize) -> String {
        if (rank > 250) || (rank < 1) {
            "Provide a number between 1 and 250".to_string()
        }
        else {
            let movie= &self.movies[rank-1];
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
    pub fn look_up_name(&self, name:String) -> String {
        match self.movies.iter().find(|movie| movie.name == name) {
            Some(movie) => movie.to_string(),
            None => "Could not find movie. Have you written correctly?".to_string()
        }
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



    pub fn size(&self) -> usize{
        self.size
    }
}
