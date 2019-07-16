extern crate clap;
extern crate reqwest;

use clap::{App, Arg};
use serde::*;
use std::collections::HashSet;

const MAX_HASHTAGS: usize = 30;

#[derive(Deserialize)]
struct JsonResult {
    word: String,
    score: usize,
}

#[derive(Default)]
struct Context {
    all_tags: Vec<Vec<String>>,
    output_tags: HashSet<String>,
}

impl Context {
    fn process_hashtags(&mut self) {
        let mut counter = 0;
        loop {
            for tags in &mut self.all_tags {
                match tags.pop() {
                    Some(tag) => {
                        self.output_tags.insert(tag);
                    }
                    None => continue,
                }
            }

            counter += 1;

            if counter > MAX_HASHTAGS {
                break;
            }
        }
    }

    fn print_hashtags(&self) -> String {
        self.output_tags
            .clone()
            .into_iter()
            .map(|hashtag| format!("#{} ", hashtag))
            .collect::<String>()
    }
}

fn main() {
    let matches = App::new("Instagram Hashtag Generator")
        .version("0.1.0")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("hashtags").required(true).multiple(true))
        .get_matches();

    let mut ctx: Context = Default::default();

    let input_hashtags: Vec<&str> = matches.values_of("hashtags").unwrap().collect();

    for hashtag in &input_hashtags {
        let url = format!("https://api.datamuse.com/words?rel_trg={}", hashtag);
        let json_result: Vec<JsonResult> = reqwest::get(&url).unwrap().json().unwrap();
        let words: Vec<String> = json_result
            .iter()
            .map(|result| result.word.clone())
            .collect();
        ctx.all_tags.push(words);
    }

    ctx.process_hashtags();

    println!("{}", &ctx.print_hashtags());
}
