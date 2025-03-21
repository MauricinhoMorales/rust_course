use std::cmp::min;

fn check_mismatch(begin_word: &str, new_word: &str) -> bool {
    let different_counter = begin_word
        .chars()
        .zip(new_word.chars())
        .fold(0, |mut init, pair| {
            if pair.0 != pair.1 {
                init += 1
            }
            init
        });
    if different_counter != 1 { false } else { true }
}

fn solution(begin_word: &str, end_word: &str, mut word_list: Vec<String>) -> u32 {
    if word_list.is_empty() {
        return 0;
    }

    if begin_word == end_word {
        return 1;
    }

    if !word_list.contains(&end_word.to_string()) {
        word_list.insert(0, end_word.to_string());
    }

    let mut shortest_path = 0;

    for new_begin_word in word_list.iter().cloned() {
        if check_mismatch(begin_word, &new_begin_word) {
            let mut new_list = word_list.clone();
            new_list.remove(
                word_list
                    .iter()
                    .position(|word| *word == new_begin_word)
                    .unwrap(),
            );
            let solution = solution(&new_begin_word, end_word, new_list);
            let path = if solution == 0 { 0 } else { 1 + solution };
            if shortest_path == 0 {
                shortest_path = path;
            } else {
                shortest_path = min(shortest_path, path)
            }
        }
    }

    return shortest_path;
}

fn main() {
    let begin_word = "hat";
    let end_word = "hog";
    let word_list = vec!["has", "tas", "tos"];

    println!(
        "The shortest path is {}",
        solution(
            begin_word,
            end_word,
            word_list.iter().map(|word| { word.to_string() }).collect()
        )
    );
}
