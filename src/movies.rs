use std::error::Error;

pub struct MovieList {
    movies: Vec<Movie>
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
    pub fn watch(&mut self) -> bool {
        self.watched = true;
        self.watched
    }
    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }
    pub fn get_rank(&mut self) -> usize {
        self.rank
    }
    pub fn get_year(&mut self) -> usize {
        self.year
    }
    pub fn watched(&mut self) -> bool{
        self.watched
    }
}

impl MovieList {
    pub fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
    }
    pub fn get_movie_by_rank(&mut self, rank: usize) -> Result<Movie, Error> {
        for movie in self.movies {
            if movie.get_rank() == rank {
                movie; 
            }
        }
        
    }
}
