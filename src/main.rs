extern crate clap;
extern crate reqwest;

use clap::*;
use reqwest::*;
use serde::*;

const MAX_HASHTAGS: usize = 30;

#[derive(Deserialize, Debug)]
struct Words {
    word: String,
    score: usize,
}

fn main() {
    let matches = App::new("Instagram Hashtag Generator")
        .version("0.1.0")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("hashtags").multiple(true))
        .get_matches();

    let input_hashtags: Vec<&str> = matches.values_of("hashtags").unwrap().collect();

    let mut current_hashtags: Vec<&str> = input_hashtags;

    let words: Vec<Words> = reqwest::get("https://api.datamuse.com/words?rel_trg=cow")
        .unwrap()
        .json()
        .unwrap();

    println!("{:?}", words);

    // while current_hashtags.len() < MAX_HASHTAGS {

    // }
}
