use std::io;

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    if is_palindrome(&input) {
        println!("The string is a palindrome");
    } else {
        println!("The string is not a palindrome");
    }
}

fn is_palindrome(s: &str) -> bool {
    let len = s.len();
    for i in 0..len / 2 {
        if s.chars().nth(i).unwrap() != s.chars().nth(len - 1 - i).unwrap() {
            return false;
        }
    }
    true
}
