use std::io;

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let shortest_word = find_shortest_word(&input);
    println!("The shortest word in the string is: {}", shortest_word);
}

fn find_shortest_word(s: &str) -> String {
    let mut shortest_word = String::new();
    let mut shortest_length = std::usize::MAX;
    for word in s.split_whitespace() {
        let word_length = word.len();
        if word_length < shortest_length {
            shortest_word = word.to_string();
            shortest_length = word_length;
        }
    }

    shortest_word
}
