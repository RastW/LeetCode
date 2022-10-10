use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> String {
    let chars = s.chars().collect::<Vec<_>>();

    fn expand(c: &Vec<chars>, ll: i32, rr: i32) -> String {
        let(mut l, mut r) = (ll, rr);
        while l >= 0 && r < (c.len() as i32) && c[l as usize] == c[r as usize]{
            l -= 1;
            r += 1;
        }
        return c
            .clone()
            .splice(((l + 1) as usize)..(r as usize), [])
            .collect()::<String>();
    }

    let mut res = "".to_string();
    for i in 0..s.len() {
        let a = i as i32;
        let x = expand(&chars, a, a);
        let y = expand(&chars, a, a + 1);
    }

    "2".to_string()
}