pub struct NonogramSolver {
    row: usize,
    col: usize,
    row_clues: Option<Vec<Vec<usize>>>,
    col_clues: Option<Vec<Vec<usize>>>,
    rows_possibilities: Vec<Vec<Vec<usize>>>,
    cols_possibilities: Vec<Vec<Vec<usize>>>,
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
            row_clues: None,
            col_clues: None,
            rows_possibilities: Vec::new(),
            cols_possibilities: Vec::new(),
            unsolvable: false,
            solved: false,
            valid: false,
            cols_current: Vec::new(),
            rows_possibilities_index: Vec::new(),
            certain_grids_from_input: Vec::new(),
            certain_grids: Vec::new(),
        }
    }

    pub fn set_row_clues(&mut self, clues: &Vec<Vec<usize>>) {
        if clues.len() != self.row {
            eprintln!("Warning: Number of row clues does not match the number of rows");
            return;
        }
        self.row_clues = Some(clues.clone());
        self.valid = false;
        self.unsolvable = false;
        self.solved = false;
    }

    pub fn set_col_clues(&mut self, clues: &Vec<Vec<usize>>) {
        if clues.len() != self.col {
            eprintln!("Warning: Number of column clues does not match the number of columns");
            return;
        }
        self.col_clues = Some(clues.clone());
        self.valid = false;
        self.unsolvable = false;
        self.solved = false;
    }

    pub fn show_state(&self) {
        println!("row: {}, col: {}", self.row, self.col);
        match self.col_clues {
            None => println!("Column clues: None"),
            Some(ref clues) => {
                println!("Column clues:");
                for (i, clue) in clues.iter().enumerate() {
                    println!("  Column {}: {:?}", i + 1, clue);
                }
            }
        }
        match self.row_clues {
            None => println!("Column clues: None"),
            Some(ref clues) => {
                println!("Column clues:");
                for (i, clue) in clues.iter().enumerate() {
                    println!("  Column {}: {:?}", i + 1, clue);
                }
            }
        }
        println!("Valid: {}, Solved: {}", self.valid, self.solved);
    }

    pub fn show_answer(&self) {
        if self.solved && !self.unsolvable {
            println!("Solution:");
            for (r, i) in self.rows_possibilities_index.iter().enumerate() {
                let row: &Vec<usize> = &self.rows_possibilities[r][*i];
                print!("{:2}: ", r + 1);
                for &cell in row.iter() {
                    if cell == 1 {
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
}
