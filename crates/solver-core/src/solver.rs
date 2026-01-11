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
    // #[allow(dead_code)] does not comply with Rust's standards, but just suppress the warning for now.
    //（这样不符合Rust规范，但只管先抑制警告）
    #[allow(dead_code)] 
    cols_current: Vec<Vec<usize>>,
    #[allow(dead_code)] 
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

    pub fn set_known_grid(&mut self, grid: &Vec<Vec<u8>>) -> Result<(), String> {
        if grid.len() != self.row {
            return Err("Known grid row count does not match".to_string());
        }
        for (r, row) in grid.iter().enumerate() {
            if row.len() != self.col {
                return Err(format!("Known grid col count does not match at row {}", r));
            }
            for (c, &v) in row.iter().enumerate() {
                if v > 2 {
                    return Err(format!("Invalid cell value at ({}, {}): {}", r, c, v));
                }
            }
        }

        self.certain_grids_from_input = grid
            .iter()
            .map(|r| r.iter().map(|&v| v as usize).collect())
            .collect();
        self.certain_grids = self.certain_grids_from_input.clone();
        self.valid = false;
        self.unsolvable = false;
        self.solved = false;
        Ok(())
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

    fn update_certain_grids(&mut self) -> bool {
        let mut updated = false;
        for i in 0..self.row {
            let certain_in_row = utils::find_certain_grids(&self.rows_possibilities[i]);
            for j in 0..self.col {
                if certain_in_row[j] != 2 {
                    if self.certain_grids[i][j] == 2 {
                        updated = true;
                    }
                    self.certain_grids[i][j] = certain_in_row[j];
                }
            }
        }
        for j in 0..self.col {
            let certain_in_col = utils::find_certain_grids(&self.cols_possibilities[j]);
            for i in 0..self.row {
                if certain_in_col[i] != 2 {
                    if self.certain_grids[i][j] == 2 {
                        updated = true;
                    }
                    self.certain_grids[i][j] = certain_in_col[i];
                }
            }
        }
        updated
    }

    pub fn check_valid(&self) -> bool {
        // 检查 row_clues 的数量是否与 row 一致
        if self.row_clues.len() != self.row {
            eprintln!("Invalid: Number of row clues does not match the number of rows.");
            return false;
        }

        // 检查 row_clues 中的每个 clue 是否全为正整数，且和不超过 col
        for (i, row_clue) in self.row_clues.iter().enumerate() {
            if row_clue.iter().any(|&clue| clue == 0) {
                eprintln!("Invalid: Row {} contains non-positive clues.", i + 1);
                return false;
            }
            let required = row_clue.iter().sum::<usize>()
                + if row_clue.is_empty() {
                    0
                } else {
                    row_clue.len() - 1
                };
            if required > self.col {
                eprintln!("Invalid: Row {} clues exceed the column limit.", i + 1);
                return false;
            }
        }

        // 检查 col_clues 的数量是否与 col 一致
        if self.col_clues.len() != self.col {
            eprintln!("Invalid: Number of column clues does not match the number of columns.");
            return false;
        }

        // 检查 col_clues 中的每个 clue 是否全为正整数，且和不超过 row
        for (i, col_clue) in self.col_clues.iter().enumerate() {
            if col_clue.iter().any(|&clue| clue == 0) {
                eprintln!("Invalid: Column {} contains non-positive clues.", i + 1);
                return false;
            }
            let required = col_clue.iter().sum::<usize>()
                + if col_clue.is_empty() {
                    0
                } else {
                    col_clue.len() - 1
                };
            if required > self.row {
                eprintln!("Invalid: Column {} clues exceed the row limit.", i + 1);
                return false;
            }
        }

        // 如果所有检查都通过，返回 true
        true
    }

    fn check_zero_possibilities(&self) -> bool {
        for i in 0..self.row {
            if self.rows_possibilities[i].is_empty() {
                return true;
            }
        }
        for j in 0..self.col {
            if self.cols_possibilities[j].is_empty() {
                return true;
            }
        }
        false
    }

    fn check_solved(&self) -> bool {
        for i in 0..self.row {
            for j in 0..self.col {
                if self.certain_grids[i][j] == 2 {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve(&mut self) {
        self.valid = self.check_valid();
        if !self.valid {
            return;
        }
        let mut updated = true;
        while updated {
            self.generate_row_possibilities();
            self.generate_col_possibilities();
            if self.check_zero_possibilities() {
                self.unsolvable = true;
                println!("The puzzle is unsolvable. (no solution)");
                return;
            }
            updated = self.update_certain_grids();
            if !updated {
                break;
            }
        }
        self.solved = self.check_solved();
        if self.solved {
            println!("The puzzle is solved uniquely!");
        } else {
            println!("The puzzle is unsolvable. (multiple solutions)");
        }
    }

    pub fn show_answer(&self) {
        if self.solved && !self.unsolvable {
            println!("row: {}, col: {}", self.row, self.col);

            // Determine the maximum width of row clues for alignment
            let max_row_clue_width = self
                .row_clues
                .iter()
                .map(|clue| clue.len())
                .max()
                .unwrap_or(0);

            // Print column clues at the top, aligned above each 3-char grid cell.
            // Each grid cell occupies `col_clue_width` characters (3).
            let col_clue_width = 3; // width per column cell
            // left padding to account for the printed row clues area (plus one separating space)
            let left_padding = max_row_clue_width * col_clue_width;
            // find the maximum column-clue height so we can print from top to bottom
            let max_col_clue_height = self.col_clues.iter().map(|c| c.len()).max().unwrap_or(0);

            for clue_row in 0..max_col_clue_height {
                // print left padding for the row clues column
                print!("{:width$}", "", width = left_padding);

                // For each column, compute its start row (so clues are bottom-aligned)
                for col_clue in &self.col_clues {
                    let start_row = max_col_clue_height.saturating_sub(col_clue.len());
                    if clue_row < start_row {
                        // this column has no clue at this top row -> print empty cell width
                        print!("{:>width$}", "", width = col_clue_width);
                    } else {
                        // print the clue number corresponding to this row
                        let idx = clue_row - start_row;
                        print!("{:>width$}", col_clue[idx], width = col_clue_width);
                    }
                }
                println!();
            }

            // Adjust row clue spacing to 3 spaces for alignment
            for (r, row) in self.certain_grids.iter().enumerate() {
                if let Some(clue) = self.row_clues.get(r) {
                    let clue_str: String = clue
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");
                    print!("{:>width$} ", clue_str, width = max_row_clue_width * 3);
                }

                // Print the grid row
                // Adjust grid cell width to 3 spaces
                for (c_idx, &cell) in row.iter().enumerate() {
                    if cell == 1 {
                        // filled cell: alternate blue/magenta background
                        if (r + c_idx) % 2 == 0 {
                            print!("\x1b[44m   \x1b[0m");
                        } else {
                            print!("\x1b[45m   \x1b[0m");
                        }
                    } else if cell == 0 {
                        // empty cell: plain spaces
                        print!("   ");
                    } else {
                        print!("...");
                    }
                }
                println!();
            }
        } else if !self.unsolvable {
            println!("The puzzle is not yet solved.");
        } else {
            println!("The puzzle is unsolvable.");
        }
    }

    pub fn grid(&self) -> &Vec<Vec<usize>> {
        &self.certain_grids
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_solved(&self) -> bool {
        self.solved
    }

    pub fn is_unsolvable(&self) -> bool {
        self.unsolvable
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
        result
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
        result
    }

    pub fn find_certain_grids(possibilities: &Vec<Vec<bool>>) -> Vec<usize> {
        let len = possibilities[0].len();
        let mut certain_grids: Vec<usize> = vec![2; len];
        for j in 0..len {
            let mut all_filled = true;
            let mut all_empty = true;
            for possibility in possibilities {
                if possibility[j] == true {
                    all_empty = false;
                } else {
                    all_filled = false;
                }
            }
            if all_filled {
                certain_grids[j] = 1;
            } else if all_empty {
                certain_grids[j] = 0;
            }
        }
        certain_grids
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

    #[test]
    fn test_find_certain_grids() {
        let possibilities = vec![
            vec![true, false, true, false],
            vec![true, false, true, false],
            vec![true, true, true, false],
        ];
        let certain_grids = utils::find_certain_grids(&possibilities);
        assert_eq!(certain_grids, vec![1, 2, 1, 0]);
    }

    #[test]
    fn test_update_certain_grids() {
        let mut solver = NonogramSolver::new(3, 3);
        dbg!(&solver.certain_grids);
        solver.rows_possibilities = vec![
            vec![vec![true, false, true], vec![true, false, true]],
            vec![vec![false, true, false], vec![false, true, false]],
            vec![vec![true, true, true], vec![true, true, true]],
        ];
        solver.cols_possibilities = vec![
            vec![vec![true, false, true], vec![true, false, true]],
            vec![vec![false, true, true], vec![false, true, true]],
            vec![vec![true, false, true], vec![true, false, true]],
        ];
        let _updated = solver.update_certain_grids();
        dbg!(&solver.certain_grids);
    }

    #[test]
    fn test_solver_15() {
        let row_clues: Vec<Vec<usize>> = vec![
            vec![6, 1, 2],
            vec![5, 1, 3],
            vec![8, 5],
            vec![1, 3, 1, 5],
            vec![9],
            //
            vec![1, 1, 6],
            vec![1, 1, 2, 1],
            vec![1, 3, 1, 1],
            vec![2, 1],
            vec![6, 2],
            //
            vec![2, 7],
            vec![2, 5],
            vec![6, 1],
            vec![3, 1, 1],
            vec![3, 1, 1],
        ];

        let col_clues: Vec<Vec<usize>> = vec![
            vec![4, 3],
            vec![3, 3],
            vec![4, 1],
            vec![4, 3, 1],
            vec![5, 1, 2],
            //
            vec![1, 1, 4, 3],
            vec![1, 1, 1, 1, 2],
            vec![4, 1, 1, 1],
            vec![1, 3],
            vec![6, 3],
            //
            vec![6, 3],
            vec![5, 6],
            vec![5, 2],
            vec![4, 1, 2, 2],
            vec![2, 6],
        ];
        let mut solver = NonogramSolver::new(15, 15);
        solver.set_row_clues(&row_clues);
        solver.set_col_clues(&col_clues);
        solver.solve();

        for i in 0..15 {
            println!("row {}", i);
            println!("{:?}", solver.certain_grids[i]);
        }
        dbg!(&solver.valid);
        dbg!(&solver.unsolvable);
        dbg!(&solver.solved);
    }

    #[test]
    fn test_solver_3() {
        let row_clues: Vec<Vec<usize>> = vec![vec![3], vec![3], vec![3]];

        let col_clues: Vec<Vec<usize>> = vec![vec![3], vec![3], vec![3]];
        let mut solver = NonogramSolver::new(3, 3);
        solver.set_row_clues(&row_clues);
        solver.set_col_clues(&col_clues);
        solver.solve();

        for i in 0..3 {
            println!("row {}", i);
            println!("{:?}", solver.certain_grids[i]);
        }
        dbg!(&solver.valid);
        dbg!(&solver.unsolvable);
        dbg!(&solver.solved);
    }

    #[test]
    fn test_solver_3_detailed() {
        let row_clues: Vec<Vec<usize>> = vec![vec![3], vec![3], vec![3]];

        let col_clues: Vec<Vec<usize>> = vec![vec![3], vec![3], vec![3]];
        let mut solver = NonogramSolver::new(3, 3);
        solver.set_row_clues(&row_clues);
        solver.set_col_clues(&col_clues);
        solver.generate_col_possibilities();
        dbg!(&solver.cols_possibilities);
        solver.generate_row_possibilities();
        dbg!(&solver.rows_possibilities);
        let updated = solver.update_certain_grids();
        dbg!(&updated);
        dbg!(&solver.certain_grids);
        solver.generate_col_possibilities();
        solver.generate_row_possibilities();
        let updated = solver.update_certain_grids();
        dbg!(&updated);
    }
}
