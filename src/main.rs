extern crate anagram;
extern crate md5;

use anagram::{EquivalenceClass, SubsetEnumerator, SwapIterator};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const ANAGRAM: &str = "poultry outwits ants";
const WORDFILE_PATH: &str = "./wordlist";
const EASY_MD5_HASH: md5::Digest = md5::Digest([
    0xe4, 0x82, 0x0b, 0x45, 0xd2, 0x27, 0x7f, 0x38, 0x44, 0xea, 0xc6, 0x6c, 0x90, 0x3e, 0x84, 0xbe,
]);

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
    let enumerator = SubsetEnumerator::new(&set, anagram);

    let test_subset = |subset: &[usize], buffer: &mut String| -> bool {
        buffer.clear();
        for i in subset.iter() {
            buffer.push_str(&words[*i]);
        }
        if EASY_MD5_HASH == md5::compute(buffer.as_bytes()) {
            print!("Easy: {}", buffer);
            true
        } else {
            false
        }
    };

    for mut subset in enumerator.take(10_000) {
        let mut s = String::new();
        if test_subset(&subset, &mut s) {
            return ();
        }
        for (i, j) in SwapIterator::new(subset.len()) {
            let t = subset[i];
            subset[i] = subset[j];
            subset[j] = t;
            if test_subset(&subset, &mut s) {
                return ();
            }
        }
    }
}
