extern crate anagram;

use anagram::{EquivalenceClass, SubsetEnumerator};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const ANAGRAM: &str = "poultry outwits ants";
const WORDFILE_PATH: &str = "./wordlist";

fn read_words<P: AsRef<Path>>(
    path: P,
    bound: EquivalenceClass,
) -> (Vec<String>, Vec<EquivalenceClass>) {
    let file = File::open(path).unwrap();
    let mut words_set = Vec::new();
    for l in BufReader::new(file).lines() {
        let w = l.unwrap();
        if let Some(e) = EquivalenceClass::new(&w) {
            if e <= bound {
                words_set.push((w, e));
            }
        }
    }
    words_set.sort_by(|x, y| y.0.len().cmp(&x.0.len()));
    let mut words = Vec::new();
    let mut set = Vec::new();
    for (w, e) in words_set.into_iter() {
        words.push(w);
        set.push(e);
    }
    (words, set)
}

fn main() {
    let anagram: EquivalenceClass = ANAGRAM
        .split_whitespace()
        .map(|e| EquivalenceClass::new(e).unwrap())
        .sum();
    let (words, set) = read_words(WORDFILE_PATH, anagram);
    println!("num words: {}", words.len());
    let enumerator = SubsetEnumerator::new(&set, anagram);

    for subset in enumerator.take(10) {
        println!("{:?}", subset);
        for i in subset {
            print!("{}", words[i]);
        }
        println!("");
    }
}
