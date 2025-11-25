from itertools import chain
from itertools import groupby
import logging

# Module logger
logger = logging.getLogger(__name__)
def integer_partitions(
    n: int, k: int, current: list[int] | None = None
) -> list[list[int]]:
    """Generate all ordered compositions of integer n into k parts.

    Args:
        n: The integer to partition
        k: The number of parts to partition into
        current: Current partition being built (used internally for recursion)

    Returns:
        A list of all possible compositions, where each composition is a list of k positive integers
        that sum to n. Different orderings are treated as different compositions.

    Example:
        >>> integer_partitions(5, 2)
        [[1, 4], [2, 3], [3, 2], [4, 1]]
        >>> integer_partitions(4, 3)
        [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
    """
    if current is None:
        current = []

    if k == 1:
        return [current + [n]]

    if k > n:
        return []

    result = []

    for i in range(1, n - k + 2):
        result.extend(integer_partitions(n - i, k - 1, current + [i]))

    return result

class NonogramSolver:
    def __init__(
        self,
        row: int,
        col: int,
        row_clues: list[list[int]] | None = None,
        col_clues: list[list[int]] | None = None,
    ):
        self.row_clues: list = row_clues
        self.col_clues: list = col_clues
        self.row: int = row
        self.col: int = col
        self.rows_possible: list[list[list[int]]] = []
        self.cols_current: list[list[int]] = [[0] for _ in range(col)]
        self.solved: bool = False
        self.ready_to_solve: bool = False
        self.rows_possible_index: list[int] = [0 for _ in range(row)]

    def row_clues(self, row_clues: list[list[int]]) -> None:
        self.row_clues = row_clues
        self.ready_to_solve = False

    def col_clues(self, col_clues: list[list[int]]) -> None:
        self.col_clues = col_clues
        self.ready_to_solve = False

    def check_ready(self) -> bool:
        """Check if the solver is ready to solve and validate clues.

        Validates:
        - Clues are not None
        - Number of clues matches grid dimensions
        - Each row clue sum doesn't exceed column count
        - Each column clue sum doesn't exceed row count
        - Clue values are positive integers
        """
        if self.row_clues is None or self.col_clues is None:
            self.ready_to_solve = False
            return False

        if len(self.row_clues) != self.row or len(self.col_clues) != self.col:
            self.ready_to_solve = False
            return False

        # Validate row clues
        for i, clue in enumerate(self.row_clues):
            # Check for empty or invalid clues
            if not isinstance(clue, list):
                self.ready_to_solve = False
                return False

            # Check all values are positive integers
            if any(val <= 0 for val in clue):
                self.ready_to_solve = False
                return False

            # Check if clue can fit in the row
            # Minimum cells needed: sum of clues + gaps between them
            min_cells_needed = sum(clue) + len(clue) - 1 if clue else 0
            if min_cells_needed > self.col:
                self.ready_to_solve = False
                return False

        # Validate column clues
        for i, clue in enumerate(self.col_clues):
            # Check for empty or invalid clues
            if not isinstance(clue, list):
                self.ready_to_solve = False
                return False

            # Check all values are positive integers
            if any(val <= 0 for val in clue):
                self.ready_to_solve = False
                return False

            # Check if clue can fit in the column
            min_cells_needed = sum(clue) + len(clue) - 1 if clue else 0
            if min_cells_needed > self.row:
                self.ready_to_solve = False
                return False
            
        col_clue_sums = sum(sum(clue) for clue in self.col_clues)
        row_clue_sums = sum(sum(clue) for clue in self.row_clues)
        if col_clue_sums != row_clue_sums:
            return False

        self.ready_to_solve = True
        return True

    @staticmethod
    def _generate_one_possibility(clue: list[int], partition: list[int]) -> list[int]:
        parts = list(
            chain.from_iterable(
                chain([0] * partition[i], [1] * c) for i, c in enumerate(clue)
            )
        )
        parts.extend([0] * partition[-1])
        return parts[1:-1]

    def _generate_possibilities_for_clue(self,
        clue: list[int], length: int
    ) -> list[list[int]]:
        partitions = integer_partitions(
            length - sum(clue) + 2, len(clue) + 1
        )
        possibilities = [
            self._generate_one_possibility(clue, partition)
            for partition in partitions
        ]
        return possibilities

    def _generate_possibilities(self) -> None:
        for clue in self.row_clues:
            self.rows_possible.append(self._generate_possibilities_for_clue(clue, self.col))
        pass

    @staticmethod
    def _check_one_column(column_current: list[int], clue: list[int]) -> bool:
        ans: bool = True

        new_cc = [
            sum(1 for _ in grp) for val, grp in groupby(column_current) if val == 1
        ]
        len_new_cc: int = len(new_cc)
        len_clue: int = len(clue)
        for i, c in enumerate(new_cc):
            if i >= len_clue:
                logger.debug(
                    "no match for reason 1: runs=%s, column=%s, clue=%s",
                    new_cc,
                    column_current,
                    clue,
                )
                return False
            elif i == len_new_cc - 1 and c > clue[i]:
                logger.debug(
                    "no match for reason 2: runs=%s, column=%s, clue=%s",
                    new_cc,
                    column_current,
                    clue,
                )
                return False
            elif i < len_new_cc - 1 and c != clue[i]:
                logger.debug(
                    "no match for reason 3: runs=%s, column=%s, clue=%s",
                    new_cc,
                    column_current,
                    clue,
                )
                ans = False

        return True

    def _check_columns_match_clues(self) -> bool:
        for i, col in enumerate(self.cols_current):
            if not self._check_one_column(col, self.col_clues[i]):
                return False
        return True

    def _columns_step_ahead(self, row: list[int]) -> None:
        for i, val in enumerate(row):
            self.cols_current[i].append(val)

    def _columns_step_back(self) -> None:
        for i in range(self.col):
            self.cols_current[i].pop()

    def _recursive_solve(self, row_idx: int) -> bool:
        """Recursively solve the nonogram using backtracking.

        Args:
            row_idx: Current row index to process

        Returns:
            True if solution found, False otherwise
        """
        # Base case: all rows processed
        if self.solved:
            return False
        if row_idx >= self.row:
            # Check if all columns match their clues
            logger.debug("Reached base case")
            logger.debug("cols_current=%s", self.cols_current)
            flag = self._check_columns_match_clues()
            if flag:
                self.solved = True
                logger.info("Solved inside recursive")
                logger.debug("Final cols_current: %s", self.cols_current)
                logger.debug("Final rows_possible_index: %s", self.rows_possible_index)
                return self._check_columns_match_clues()
            else:
                return False

        # Try each possible configuration for the current row
        for i, possible_row in enumerate(self.rows_possible[row_idx]):

            # Update column states with this row
            self._columns_step_ahead(possible_row)
            self.rows_possible_index[row_idx] = i
            logger.debug("Trying row %s possibility %s: %s", row_idx, i, possible_row)
            logger.debug("Current cols_current: %s", self.cols_current)
            # Check if columns are still valid
            if self._check_columns_match_clues():
                # Recursively try next row
                if self._recursive_solve(row_idx + 1):
                    self.solved = True
                    return True

            # Backtrack: undo column changes
            else:
                self._columns_step_back()
        self._columns_step_back()
        return False

    def solve(self) -> None:
        """Solve the nonogram puzzle."""
        self.check_ready()
        if not self.ready_to_solve:
            raise RuntimeError("Solver is not ready. Please check clues and grid size.")

        # Generate all possible row configurations
        self._generate_possibilities()

        # Initialize column state tracking
        self.cols_current = [[0] for _ in range(self.col)]

        # Start recursive search from row 0
        self.solved = self._recursive_solve(0)

        if not self.solved:
            raise RuntimeError("No solution found for the given clues.")

    def get_solution(self) -> list[list[int]]:
        """Retrieve the solved nonogram grid.

        Returns:
            A 2D list representing the solved grid, where 1 indicates filled cells and 0 indicates empty cells.

        Raises:
            RuntimeError: If the puzzle has not been solved yet.
        """
        if not self.solved:
            raise RuntimeError("Puzzle not solved yet. Call solve() first.")

        solution = [
            self.rows_possible[i][self.rows_possible_index[i]] for i in range(self.row)
        ]
        return solution

    def print_solution(
        self,
        as_chars: bool = True,
        cell_width: int = 3,
        show_row_clues: bool = True,
        show_col_clues: bool = True,
        show_row_numbers: bool = True,
    ) -> None:
        """Print the solved nonogram to stdout in a nicely formatted way.

        Shows column clues vertically above the grid, row clues to the left,
        optional row numbers, and ensures equal column width.

        Args:
            as_chars: If True, prints using `#` for filled and `.` for empty.
                      If False, prints numeric 1/0.
            cell_width: width in characters for each cell (default 3).
            show_row_clues: whether to print row clues at left.
            show_col_clues: whether to print column clues above.
            show_row_numbers: whether to print row numbers at left.
        """
        if not self.solved:
            raise RuntimeError("Puzzle not solved yet. Call solve() first.")

        solution = self.get_solution()

        # Prepare row clues and column clues
        row_clues = self.row_clues or [[] for _ in range(self.row)]
        col_clues = self.col_clues or [[] for _ in range(self.col)]

        # Compute widths for left area
        rownum_width = len(str(self.row)) if show_row_numbers else 0
        row_clue_texts = [" ".join(map(str, rc)) for rc in row_clues]
        row_clue_width = (
            max((len(t) for t in row_clue_texts), default=0) if show_row_clues else 0
        )
        left_pad = 0
        if show_row_numbers:
            left_pad += rownum_width + 1  # number + space
        if show_row_clues:
            left_pad += row_clue_width + 1  # clues + space

        # Prepare column clues matrix (top to bottom)
        max_col_clue_h = max((len(c) for c in col_clues), default=0)
        col_clue_matrix = []
        for c in col_clues:
            padded = [""] * (max_col_clue_h - len(c)) + [str(x) for x in c]
            col_clue_matrix.append(padded)

        def format_cell(v: int) -> str:
            if as_chars:
                ch = "#" if v == 1 else "."
            else:
                ch = str(v)
            # center in cell_width
            return ch.center(cell_width)

        # Print column clues
        if show_col_clues and max_col_clue_h > 0:
            for r in range(max_col_clue_h):
                line = " " * left_pad
                parts = []
                for j in range(self.col):
                    val = col_clue_matrix[j][r]
                    parts.append(val.center(cell_width))
                line += "".join(parts)
                print(line)

        # Print grid with row numbers and row clues
        for idx, row in enumerate(solution, start=1):
            parts = []
            for v in row:
                parts.append(format_cell(v))

            left = ""
            if show_row_numbers:
                left += str(idx).rjust(rownum_width) + " "
            if show_row_clues:
                left += row_clue_texts[idx - 1].rjust(row_clue_width) + " "

            print(left + "".join(parts))
