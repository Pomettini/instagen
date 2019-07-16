extern crate clap;
extern crate reqwest;

use clap::{App, Arg};
use serde::*;
use std::collections::HashSet;

const MAX_HASHTAGS: usize = 30;

// TODO: Add tests

#[derive(Deserialize)]
struct JsonResult {
    word: String,
    #[allow(dead_code)]
    score: usize,
}

#[derive(Default)]
struct Context<'a> {
    input_hashtags: Vec<&'a str>,
    similar_words: Vec<Vec<String>>,
    output_tags: HashSet<String>,
}

impl<'a> Context<'a> {
    fn process_hashtags(&mut self) {
        // Add user hashtags
        for input_hashtag in &self.input_hashtags {
            self.output_tags.insert(input_hashtag.to_string());
        }

        let mut counter = 0;
        // Add hashtags from suggestions until max size
        while counter < MAX_HASHTAGS {
            for words in &mut self.similar_words {
                if let Some(word) = words.pop() {
                    self.output_tags.insert(word);
                    counter += 1;
                }
            }
            counter += 1;
        }
    }

    fn print_hashtags(&self) -> String {
        // Print all hashtags with a # prefix in a string
        // Also, removing white spaces from tags
        self.output_tags
            .clone()
            .into_iter()
            .map(|hashtag| format!("#{} ", hashtag.replace(" ", "")))
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
        .expect("No input hashtags founds")
        .collect();

    for hashtag in &ctx.input_hashtags {
        // Ask a json with similar words to the datamuse APIs
        let url = format!("https://api.datamuse.com/words?rel_syn={}", hashtag);
        let json_result: Vec<JsonResult> = reqwest::get(&url)
            .expect("Could not fetch JSON from datamuse")
            .json()
            .expect("Could not parse JSON");
        let words: Vec<String> = json_result
            .iter()
            .map(|result| result.word.clone())
            .collect();
        ctx.similar_words.push(words);
    }

    ctx.process_hashtags();

    println!("{}", &ctx.print_hashtags());
}
