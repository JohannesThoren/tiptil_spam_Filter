use std::fs::File;

use csv::Reader;

use crate::filter::{count_word_occurrence, filter_msg};
use crate::vocabulary::Vocabulary;

mod vocabulary;
mod filter;

fn main() {
    let mut v = vocabulary::Vocabulary::new();
    v.build_vocabulary_from_csv("msgs.csv");
    v.save_to_file("vocabulary.txt");

    let csv_file = File::open("msgs.csv");
    let mut rdr = csv::Reader::from_reader(csv_file.expect("could not open file!"));

    filter::gen_spam_vocabulary(rdr, v.clone());


    let csv_file = File::open("msgs.csv");
    let mut rdr = csv::Reader::from_reader(csv_file.expect("could not open file!"));
    run_filter(rdr);
}


fn run_filter(mut rdr: Reader<File>) {
    println!("==== Filtering and Sorting Messages ====");
    println!("[tag, prediction], msg");
    let mut msg_vec: Vec<(String, String)> = Vec::new();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let msg_string = record.get(1).expect("could not get msg").to_string();
                let tag = record.get(0).expect("could not get tag");

                msg_vec.push((msg_string.clone(), tag.clone().to_string()));
            }
            Err(_) => {}
        }
    }

    let mut accuracy = 0.0;

    for msg in msg_vec.clone() {
        let result = filter_msg(msg.0.clone(), msg.1.clone());

        match result {
            Ok(b) => {
                println!("{:<20}[{:<8},{:>8}] {}", b.2.clone() ,msg.clone().1, b.1.clone(), msg.clone().0);

                if b.0 {
                    accuracy += 1.0;
                }
            }
            Err(_) => {}
        }
    }

    println!("accuracy: {}", accuracy as f64 / msg_vec.clone().len() as f64)
}