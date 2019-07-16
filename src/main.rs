extern crate clap;
extern crate reqwest;

use clap::{App, Arg};
use serde::*;
use std::collections::HashSet;

const MAX_HASHTAGS: usize = 30;

// TODO: Add tests
// TODO: Remove white spaces from json words

#[derive(Deserialize)]
struct JsonResult {
    word: String,
    #[allow(dead_code)]
    score: usize,
}

#[derive(Default)]
struct Context {
    input_hashtags: Vec<String>,
    all_tags: Vec<Vec<String>>,
    output_tags: HashSet<String>,
}

impl Context {
    fn process_hashtags(&mut self) {
        // Add user hashtags
        for input_hashtag in &self.input_hashtags {
            self.output_tags.insert(input_hashtag.clone());
        }

        // Add hashtags from suggestions until max size
        let mut counter = 0;
        while counter < MAX_HASHTAGS {
            for tag_group in &mut self.all_tags {
                if let Some(tag) = tag_group.pop() {
                    self.output_tags.insert(tag);
                    counter += 1;
                }
            }
            counter += 1;
        }
    }

    fn print_hashtags(&self) -> String {
        // Print all hashtags with a # prefix in a string
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

    ctx.input_hashtags = matches
        .values_of("hashtags")
        .unwrap()
        .map(|hashtag| hashtag.to_string())
        .collect();

    for hashtag in &ctx.input_hashtags {
        let url = format!("https://api.datamuse.com/words?rel_syn={}", hashtag);
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
