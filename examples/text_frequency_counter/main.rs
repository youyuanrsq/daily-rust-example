use std::collections::HashMap;

fn main() {
    let words = "The quick brown fox jumps over the lazy dog. The dog sleeps.".to_lowercase();
    let no_punct: String = words
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect();
    println!("no_punct words: {}", no_punct);
    let n = 3;
    let mut word_count = HashMap::new();
    for i in no_punct.split_whitespace() {
        if let Some(value) = word_count.get_mut(i) {
            *value += 1;
        } else {
            word_count.insert(i, 1);
        }
    }

    let mut count_vec: Vec<(&str, i32)> = word_count.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    for (k, v) in count_vec[..n].into_iter() {
        println!("word: {}, count: {}", k, v);
    }
}

// Claude 3.5 sonnet 优化后的版本
// fn main() {
//     let text = "The quick brown fox jumps over the lazy dog. The dog sleeps.".to_lowercase();

//     // Remove punctuation and keep only alphabetic chars and whitespace
//     let cleaned_text: String = text
//         .chars()
//         .filter(|c| c.is_alphabetic() || c.is_whitespace())
//         .collect();

//     // Count word frequencies
//     let mut frequency_map = HashMap::new();
//     for word in cleaned_text.split_whitespace() {
//         *frequency_map.entry(word).or_insert(0) += 1;
//     }

//     // Convert to vector and sort by frequency (descending)
//     let mut frequency_vec: Vec<_> = frequency_map.into_iter().collect();
//     frequency_vec.sort_by(|a, b| b.1.cmp(&a.1));

//     // Print top N results
//     let top_n = 3;
//     for (word, count) in frequency_vec.iter().take(top_n) {
//         println!("word: {}, count: {}", word, count);
//     }
// }
