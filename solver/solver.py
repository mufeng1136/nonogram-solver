from itertools import chain
from utils.math_utils import integer_partitions

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

        new_cc = list(filter(lambda x: x != 0, column_current))
        len_new_cc: int = len(new_cc)
        len_clue: int = len(clue)
        for i, c in enumerate(new_cc):
            if i >= len_clue or c > clue[i]:
                return False
            elif i == len_new_cc - 1 and c < clue[i]:
                return False
            elif c != clue[i]:
                ans = False
            
        return True
    
    def _check_columns_match_clues(self) -> bool:
        for i, col in enumerate(self.cols_current):
            if not self._check_one_column(col, self.col_clues[i]):
                return False
        return True

    def _columns_step_ahead(self, row: list[int]) -> None:
        # Placeholder for stepping columns ahead logic
        pass

    def _columns_step_back(self, row: list[int]) -> None:
        # Placeholder for stepping columns back logic
        pass

    def _recursive_solve(self, row_idx: int) -> bool:
        """Recursively solve the nonogram using backtracking.

        Args:
            row_idx: Current row index to process

        Returns:
            True if solution found, False otherwise
        """
        # Base case: all rows processed
        if row_idx >= self.row:
            # Check if all columns match their clues
            return self._check_columns_match_clues()

        # Try each possible configuration for the current row
        for i, possible_row in self.rows_possible[row_idx]:
            # Update column states with this row
            self._columns_step_ahead(possible_row)
            self.rows_possible_index[row_idx] = i
            # Check if columns are still valid
            if self._check_columns_match_clues():
                # Recursively try next row
                if self._recursive_solve(row_idx + 1):
                    return True

            # Backtrack: undo column changes
            self._columns_step_back(possible_row)

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
