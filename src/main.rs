use clap::Parser;
use md5::compute;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::thread;
use rayon::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    wordlist: String,
    hash_file: String,
}

fn compute_md5(input: &str, wordlist: Arc<String>) -> String {
    let words = BufReader::new(File::open(&*wordlist).unwrap());
    for word in words.lines() {
        let word = word.unwrap_or_default();
        let computed = format!("{:x}", compute(&word));
        if computed == input {
            return word
        }
    }
    String::from("")
}

fn main() {
    let cli = Cli::parse();
    let wordlist = Arc::new(cli.wordlist);
    let hash_to_crack:Vec<String> = read_to_string(cli.hash_file).unwrap().split('\n').map(|x| x.to_owned()).collect();
    let static_hash = hash_to_crack.leak();
    let hash_length = static_hash.len();
    let mut threads = vec![];

    for i in 0..hash_length {
        let hash = static_hash.get(i).unwrap();
        let wordlist = wordlist.clone();
        threads.push(thread::spawn(move || {
            (i+1, compute_md5(hash, wordlist))
        }));
    }
    threads
    .into_par_iter()
    .map(|t| t.join().unwrap())
    .for_each(|(idx, computed)| {
        println!("The input for {} was: {}", idx, computed);
    });
}