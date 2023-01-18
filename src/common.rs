
use std::{fs};
use std::fs::File;

use std::io::{Write};

use crate::{INPUT_FILE, movies::{Movie, self, MovieList}, WATCHED_TEST};

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

pub fn write_to_file(movie: &Movie) -> std::io::Result<()>{
    
    let mut file =  File::options()
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

//TODO parse movies to json format and read from that instead