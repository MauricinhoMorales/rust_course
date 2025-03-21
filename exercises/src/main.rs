use std::cmp::{max, min};

fn main() {
    println!("{}", get_highest_product(&mut [-10, -2]));
}

fn get_highest_product(nums: &mut [i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }
    // Sort the array in ascending order
    nums.sort();
    let n = nums.len();
    let upperlimit = min(n, 3);
    let underlimit = min(n, 2);

    let mut case1 = nums[n - 1];

    for value in 2..=upperlimit {
        case1 *= nums[n - value];
    }

    let mut case2 = nums[n - 1];

    for value in 0..underlimit {
        case2 *= nums[value];
    }

    // Return the maximum of the two cases
    max(case1, case2)
}
