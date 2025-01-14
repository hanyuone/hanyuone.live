# Changelog

## v0.3.1 (2025-01-11)

Refactor the MD parser, internally adding a `Container` enum that encapsulates behaviour of
more complex MD elements which can have children.

## v0.3.0 (2025-01-02)

Add support for **raw HTML** in the MD parser.

## v0.2.0 (2025-01-01)

Implement **tables** in the MD parser. This includes:
- **Cell merging**, using `<` and `^` to merge with the cell to the left and above respectively
- **Parse errors** when non-rectangular shapes are created as a result of cell merging

## v0.1.0 (2024-12-11)

Initial commit.
