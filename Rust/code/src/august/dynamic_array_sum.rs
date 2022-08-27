pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        let surplus_var: &[i32] = &nums[0..i + 1];
        result.push(surplus_var.iter().sum());
    }
    result
}

pub fn running_sum_one(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }).collect()
}


#[cfg(test)]
mod test {
    use crate::august::dynamic_array_sum::*;

    #[test]
    fn test() {
        println!("{:#?}", running_sum_one(vec![1, 1, 1, 1]));
        // assert_eq!(running_sum(vec![1, 1, 1, 1]), vec![4, 3, 2])
    }
}