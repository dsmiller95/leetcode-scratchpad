struct Solution{}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }

        let ord_a = Self::map_to_ordering_id(s.chars());
        let ord_b = Self::map_to_ordering_id(t.chars());

        ord_a.eq(ord_b)
    }

    fn map_to_ordering_id(bytes: impl Iterator<Item = char>) -> impl Iterator<Item = usize>{
        // use linear search, assuming total n is low, linear search may be fine.
        let mut remapped_so_far: Vec<char> = vec![];
        bytes.into_iter()
            .map(move |in_byte| {
                let index_of_unique = remapped_so_far
                    .iter()
                    .position(|x| *x == in_byte)
                    .unwrap_or_else(|| {
                        remapped_so_far.push(in_byte);
                        remapped_so_far.len() - 1
                    });

                index_of_unique
            })
    }
}


mod test {
    use super::*;
    #[test]
    fn test_true_egg_add(){
        assert_eq!(Solution::is_isomorphic("egg".to_string(), "add".to_string()), true);
    }
    #[test]
    fn test_false_eggg_add(){
        assert_eq!(Solution::is_isomorphic("eggg".to_string(), "add".to_string()), false);
    }

    #[test]
    fn ordering_ids(){
        let ordering: Vec<usize> = Solution::map_to_ordering_id("george".chars()).collect();
        assert_eq!(ordering, vec![0, 1, 2, 3, 0, 1])
    }
}