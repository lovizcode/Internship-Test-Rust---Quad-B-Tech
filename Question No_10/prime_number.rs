use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: u64 = input.trim().parse().expect("Invalid number");

    if is_prime(number) {
        println!("{} is a prime number", number);
    } else {
        println!("{} is not a prime number", number);
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    } else if n == 2 {
        return true;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    true
}
