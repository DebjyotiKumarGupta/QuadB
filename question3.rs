fn shortest_word(s: &str) -> String {
    s.split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
        .to_string()
}

fn main() {
    let text = "Given a string of words, implement a function that returns the shortest word in the string.";
    let shortest = shortest_word(text);
    println!("The shortest word is: {}", shortest);
}
