use std::fs;

use crate::{INPUT_FILE, movies::Movie};

pub fn init() {
    let content = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let mut split;// = content.split("\\");



    for line in content.lines() {
        split = line.split("\\");
        let vec = split.collect::<Vec<&str>>();
        for i in vec  {
            print!("{}", i);
            //Add each element of
        }
        println!();
    }
}

// fn line_to_movie(line: &str) -> Movie {

// }

// pub fn find_movie (number: usize) {
//     let content = fs::read_to_string(INePUT_FILE).expect("Unable to read file");
//     println!("{}", content);
// }