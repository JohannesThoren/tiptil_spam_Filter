use std::fmt::Error;
use std::fs::File;

use csv::{Reader, StringRecordsIter};

use crate::vocabulary::{sanitize_word, Vocabulary};
use crate::vocabulary;

pub fn count_word_occurrence(message: String, vocabulary: Vocabulary) -> Vec<usize> {
    let mut word_count: Vec<usize> = Vec::new();

    for v_word in vocabulary.vocabulary {
        let occurrence = message.matches(&v_word).count();
        if occurrence == 0 {
            word_count.push(0);
        } else {
            word_count.push(occurrence);
        }
    }

    return word_count;
}
/// this function generates a spam vocabulary, that means that it takes all of the word that
/// are more common and does some calculations with them and marks them as a "spam word".
pub fn gen_spam_vocabulary(mut rdr: Reader<File>, vocabulary: Vocabulary) {
    let mut spam_msgs_word_count: Vec<Vec<usize>> = Vec::new();
    let mut good_msgs_word_count: Vec<Vec<usize>> = Vec::new();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let msg_string = record.get(1).expect("could not get msg").to_string();
                let counted_words = count_word_occurrence(msg_string, vocabulary.clone());
                let tag = record.get(0).expect("could not get tag");

                match tag {
                    "spam" => { spam_msgs_word_count.push(counted_words) }
                    "good" => { good_msgs_word_count.push(counted_words) }
                    &_ => {}
                }
            }
            Err(_) => {}
        }
    }

    let mut spam_words: Vec<String> = Vec::new();

    let mut common_words_vocabulary = Vocabulary::new();
    common_words_vocabulary.load_from_file("topcommonwords.txt");
    common_words_vocabulary.save_to_file("topcommonwords.txt");

    for word_index in 0..vocabulary.vocabulary.len() {
        let mut good_factor = 0.0;
        let mut spam_factor = 0.0;

        for spam_msg in &spam_msgs_word_count {
            spam_factor += spam_msg[word_index] as f64;
        }

        for good_msg in &good_msgs_word_count {
            good_factor += good_msg[word_index] as f64;
        }



        if good_factor != 0.0 && spam_factor != 0.0 {
            if (good_factor / spam_factor) < 1.0 {
                if !common_words_vocabulary.in_vocabulary(vocabulary.vocabulary[word_index].clone()) {
                    spam_words.push(vocabulary.vocabulary[word_index].clone());
                }
            }
        }
    }

    let mut v = Vocabulary::new();
    v = v.set_vocabulary(spam_words.clone());
    v.save_to_file("spamvocabulary.txt");
}
///the filter_msg function does, as the name suggests filters the messages and predicts it tag.
///it will according to the value of a factor return a result containing bool, String and f64.
/// # The result
/// * the bool represents if the predicted tag is equal to the provided tag in the arguments
/// * the String represents the predicted tag as a string
/// * the f64 is the factor
pub fn filter_msg(msg: String, tag: String) -> Result<(bool, String, f64), String> {

    // this part of the code creates a vector out of the provided message,
    let mut msg_vec: Vec<String> = Vec::new();
    let words: Vec<&str> = msg.split(" ").collect();
    for word in words {
        msg_vec.push(vocabulary::sanitize_word(word.to_string()));
    }

    // some variables that are used further down in the code
    // the total_value will be used to get a factor.
    let mut total_value = 0;
    let mut spam_vocabulary = Vocabulary::new();
    let mut is_spam = false;

    // loads the spam vocabulary
    spam_vocabulary.load_from_file("spamvocabulary.txt");

    // for each msg, check if it's in the spam vocabulary
    for m in msg_vec.clone() {
        if spam_vocabulary.in_vocabulary(m) {
            // add one to the total value word is in the spam vocabulary
            total_value += 1
        }
    }


    let factor = total_value as f64 / msg_vec.len() as f64;

    if factor < 0.1 && (total_value as f64 / msg_vec.len() as f64) > 0.0 {
        is_spam = true;
    }


    return match tag.as_str() {
        "spam" => {
            return match is_spam {
                true => { Result::Ok((true, String::from("spam"), factor)) }
                false => { Result::Ok((false, String::from("good"), factor)) }
            };
        }
        "good" => {
            return match is_spam {
                true => { Result::Ok((false, String::from("spam"), factor)) }
                false => { Result::Ok((true, String::from("good"), factor)) }
            };
        }
        _ => { Result::Err(String::from("error")) }
    };
}