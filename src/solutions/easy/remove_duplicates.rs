
pub fn remove_duplicates_simple(nums: &mut Vec<i32>) -> i32 {
    let mut k: usize = 1;
    let nums_size = nums.len();
    for i in 1..nums_size {
        if nums[i] != nums[i - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

pub fn remove_duplicates_vec_approach(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as _
}
