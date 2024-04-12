struct Solution{}
use std::cmp::{max, min};
use std::iter::zip;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let max_wrap_fn = |max_so_far: &mut i32, &x: &i32|{
            *max_so_far = x.max(*max_so_far);
            Some(*max_so_far)
        };
        let max_from_right = height.iter().rev()
            .scan(0, max_wrap_fn)
            .collect::<Vec<_>>().into_iter().rev();
        let max_from_left = zip(height, max_from_right).scan(0, |max_left, (ht, max_right)| {
            *max_left = ht.max(*max_left);
            let sea_level = min(*max_left, max_right);
            let capacity = max(0, sea_level - ht);
            Some(capacity)
        });
        max_from_left.sum()
    }
}