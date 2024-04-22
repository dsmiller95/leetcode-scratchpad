use core::panic;
use std::collections::hash_map::Entry;
use std::{
    collections::{BinaryHeap, HashMap},
    num::ParseIntError,
};

struct Solution {}

struct AStarComboPather {
    pub target_point: CombinationPoint,
    pub blocked_points: Vec<CombinationPoint>,
    pub visited_points: HashMap<CombinationPoint, ComboVisitData>,
    pub frontier: BinaryHeap<PriorityHeapEntry<CombinationPoint, i16>>,
}

#[derive(Clone, Copy)]
struct ComboVisitData {
    pub shortest_path_len: u16,
}

impl AStarComboPather {
    fn new(target_point: CombinationPoint, blocked_points: Vec<CombinationPoint>) -> Self {
        Self {
            target_point,
            blocked_points,
            visited_points: HashMap::new(),
            frontier: BinaryHeap::new(),
        }
    }

    pub fn solve_path(&mut self, start: CombinationPoint) -> Option<u16> {
        self.visit_point(start, 0);

        while !self.frontier.is_empty() {
            let next_frontier = self.frontier.pop().unwrap().point;
            let point_data = self.visited_points.get(&next_frontier).unwrap().to_owned();
            for neighbor in next_frontier.neighbors() {
                self.visit_point(neighbor, point_data.shortest_path_len + 1)
            }

            if let Some(visited_data) = self.visited_points.get(&self.target_point) {
                return Some(visited_data.shortest_path_len);
            }
        }

        None
    }

    pub fn visit_point(&mut self, point: CombinationPoint, dist_from_start: u16) {
        let visited_data_entry = match self.visited_points.entry(point) {
            // if an entry exists and is a shorter distance than this entry, noop. we've already
            // visited it and dont need to again.
            Entry::Occupied(existing) if existing.get().shortest_path_len < dist_from_start => {
                return;
            }
            existing => existing,
        };
        let hueristic: i16 = point.distance(&self.target_point).into();
        self.frontier.push(PriorityHeapEntry {
            // binary heap takes maximum value off top. we want to minimize distance.
            priority: -hueristic,
            point,
        });
        let visit_data = ComboVisitData {
            shortest_path_len: dist_from_start,
        };

        let entry_data = visited_data_entry.or_insert(visit_data);
        *entry_data = visit_data;
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct CombinationPoint {
    pub code: u16,
}

impl CombinationPoint {
    pub fn distance(&self, other: &Self) -> u8 {
        let comp_a = self.components();
        let comp_b = other.components();
        let mut res: u8 = 0;
        for i in 0..4 {
            res += comp_a[i] + comp_b[i];
        }
        res
    }
    pub fn neighbors(&self) -> impl Iterator<Item = CombinationPoint> {
        vec![].into_iter()
    }
    fn components(&self) -> [u8; 4] {
        let mut code = self.code;
        let a = (code % 10) as u8;
        code /= 10;
        let b = (code % 10) as u8;
        code /= 10;
        let c = (code % 10) as u8;
        code /= 10;
        let d = (code % 10) as u8;
        [a, b, c, d]
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
        let target: CombinationPoint = target.try_into().unwrap();
        let deadends: Vec<CombinationPoint> = deadends
            .into_iter()
            .map(|x| x.try_into().unwrap())
            .collect();
        let mut solver = AStarComboPather::new(target, deadends);
        let start: CombinationPoint = "0000".to_string().try_into().unwrap();

        match solver.solve_path(start) {
            Some(x) => x.into(),
            None => -1,
        }
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
