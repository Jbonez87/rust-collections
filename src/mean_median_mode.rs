use std::collections::HashMap;

pub fn mean(nums: &[i32]) -> f32 {
    nums.iter().sum::<i32>() as f32 / nums.len() as f32
}

pub fn median(nums: &mut [i32]) -> i32 {
    nums.sort();
    let mid = nums.len() / 2;
    nums[mid]
}

pub fn mode(nums: &[i32]) -> i32 {
    let mut mode = HashMap::new();

    for &value in nums {
        *mode.entry(value).or_insert(0) += 1;
    }

    mode.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

pub fn better_mode(nums: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();

    nums.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}
