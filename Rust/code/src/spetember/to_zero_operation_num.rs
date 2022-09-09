pub fn number_of_steps(num: i32) -> i32 {
    let mut i = 0;
    let mut num = num.clone();
    while num > 0{
        i += 1;
        if num % 2 == 0 {
            num = num / 2;
            continue;
        }
        num -= 1;
    }
    i
}

pub fn number_of_steps_one(num: i32) -> i32 {
    num.count_ones();
    3
}

#[cfg(test)]
pub mod test {

    #[test]
    fn test() {
        println!("{}", 789_i32.count_ones());
    }
}
