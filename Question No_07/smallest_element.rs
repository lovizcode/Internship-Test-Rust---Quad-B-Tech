use std::io;

fn kth_smallest_element(arr: &mut Vec<i32>, k: usize) -> Option<i32> {
    arr.sort();
    arr.get(k - 1).map(|&x| x)
}

fn main() {
    let mut arr = Vec::new();

    println!("Enter 5 numbers, one by one:");

    for i in 1..=5 {
        println!("Enter number {}: ", i);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue;
            }
        };
        arr.push(num);
    }

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read input");
    let k: usize = match k_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for k! Using default value 1.");
            1
        }
    };

    match kth_smallest_element(&mut arr, k) {
        Some(result) => println!("The {}th smallest element is: {}", k, result),
        None => println!("Invalid value of k!"),
    }
}
