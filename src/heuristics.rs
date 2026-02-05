use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            // blind heuristic always returns 0
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut sum = 0;
                for i in 0..=8 {
                    if !(board.value_at(i/3, i%3) == i as u8) {
                        sum +=1;
                    }
                }
                sum
            }
            // TODO
            // Can't just use sort, since 0 is at the end of the list and not at the beginning
            Heuristic::Manhattan => {
                let mut sum = 0;
                for i in 1..=8 {
                    let pos = board.position(i);

                    let index: i8 = i as i8;
                    let row: i8 = pos.0 as i8;
                    let col: i8 = pos.1 as i8;


                    sum += (row - (index/3)).abs() + (col - index%3).abs();
                    println!("{} {}", row, index/3);
                }
                sum as u32
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 2+3+3);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 16);
    }
}
