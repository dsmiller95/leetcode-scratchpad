use core::panic;
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::write,
    num::ParseIntError,
};

struct Solution {}

struct AStarComboPather {
    pub target_point: CombinationPoint,
    pub blocked_points: Vec<CombinationPoint>,
    pub visited_points: HashMap<CombinationPoint, ComboVisitData>,
    pub frontier: BinaryHeap<PriorityHeapEntry<CombinationPoint, i16>>,
}

struct ComboVisitData {
    pub visited: bool,
    pub shortest_path_len: u16,
}

impl AStarComboPather {
    pub fn solve_path(&mut self, start: CombinationPoint) -> Option<u16> {
        self.add_frontier(start);

        while !self.frontier.is_empty() {
            let next_frontier = self.frontier.pop().unwrap().point;
            let Occupied(point_data) = self.visited_points.entry(next_frontier) else {
                panic!("must have entry for every item in frontier");
            };
            for neighbor in next_frontier.neighbors() {
                self.try_add_frontier(neighbor);
            }
        }
        todo!()
    }

    pub fn add_frontier(&mut self, point: CombinationPoint) {
        let hueristic: i16 = point.distance(&self.target_point).try_into().unwrap();
        self.frontier.push(PriorityHeapEntry {
            // binary heap takes maximum value off top. we want to minimize distance.
            priority: -hueristic,
            point,
        })
    }

    fn try_add_frontier(&self, neighbor: CombinationPoint) {
        todo!()
    }
}

#[derive(PartialEq, Eq, Hash)]
struct CombinationPoint {
    pub code: u16,
}

impl CombinationPoint {
    pub fn distance(&self, other: &Self) -> u16 {
        todo!()
    }
    pub fn neighbors(&self) -> impl Iterator<Item = CombinationPoint> {
        vec![].into_iter()
    }
}

impl TryFrom<String> for CombinationPoint {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let code: u16 = value.parse()?;
        Ok(Self { code })
    }
}

#[derive(PartialEq, Eq)]
struct PriorityHeapEntry<TData, TPriority>
where
    TPriority: Ord,
    TData: Eq,
{
    pub priority: TPriority,
    pub point: TData,
}

impl<TData, TPriority> PartialOrd for PriorityHeapEntry<TData, TPriority>
where
    TPriority: Ord,
    TData: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<TData, TPriority> Ord for PriorityHeapEntry<TData, TPriority>
where
    TPriority: Ord,
    TData: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target: u16 = target.parse().unwrap();
        let deadends: Vec<u16> = deadends.into_iter().map(|x| x.parse().unwrap()).collect();

        0
    }
}

mod tests {
    use std::fmt::write;

    use crate::n_725_open_the_lock::Solution;

    #[test]
    fn test_multiple_moves() {
        let dead_ends = ["0201", "0101", "0102", "1212", "2002"];
        let target = "0202";
        let expected_out = 6;
        let dead_ends = dead_ends.map(|x| x.into()).to_vec();
        assert_eq!(expected_out, Solution::open_lock(dead_ends, target.into()))
    }

    #[test]
    fn test_single_wrap_around() {
        let dead_ends = ["8888"];
        let target = "0009";
        let expected_out = 1;
        let dead_ends = dead_ends.map(|x| x.into()).to_vec();
        assert_eq!(expected_out, Solution::open_lock(dead_ends, target.into()))
    }

    #[test]
    fn test_blocked_by_dead_ends() {
        let dead_ends = [
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ];
        let target = "8888";
        let expected_out = -1;
        let dead_ends = dead_ends.map(|x| x.into()).to_vec();
        assert_eq!(expected_out, Solution::open_lock(dead_ends, target.into()))
    }
}
