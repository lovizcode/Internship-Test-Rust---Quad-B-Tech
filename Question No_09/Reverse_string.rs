use std::io;

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let reversed_string = reverse_string(&input);

    println!("Reversed string: {}", reversed_string);
}

fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}
