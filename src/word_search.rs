use std::ops::{Index, IndexMut};

struct Solution{}


impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let x_size = board.len();
        let y_size = board[0].len();
        let mut visited_map = (0..x_size).map(
            |_| vec![false; y_size]
        ).collect();

        let char_vec: Vec<char> = word.chars().collect();
        let char_slice = char_vec.as_slice();

        let mut search_attempt_index = 0;
        for x in 0..x_size {
            for y in 0..y_size {
                search_attempt_index += 1;
                if Self::search_from_cell(&board, search_attempt_index, x, y, &mut visited_map, char_slice) {
                    return true;
                }
            }
        }


        false
    }

    pub fn search_from_cell(
        board: &Vec<Vec<char>>,
        search_index: usize,
        cell_x: usize,
        cell_y: usize,
        visited: &mut Vec<Vec<bool>>,
        word: &[char]) -> bool {

        if word.len() == 0 { return true }

        let Some(char_at_cell) = board.get(cell_x).and_then(|a| a.get(cell_y)) else {
            return false
        };

        if *char_at_cell != word[0] {
            return false;
        }

        let Some(visited_cell) = visited.get_mut(cell_x).and_then(|a| a.get_mut(cell_y)) else {
            return false;
        };

        if *visited_cell {
            return false;
        }

        *visited_cell = true;

        let (_, new_word) = word.split_at(1);

        let did_any_branch_match =
                          Self::search_from_cell(board, search_index, cell_x + 1, cell_y, visited, new_word) ||
            cell_x > 0 && Self::search_from_cell(board, search_index, cell_x - 1, cell_y, visited, new_word) ||
                          Self::search_from_cell(board, search_index, cell_x, cell_y + 1, visited, new_word) ||
            cell_y > 0 && Self::search_from_cell(board, search_index, cell_x, cell_y - 1, visited, new_word);

        // on unwind, reset visited matrix
        visited[cell_x][cell_y] = false;

        did_any_branch_match
    }
}



struct Vector2D {x: usize, y: usize}

struct Vec2DWrapper<T>{
    pub data: Vec<Vec<T>>
}

impl<T> Index<Vector2D> for Vec2DWrapper<T> {
    type Output = T;

    fn index(&self, index: Vector2D) -> &Self::Output {
        self.data
            .index(index.x)
            .index(index.y)
    }
}

impl<T> IndexMut<Vector2D> for Vec2DWrapper<T>{
    fn index_mut(&mut self, index: Vector2D) -> &mut Self::Output {
        self.data
            .index_mut(index.x)
            .index_mut(index.y)
    }
}

impl<T> Vec2DWrapper<T>{
    fn new(data: Vec<Vec<T>>) -> Vec2DWrapper<T> {
        // TODO: validate non-jagged?
        Vec2DWrapper {
            data
        }
    }

    fn size(&self) -> Vector2D {
        return Vector2D {x: self.data.len(), y: self.data[0].len()};
    }
}

impl<T> Vec2DWrapper<T> where T : Copy{
    fn new_blank(size: Vector2D, default: T) -> Vec2DWrapper<T> {
        let data = (0..size.x).map(
            |_| vec![default; size.y]
        ).collect();

        Vec2DWrapper{
            data
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn test_1(){
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']];
        let word = "ABCCED".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }
    #[test]
    fn test_2(){
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','E','S'],
            vec!['A','D','E','E']];
        let word = "ABCESEEEFS".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }
}