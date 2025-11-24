"""Mathematical utility functions for nonogram solver."""


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
