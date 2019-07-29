extern crate reqwest;
#[macro_use]
extern crate maplit;

use serde::*;
use std::collections::HashSet;

const MAX_HASHTAGS: usize = 30;

#[derive(Deserialize)]
struct JsonResult {
    word: String,
    score: usize,
}

pub struct Instagen {
    output_tags: HashSet<String>,
}

impl Instagen {
    pub fn generate(input_hashtags: Vec<&str>) -> Self {
        let mut instagen = Self {
            output_tags: HashSet::new(),
        };
        let mut similar_words: Vec<Vec<String>> = Vec::new();

        for hashtag in &input_hashtags {
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

            similar_words.push(words);
        }

        instagen.process_hashtags(&input_hashtags, &mut similar_words);

        instagen
    }

    fn process_hashtags(
        &mut self,
        input_hashtags: &Vec<&str>,
        similar_words: &mut Vec<Vec<String>>,
    ) {
        // Add user hashtags
        for input_hashtag in input_hashtags {
            self.output_tags.insert(input_hashtag.to_string());
        }

        let mut counter = 0;
        // Add hashtags from suggestions until max size
        while counter < MAX_HASHTAGS {
            for words in &mut *similar_words {
                if let Some(word) = words.pop() {
                    self.output_tags.insert(word.to_string());
                    counter += 1;
                }
            }
            counter += 1;
        }
    }

    pub fn to_hashtags(self) -> String {
        // Print all hashtags with a # prefix in a string
        // Also, removes white spaces from tags
        self.output_tags
            .clone()
            .into_iter()
            .map(|hashtag| format!("#{} ", hashtag.replace(" ", "")))
            .collect::<String>()
    }

    pub fn to_hashset(self) -> HashSet<String> {
        self.output_tags
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::*;

    #[test]
    fn test_load_sample_tags() {
        let result = Instagen::generate(vec!["sweden", "summer", "sea"]);

        assert_eq!(
            result.output_tags,
            hashset! {String::from("sweden"), String::from("sverige"), String::from("ocean"), String::from("summertime"), String::from("sea"), String::from("summer")}
        );
    }

    #[test]
    fn test_load_sample_tags_red() {
        let result = Instagen::generate(vec!["sweden", "summer", "sea"]);

        assert_ne!(result.output_tags, hashset! {String::from("italy")});
    }

    #[test]
    fn test_load_empty() {
        let result = Instagen::generate(vec![""]);

        assert_eq!(result.output_tags, hashset! {String::from("")});
    }

    #[test]
    fn test_load_empty_red() {
        let result = Instagen::generate(vec![""]);

        assert_ne!(result.output_tags, hashset! {String::from("italy")});
    }

    #[test]
    fn test_load_sample_tags_return_hashset() {
        let result = Instagen::generate(vec!["cow"]).to_hashset();

        assert_eq!(
            result,
            hashset! {String::from("cow"), String::from("overawe"), String::from("moo-cow")}
        );
    }

    #[test]
    fn test_load_sample_tags_return_hashset_red() {
        let result = Instagen::generate(vec!["cow"]).to_hashset();

        assert_ne!(result, hashset! {String::from("dog")});
    }

    #[test]
    fn test_load_sample_tags_return_hashtags() {
        let result = Instagen::generate(vec!["kitten"]).to_hashtags();

        assert!(result.contains("#kitty"));
    }

    #[test]
    fn test_load_sample_tags_return_hashtags_red() {
        let result = Instagen::generate(vec!["kitten"]).to_hashtags();

        assert!(!result.contains("#italy"));
    }
}
