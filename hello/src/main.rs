struct Solution;

static LOOKUP: [usize; 10] = {
    let mut l = [0; 10];

    for i in 0..10 {
        l[i] = i.pow(2);
    }

    l
};

impl Solution {

    fn get_next(n: i32) -> i32 {
        let s = n.to_string();
        //return s.chars().fold(|acc, x| 1);
        3
    }

    pub fn is_happy(n: i32) -> bool {
        true
    }
}

fn main() {
    println!("{}", '1' as u32);
    println!("{}", Solution::is_happy(19));
}
