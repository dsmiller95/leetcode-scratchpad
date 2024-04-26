struct Solution {}

use std::{cell::Cell, cmp::Ordering};
struct OrderedSlice<TItem, const SIZE: usize> {
    pub sorted_items: [Option<TItem>; SIZE],
}

impl<TItem, const SIZE: usize> OrderedSlice<TItem, SIZE>
where
    TItem: Copy,
{
    fn new() -> Self {
        Self {
            sorted_items: [None; SIZE],
        }
    }
}

impl<TItem, const SIZE: usize> OrderedSlice<TItem, SIZE>
where
    TItem: Ord,
{
    fn push(&mut self, item: TItem) {
        let Some(swap_idx) = self.get_swap_index(&item) else {
            return;
        };

        let swap_item = self.sorted_items.get_mut(swap_idx).unwrap();

        let old_item = swap_item.replace(item);
        let Some(old_item) = old_item else {
            return;
        };

        self.push_forward_from(swap_idx + 1, old_item);
    }

    fn get_swap_index(&self, item: &TItem) -> Option<usize> {
        for (idx, item_opt) in self.sorted_items.iter().enumerate() {
            let Some(existing_item) = item_opt else {
                // no items here. place our item inside. done.
                return Some(idx);
            };

            // incoming item is less than existing item
            let will_swap = item.cmp(existing_item) == Ordering::Less;
            if !will_swap {
                continue;
            }

            return Some(idx);
        }

        None
    }

    fn push_forward_from(&mut self, first_swap_index: usize, mut carry_item: TItem) {
        for item_opt in self.sorted_items[first_swap_index..].iter_mut() {
            // todo: extract to function just to push forward?
            let swap = item_opt.take();
            *item_opt = Some(carry_item);
            let Some(swap) = swap else {
                return;
            };

            carry_item = swap;
        }
    }

    fn push_maybe(&mut self, best_next: Option<TItem>) {
        let Some(item) = best_next else {
            return;
        };
        self.push(item)
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct OrderedEntry<TData, TPriority>
where
    TPriority: Ord,
    TData: Eq,
{
    pub ordered_by: TPriority,
    pub data: TData,
}

impl<TData, TPriority> PartialOrd for OrderedEntry<TData, TPriority>
where
    TPriority: Ord,
    TData: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<TData, TPriority> Ord for OrderedEntry<TData, TPriority>
where
    TPriority: Ord,
    TData: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordered_by.cmp(&other.ordered_by)
    }
}

#[derive(Clone, Copy)]
struct CellValue {
    index: usize,
    best_value: i32,
}

const OPT_SIZE: usize = 3;
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let best_3_per_row = grid.into_iter().map(|x| {
            let mut slice = OrderedSlice::<_, OPT_SIZE>::new();
            for (idx, cell_val) in x.into_iter().enumerate() {
                let entry = OrderedEntry {
                    ordered_by: cell_val,
                    data: idx,
                };
                slice.push(entry);
            }
            slice.sorted_items
        });
        let mut last_case = None;

        for best_in_row in best_3_per_row {
            let Some(last_case) = last_case.as_mut() else {
                last_case = Some(best_in_row);
                continue;
            };

            let mut best_choices = OrderedSlice::<_, OPT_SIZE>::new();

            for entry in best_in_row {
                let best_next = Solution::get_best_next(&entry, last_case);

                best_choices.push_maybe(best_next);
            }

            *last_case = best_choices.sorted_items;
        }

        last_case.unwrap()[0].unwrap().ordered_by
    }

    fn get_best_next<const SIZE: usize>(
        entry: &Option<OrderedEntry<usize, i32>>,
        last_case: &[Option<OrderedEntry<usize, i32>>; SIZE],
    ) -> Option<OrderedEntry<usize, i32>> {
        let entry = entry.as_ref()?;
        let best_choice = Solution::find_best(last_case, entry.data)?;
        Some(OrderedEntry {
            ordered_by: best_choice + entry.ordered_by,
            data: entry.data,
        })
    }

    fn find_best<const SIZE: usize>(
        choices: &[Option<OrderedEntry<usize, i32>>; SIZE],
        previous_index: usize,
    ) -> Option<i32> {
        for choice in choices {
            match choice {
                None => {
                    return None;
                }
                Some(OrderedEntry { data, .. }) if *data == previous_index => {
                    continue;
                }
                Some(OrderedEntry { ordered_by, .. }) => {
                    return Some(*ordered_by);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests_ordered_slice {
    use super::OrderedSlice;

    #[test]
    fn test_fills_with_numbers() {
        let mut slice = OrderedSlice::<_, 4>::new();
        slice.push(3);
        slice.push(4);
        slice.push(5);
        slice.push(6);

        assert_eq!([Some(3), Some(4), Some(5), Some(6)], slice.sorted_items);
    }

    #[test]
    fn test_fills_with_numbers_sorted() {
        let mut slice = OrderedSlice::<_, 4>::new();
        slice.push(5);
        slice.push(4);
        slice.push(3);
        slice.push(2);

        assert_eq!([Some(2), Some(3), Some(4), Some(5)], slice.sorted_items);
    }
}

#[cfg(test)]
mod tests {
    use crate::n_1289_minimum_falling_path_sum_II::Solution;

    #[test]
    fn single_grid_single_path() {
        let grid = [[7]];
        let expected = 7;

        let grid = grid.map(|x| x.to_vec()).to_vec();
        assert_eq!(expected, Solution::min_falling_path_sum(grid))
    }

    #[test]
    fn small_grid_best_path() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected = 13;

        let grid = grid.map(|x| x.to_vec()).to_vec();
        assert_eq!(expected, Solution::min_falling_path_sum(grid))
    }
}
