use std::cmp;


// ()()(())
pub fn score_of_parentheses(s: String) -> i32 {
    let mut stack = Vec::new();
    stack.push(0);                                              // [1]

    for byte_ptr in s.as_bytes() {
        if *byte_ptr == b'(' {
            stack.push(0);                                      // [1, 0]
        } else {
            let v = stack.pop().unwrap();
            let w = stack.pop().unwrap();
            stack.push(w + cmp::max(2 * v, 1));          // [1, 0]
        }
    }
    stack.pop().unwrap()
}