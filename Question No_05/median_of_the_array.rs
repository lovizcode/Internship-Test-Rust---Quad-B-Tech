fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        let mid_right = nums[len / 2];
        let mid_left = nums[len / 2 - 1];
        return (mid_left as f64 + mid_right as f64) / 2.0;
    } else {
        return nums[len / 2] as f64;
    }
}

fn main() {
    let nums = vec![1, 3, 5, 7, 9];

    let median = find_median(&nums);

    println!("Median: {}", median);
}
