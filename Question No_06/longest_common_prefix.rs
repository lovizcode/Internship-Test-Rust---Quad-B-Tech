fn longest_common_prefix(strings: Vec<&str>) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let shortest = strings.iter().min_by_key(|s| s.len()).unwrap();

    for (i, &ch) in shortest.as_bytes().iter().enumerate() {
        if !strings.iter().all(|s| s.as_bytes()[i] == ch) {
            return shortest[..i].to_string();
        }
    }

    shortest.to_string()
}

fn main() {
    let strings = vec!["flower", "flow", "flight"];
    println!("{}", longest_common_prefix(strings)); 
}
