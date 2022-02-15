use std::alloc::System;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Clone)]
pub struct Vocabulary {
    pub vocabulary: Vec<String>,
}

/// removes some chars from a string,
/// the so called illegal_chars are the chars that it will remove
pub fn sanitize_word(word: String) -> String {
    let illegal_chars = ["!", "$", "@", ",", ".", "?", ":"];
    let mut w = word.clone();
    for char in illegal_chars {
        let index = w.find(char);
        match index {
            None => {}
            Some(i) => {
                w.remove(i);
            }
        }
    }
    return w;
}

/// vocabulary
impl Vocabulary {
    pub fn new() -> Self {
        Self {
            vocabulary: Vec::new(),
        }
    }

    /// takes a vector of string that will be turned into a vocabulary
    pub fn set_vocabulary(mut self, vocabulary: Vec<String>) -> Vocabulary {
        self.vocabulary = vocabulary;
        self
    }

    /// loads a vocabulary from a file
    pub fn load_from_file(&mut self, file_path: &str) {
        let vocabulary_file = File::open(file_path).expect("could not open file");
        let reader = BufReader::new(vocabulary_file);

        for line in reader.lines() {
            match line {
                Ok(word) => {
                    if word != "" && word != " " {
                        &self.vocabulary.push(word);
                    }
                }
                Err(e) => { eprintln!("{}", e); }
            }
        }

        &self.vocabulary.sort();
    }

    /// saves all words in a vocabulary to a file
    pub fn save_to_file(&mut self, file_path: &str) {
        let mut of = File::create(file_path).expect("could not create file!");
        for word in &self.vocabulary {
            if word != "" && word != " " {
                let w = format!("{}\n", word);
                of.write(w.as_bytes());
            }
        }
    }

    /// takes a csv file with msgs, and then creates a vocabulary from the msgs
    pub fn build_vocabulary_from_csv(&mut self, csv_file: &str) {
        let mut rdr = csv::Reader::from_reader(File::open(csv_file).expect("could not open file!"));

        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let words: Vec<&str> = record.get(1).expect("could not read record").split(" ").collect();
                    for word in words {
                        if word != " " && word != "" {
                            let w = sanitize_word(word.to_string());
                            if !&self.in_vocabulary(w.clone().to_lowercase()) {
                                &self.append_word(w.clone().to_lowercase());
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
        &self.vocabulary.sort();
        println!("vocabulary size: {}", &self.vocabulary.len())
    }

    /// checks if a word is in the vocabulary
    pub fn in_vocabulary(&self, word: String) -> bool {
        for w in &self.vocabulary {
            if w == &word { return true; }
        }
        return false;
    }

    /// appends a word to the vocabulary
    pub fn append_word(&mut self, word: String) {
        &self.vocabulary.push(word);
    }
}