use std::ops::Add;

/**
 *  https://leetcode.cn/problems/ransom-note/submissions/
 *  383. 赎金信
 */
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note == magazine {
        return true;
    }

    let mut cut: std::collections::HashMap<u8, i32> = std::collections::HashMap::new();

    for v in magazine.as_bytes().iter() {
        // 此处将解引用的值转移到vv, vv已经是新的值了，即使进行操作也仅仅只针对这里的vv
        // let mut vv = *cut.entry(*v).or_insert(0);
        // vv = vv + 1;

        // 并为赋值新变量，可行
        let vv = cut.entry(*v).or_insert(0);
        *vv = *vv + 1;
    }

    for index in ransom_note.as_bytes().iter() {
        let optional: Option<&i32> = cut.get(&index);
        match optional {
            Some(v) => {
                let mut rev = *v - 1;
                cut.insert(*index, rev);

                if rev < 0 {
                    return false;
                }
            }
            _ => {
                return false;
            }
        };
    }

    true
}

pub fn can_construct_one(ransom_note: String, magazine: String) -> bool {
    let mut arr = [0; 26];
    ransom_note
        .as_bytes()
        .iter()
        .for_each(|c| arr[(c - b'a') as usize] += 1);
    magazine
        .as_bytes()
        .iter()
        .for_each(|c| arr[(c - b'a') as usize] -= 1);
    (0..26).fold(true, |pre, i| pre && arr[i] >= 0)
}

#[cfg(test)]
mod tests {
    use crate::august::ransom::*;

    #[test]
    fn test() {
        assert_eq!(can_construct_one("a".to_string(), "b".to_string()), false);
    }
}
