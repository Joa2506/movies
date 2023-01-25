use std::fs;
use std::fs::File;

use std::io::Write;

//use crate::WATCHED_ARRAY;
use crate::{
    movies::{self, Movie, MovieList},
    INPUT_FILE, WATCHED_TEST,
};

pub fn init(movies: &mut MovieList) {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    //let mut split;// = content.split("\\");

    for line in content.lines() {
        //split = line.split("\\");
        let vec: Vec<&str> = line.split("\\").collect();

        let rank = vec[0].parse::<usize>().unwrap();
        let year = vec[2].parse::<usize>().unwrap();

        let movie: Movie = movies::Movie::new(rank, vec[1].to_string(), year);
        movies.add_movie(movie);
    }

    init_watched();
    movies.update_watched_movies();
}

pub fn write_to_file(movie: &Movie) -> std::io::Result<()> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .append(true)
        .open(WATCHED_TEST)?;
    //let mut buf_reader = BufReader::new(file);

    let mut contents = movie.to_string();
    contents.push('\n');
    //let contents = "Hello world!";
    //buf_reader.read_to_string(&mut contents)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()?;
    drop(file);
    Ok(())
}

pub fn init_watched() {
    let watched_movies = fs::read_to_string(WATCHED_TEST).expect("Could not read watched_test.txt");

    for line in watched_movies.lines() {
        //split = line.split("\\");
        let vec: Vec<&str> = line.split(" ").collect();

        let rank = vec[0].parse::<usize>().unwrap();
        unsafe {
            crate::WATCHED_ARRAY[rank - 1] = true;
        }
    }
}

pub fn check_if_watched(movies: &mut MovieList, index: usize) -> bool {
    //unsafe { WATCHED_ARRAY[index - 1] }
    if index > 250 {
        println!("Index needs to be withing the borders of 0 - 250");
        return false;
    }

    let movie_list = movies.get_list();
    let movie = movie_list.get(index - 1);
    match movie {
        Some(x) => return x.is_watched(),
        None => {
            println!("There was no movie in the list");
            return false;
        }
    }
}
//Writes the movies vector to the json file, with updated struct values
pub fn json_writer(movies: Vec<Movie>) {
    let mut file = File::create("ressources/movies.json").unwrap();
    //let movie_list = movies.get_list();

    let json = serde_json::to_string_pretty(&movies).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

//Parses from json file
pub fn init_from_json(movies: &mut MovieList) {
    init_watched();
    movies.json_parser();
}
