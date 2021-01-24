use std::collections::HashMap;
struct Solution;

static LOOKUP_POW: [i32; 10] = [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
const zero: usize = '0' as usize;

impl Solution {

    #[inline]
    fn char_pow_2(c: char) -> i32 {
        LOOKUP_POW[(c as usize) - zero]
    }

    fn get_next(n: i32) -> i32 {
        let s = n.to_string();
        return s.chars().fold(0, |acc, c| acc + Solution::char_pow_2(c));
    }

    pub fn is_happy(n: i32) -> bool {
        let mut cache: HashMap<i32, bool> = HashMap::new();
        let mut result = n;
        loop {
            if result == 1 {
                return true
            }
            cache.get(k)
        }
    }
}

fn main() {
    println!("{}", '1' as u32);
    println!("{}", Solution::is_happy(19));
}
