extern crate assert_cmd;
extern crate clap;
extern crate instagen;

use clap::{App, Arg};
use instagen::Instagen;

fn main() {
    let matches = App::new("Instagen")
        .version("0.1.0")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("hashtags").required(true).multiple(true))
        .get_matches();

    let hashtags: Vec<&str> = matches
        .values_of("hashtags")
        .expect("No input hashtags founds")
        .collect();

    let result = Instagen::generate(hashtags).to_hashtags();

    println!("{}", result);
}

mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_no_arguments() {
        Command::cargo_bin("instagen").unwrap().assert().failure();
    }

    #[test]
    fn test_generate_hashtags_green() {
        let output = Command::cargo_bin("instagen")
            .unwrap()
            .arg("sweden")
            .arg("summer")
            .arg("sea")
            .output()
            .unwrap();

        let output_string = String::from_utf8_lossy(&output.stdout);

        assert!(output_string.contains("#sweden"));
        assert!(output_string.contains("#summer"));
        assert!(output_string.contains("#sea"));
        assert!(output_string.contains("#sverige"));
        assert!(output_string.contains("#summer"));
        assert!(output_string.contains("#ocean"));
    }

    #[test]
    fn test_generate_hashtags_red() {
        let output = Command::cargo_bin("instagen")
            .unwrap()
            .arg("sweden")
            .arg("summer")
            .arg("sea")
            .output()
            .unwrap();

        let output_string = String::from_utf8_lossy(&output.stdout);

        assert!(!output_string.contains("#italy"));
    }
}
