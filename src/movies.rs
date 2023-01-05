
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
    pub fn watch(&mut self) -> bool {
        self.watched = true;
        self.watched
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
    pub fn pick_movie_by_rank(&self, rank:usize) -> String {
        if (rank > 250) || (rank < 1) {
            "Provide a number between 1 and 250".to_string()
        }
        else {
            let movie= &self.movies[rank-1];
            movie.to_string()
        }
    }

    pub fn get_movie_by_name(&self, name:String) -> Option<& Movie> {
        self.movies.iter().find(|movie| movie.name == name)
    }
    pub fn size(&self) -> usize{
        self.size
    }
}
