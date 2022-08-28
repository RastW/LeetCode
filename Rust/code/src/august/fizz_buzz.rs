pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..n + 1)
        .into_iter()
        .map(|num| {
            if num % 3 == 0 && num % 5 == 0 {
                String::from("FizzBuzz")
            } else if num % 3 == 0 {
                String::from("Fizz")
            } else if num % 5 == 0 {
                String::from("Buzz")
            } else {
                num.to_string()
            }
        })
        .collect()
}

pub fn fizz_buzz_one(n: i32) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut i = 1;
    while i <= n {
        match i {
            x if x % (3 * 5) == 0 => result.push("FizzBuzz".to_string()),
            x if x % 3 == 0 => result.push("Fizz".to_string()),
            x if x % 5 == 0 => result.push("Buzz".to_string()),
            _ => result.push(i.to_string()),
        }
        i += 1;
    }
    result
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["1","2","Fizz"], fizz_buzz(3));
    }
}