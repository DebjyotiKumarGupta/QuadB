fn kth_smallest(mut nums: Vec<i32>, k: usize) -> i32 {
    nums.sort();
    nums[k - 1]
}

fn main() {
    let nums = vec![7, 5, 3, 1, 9];
    let k = 2;
    let kth_smallest_num = kth_smallest(nums, k);
    println!("The {}th smallest number is: {}", k, kth_smallest_num);
}
