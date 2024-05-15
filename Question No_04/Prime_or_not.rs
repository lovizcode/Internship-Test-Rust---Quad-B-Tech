use std::io;

fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false; 
    }
    if number <= 3 {
        return true; 
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false; 
    }
    
    
    let mut i = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false; 
        }
        i += 6;
    }

    true 
}

fn main() {
    println!("Enter a number to check if it's prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u64 = input.trim().parse().expect("Please enter a valid number");

    println!("Is {} prime? {}", input, is_prime(input));
}
