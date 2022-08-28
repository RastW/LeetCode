pub fn fizz_buzz(n: i32) -> Vec<String> {
    let result = Vec::new();
    (0..5)
        .scan(0, |sum, v| {
            *sum += v;
            Some(*sum)
        }).collect();
    result
}


#[cfg(test)]
mod test {

    #[test]
    fn test() {}
}