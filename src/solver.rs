pub struct NonogramSolver {
    row: usize,
    col: usize,
    row_clues: Option<Vec<Vec<usize>>>,
    col_clues: Option<Vec<Vec<usize>>>,
    rows_possibilities: Vec<Vec<Vec<usize>>>,
    cols_possibilities: Vec<Vec<Vec<usize>>>,
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
            certain_grids: Vec::new(),
            unsolvable: false,
            solved: false,
            valid: false,
            cols_current: Vec::new(),
            rows_possibilities_index: Vec::new(),
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let mut TestSolver = NonogramSolver::new(10, 10);
        TestSolver.show_state();
        TestSolver.show_answer();
    }
}