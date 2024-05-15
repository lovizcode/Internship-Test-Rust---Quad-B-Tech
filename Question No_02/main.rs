use std::io;

fn main() {
    // Array that store 5 Numbers
    let mut numbers = [0; 5];

    for i in 0..5 {
        println!("Enter the {}th number:", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid number");
        numbers[i] = num;
    }
    println!("Enter the number to search:");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target: i32 = target.trim().parse().expect("Invalid number");

    let index = find_first_occurrence(&numbers, target);

    if index != -1 {
        println!("The index of {} is {}", target, index);
    } else {
        println!("{} is not found in the array.", target);
    }
}

fn find_first_occurrence(arr: &[i32; 5], target: i32) -> i32 {
    let mut result = -1;

    for i in 0..5 {
        if arr[i] == target {
            result = i as i32;
            break;
        }
    }
    result
}
