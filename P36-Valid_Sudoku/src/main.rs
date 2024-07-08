use std::collections::{HashMap, HashSet};

fn main() {
    
}


struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        /// Remplir 3 set :
        /// - set de lignes
        /// - set de colonnes
        /// - set de sous-grille
        /// Si une insertion retourne false alors nous retournons false
        
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut columns: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut sub_boxes: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for (y, row) in board.iter().enumerate() {
            for (x, digit) in row.iter().enumerate() {
                if *digit == '.' {
                    continue;
                }

                match rows.get_mut(&y) {
                    Some(digits) => {
                        if !digits.insert(*digit) {
                            return false;
                        }
                    },
                    None => {
                        let mut digits = HashSet::new();
                        digits.insert(*digit);
                        rows.insert(y, digits);
                    }
                }

                match columns.get_mut(&x) {
                    Some(digits) => {
                        if !digits.insert(*digit) {
                            return false;
                        }
                    },
                    None => {
                        let mut digits = HashSet::new();
                        digits.insert(*digit);
                        columns.insert(x, digits);
                    }
                }

                let sub_boxe_x_y = Solution::get_sub_boxe_x_y(y, x);

                match sub_boxes.get_mut(&sub_boxe_x_y) {
                    Some(digits) => {
                        if !digits.insert(*digit) {
                            return false;
                        }
                    },
                    None => {
                        let mut digits = HashSet::new();
                        digits.insert(*digit);
                        sub_boxes.insert(sub_boxe_x_y, digits);
                    }
                }
            }
        }

        true
    }

    fn get_sub_boxe_x_y(y: usize, x: usize) -> (usize, usize) {
        let sub_boxe_y = match y {
            v if v < 3 => 3,
            v if v < 6 => 6,
            v if v < 9 => 9,
            _ => unreachable!()
        };

        let sub_boxe_x = match x {
            v if v < 3 => 3,
            v if v < 6 => 6,
            v if v < 9 => 9,
            _ => unreachable!()
        };

        (sub_boxe_y, sub_boxe_x)
    }
}