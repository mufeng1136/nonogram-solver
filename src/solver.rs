use crate::solver::utils::integer_partitions;

pub struct NonogramSolver {
    row: usize,
    col: usize,
    row_clues: Vec<Vec<usize>>,
    col_clues: Vec<Vec<usize>>,
    rows_possibilities: Vec<Vec<Vec<bool>>>,
    cols_possibilities: Vec<Vec<Vec<bool>>>,
    // 0 means empty, 1 means filled, 2 means unknown
    certain_grids_from_input: Vec<Vec<usize>>,
    certain_grids: Vec<Vec<usize>>,
    unsolvable: bool,
    solved: bool,
    valid: bool,
    cols_current: Vec<Vec<usize>>,
    rows_possibilities_index: Vec<usize>,
}

impl NonogramSolver {
    pub fn new(row: usize, col: usize) -> Self {
        NonogramSolver {
            row,
            col,
            row_clues: Vec::new(),
            col_clues: Vec::new(),
            rows_possibilities: Vec::new(),
            cols_possibilities: Vec::new(),
            unsolvable: false,
            solved: false,
            valid: false,
            cols_current: Vec::new(),
            rows_possibilities_index: Vec::new(),
            certain_grids_from_input: vec![vec![2; col]; row],
            certain_grids: vec![vec![2; col]; row],
        }
    }

    pub fn set_row_clues(&mut self, clues: &Vec<Vec<usize>>) {
        if clues.len() != self.row {
            eprintln!("Warning: Number of row clues does not match the number of rows");
            return;
        }
        self.row_clues = clues.clone();
        self.valid = false;
        self.unsolvable = false;
        self.solved = false;
    }

    pub fn set_col_clues(&mut self, clues: &Vec<Vec<usize>>) {
        if clues.len() != self.col {
            eprintln!("Warning: Number of column clues does not match the number of columns");
            return;
        }
        self.col_clues = clues.clone();
        self.valid = false;
        self.unsolvable = false;
        self.solved = false;
    }

    pub fn show_state(&self) {
        println!("row: {}, col: {}", self.row, self.col);

        println!("Row clues:");
        for (i, clue) in self.row_clues.iter().enumerate() {
            println!("  Row {}: {:?}", i + 1, clue);
        }

        println!("Column clues:");
        for (i, clue) in self.col_clues.iter().enumerate() {
            println!("  Column {}: {:?}", i + 1, clue);
        }

        println!("Valid: {}, Solved: {}", self.valid, self.solved);
    }

    fn generate_row_possibilities(&mut self) {
        self.rows_possibilities = vec![Vec::new(); self.row];
        for (i, row_clue) in self.row_clues.iter().enumerate() {
            let partitions: Vec<Vec<usize>> = utils::integer_partitions(
                self.col + 2 - row_clue.iter().sum::<usize>(),
                row_clue.len() + 1,
            );
            'different_partition: for partition in &partitions {
                let possibility: Vec<bool> =
                    utils::generate_possibility_from_partition(partition, row_clue);
                for j in 0..self.col {
                    if self.certain_grids[i][j] == 1 && possibility[j] == false {
                        continue 'different_partition;
                    }
                    if self.certain_grids[i][j] == 0 && possibility[j] == true {
                        continue 'different_partition;
                    }
                }
                self.rows_possibilities[i].push(possibility);
            }
        }
    }

    fn generate_col_possibilities(&mut self) {
        self.cols_possibilities = vec![Vec::new(); self.col];
        for (i, col_clue) in self.col_clues.iter().enumerate() {
            let partitions: Vec<Vec<usize>> = utils::integer_partitions(
                self.row + 2 - col_clue.iter().sum::<usize>(),
                col_clue.len() + 1,
            );
            'different_partition: for partition in &partitions {
                let possibility: Vec<bool> =
                    utils::generate_possibility_from_partition(partition, col_clue);
                for j in 0..self.row {
                    if self.certain_grids[j][i] == 1 && possibility[j] == false {
                        continue 'different_partition;
                    }
                    if self.certain_grids[j][i] == 0 && possibility[j] == true {
                        continue 'different_partition;
                    }
                }
                self.cols_possibilities[i].push(possibility);
            }
        }
    }

    pub fn show_answer(&self) {
        if self.solved && !self.unsolvable {
            println!("Solution:");
            for (r, i) in self.rows_possibilities_index.iter().enumerate() {
                let row: &Vec<bool> = &self.rows_possibilities[r][*i];
                print!("{:2}: ", r + 1);
                for &cell in row.iter() {
                    if cell == true {
                        print!("██");
                    } else {
                        print!("  ");
                    }
                }
            }
        } else if !self.unsolvable {
            println!("The puzzle is not yet solved.");
        } else {
            println!("The puzzle is unsolvable.");
        }
    }
}

mod utils {

    pub fn integer_partitions(n: usize, k: usize) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::new();
        let mut current: Vec<usize> = Vec::new();
        fn helper(n: usize, k: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
            if k == 1 {
                let mut composition: Vec<usize> = current.clone(); // 复制当前前缀
                composition.push(n); // 相当于 current + [n]
                result.push(composition); // 收集到结果中
                return;
            }
            if k > n {
                return;
            }
            for i in 1..=(n - k + 1) {
                current.push(i);
                helper(n - i, k - 1, current, result);
                current.pop();
            }
        }
        helper(n, k, &mut current, &mut result);
        return result;
    }

    pub fn generate_possibility_from_partition(
        partition: &Vec<usize>,
        clue: &Vec<usize>,
    ) -> Vec<bool> {
        let mut result: Vec<bool> = Vec::new();
        result.extend(vec![false; partition[0] - 1]);
        for (i, &block_len) in clue.iter().enumerate() {
            result.extend(vec![true; block_len]);
            result.extend(vec![false; partition[i + 1]]);
        }
        result.pop(); // 移除最后一个多余的
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let test_solver = NonogramSolver::new(10, 10);
        test_solver.show_state();
        test_solver.show_answer();
    }

    #[test]
    fn test_integer_partitions() {
        let partitions = utils::integer_partitions(5, 2);
        assert_eq!(
            partitions,
            vec![vec![1, 4], vec![2, 3], vec![3, 2], vec![4, 1]]
        );
    }

    #[test]
    fn test_generate_possibility_from_partition() {
        let partition = vec![1, 2, 1];
        let clue = vec![2, 3];
        let possibility = utils::generate_possibility_from_partition(&partition, &clue);
        assert_eq!(
            possibility,
            vec![true, true, false, false, true, true, true]
        );
    }

    #[test]
    fn test_generate_row_possibilities() {
        let mut solver = NonogramSolver::new(5, 5);
        solver.row_clues = vec![vec![2], vec![1, 1], vec![3], vec![1], vec![2]];
        solver.col_clues = vec![vec![1], vec![2], vec![1, 1], vec![2], vec![1]];
        solver.certain_grids[0][0] = 1;
        solver.generate_row_possibilities();
        solver.generate_col_possibilities();

        for i in 0..5 {
            println!("Row {} possibilities:", i + 1);
            for possibility in &solver.rows_possibilities[i] {
                println!("{:?}", possibility);
            }
        }
        for i in 0..5 {
            println!("Col {} possibilities:", i + 1);
            for possibility in &solver.cols_possibilities[i] {
                println!("{:?}", possibility);
            }
        }
    }
}
