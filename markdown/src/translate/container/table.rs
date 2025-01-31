use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

use pulldown_cmark::Alignment;

use crate::translate::{
    element::{AttributeName, ElementTag, RenderElement},
    error::TranslateError,
    node::RenderNode,
};

use super::Container;

/// The direction we want to merge the current cell towards.
enum MergeDirection {
    // Corresponds to "<"
    Left,
    // Corresponds to "^"
    Top,
}

impl MergeDirection {
    /// Detects whether a `&Vec<RenderNode>` (what the raw contents of a table is represented as)
    /// represents a cell that should be merged in a certain direction. If it's just
    /// a normal cell, then return a `None`.
    fn extract_from_nodes(content: &[RenderNode]) -> Option<MergeDirection> {
        if content.len() != 1 {
            return None;
        }

        match &content[0] {
            RenderNode::Text(text) => match text.as_str() {
                "<" => Some(Self::Left),
                "^" => Some(Self::Top),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Basic struct that represents xy-coordinates. Needs to be an `isize`
/// because we want to add positions with *negative* values together.
struct CellPosition(isize, isize);

impl Add<CellPosition> for CellPosition {
    type Output = Self;

    fn add(self, rhs: CellPosition) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy)]
/// Basic struct that represents the *dimensions* of a cell, used for
/// cell merging.
struct CellDimensions(usize, usize);

impl Add<CellDimensions> for CellDimensions {
    type Output = Self;

    fn add(self, rhs: CellDimensions) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<CellDimensions> for CellDimensions {
    fn add_assign(&mut self, rhs: CellDimensions) {
        *self = *self + rhs;
    }
}

#[derive(Debug)]
/// Cells in our `Chunk` helper struct can be two types - either
/// they contain our actual content, or they're "placeholders" that
/// tell the `RenderNode` generation function that this cell is
/// supposed to be merged. This struct represents both states.
enum Cell {
    Content(Vec<RenderNode>),
    Pointer(CellPosition),
}

struct Chunk {
    is_head: bool,
    cells: Vec<Vec<Cell>>,
    /// Hashmap of all cells with dimensions larger than 1x1.
    merged_sizes: HashMap<CellPosition, CellDimensions>,
}

impl Chunk {
    fn new(is_head: bool) -> Self {
        Self {
            is_head,
            cells: vec![],
            merged_sizes: HashMap::new(),
        }
    }

    // Chunk helper functions

    
    /// Returns the current position of the cell we're adding.
    /// Call when we're fetching cells from the parser.
    ///
    /// # Panics
    ///
    /// Panics if `self.cells` is empty, i.e. there are no rows, which
    /// should not be possible.
    fn current_position(&self) -> CellPosition {
        CellPosition(
            self.cells.len() as isize - 1,
            self.cells.last().unwrap().len() as isize,
        )
    }

    /// Gets the contents of a cell at a certain position.
    fn get(&self, position: CellPosition) -> &Cell {
        let row = &self.cells[position.0 as usize];
        &row[position.1 as usize]
    }

    // Adding rows/cells to chunk

    /// Creates a new, empty row in our chunk.
    fn add_row(&mut self) {
        self.cells.push(vec![]);
    }

    /// Given a "wrapped" cell, adds it to the last row. Only called after
    /// a row is initialised through `Chunk::add_row`.
    ///
    /// # Panics
    ///
    /// Panics if `self.cells` is empty (which should not be possible).
    fn add_cell(&mut self, cell: Cell) {
        self.cells.last_mut().unwrap().push(cell);
    }

    /// Given the contents of a table cell as a `Vec<RenderNode>` (which could
    /// include the special cells "<", "^", indicating merging), figure out
    /// whether we have a regular cell or a cell to be merged and add it to the
    /// chunk.
    ///
    /// # Errors
    ///
    /// This function will return an error if we have merging that results in
    /// non-rectangular cells.
    fn add_contents(&mut self, content: Vec<RenderNode>) -> Result<(), TranslateError> {
        let merge_dir = match MergeDirection::extract_from_nodes(&content) {
            Some(merge_dir) => merge_dir,
            None => {
                self.add_cell(Cell::Content(content));
                return Ok(());
            }
        };

        let curr_pos = self.current_position();

        let target_pos = curr_pos
            + match merge_dir {
                MergeDirection::Left => CellPosition(0, -1),
                MergeDirection::Top => CellPosition(-1, 0),
            };

        let dimension_base = match merge_dir {
            MergeDirection::Left => CellDimensions(0, 1),
            MergeDirection::Top => CellDimensions(1, 0),
        };

        let target_cell = self.get(target_pos);

        match target_cell {
            Cell::Content(_) => {
                self.merged_sizes
                    .entry(target_pos)
                    .and_modify(|e| *e += dimension_base)
                    .or_insert(CellDimensions(1, 1) + dimension_base);

                self.add_cell(Cell::Pointer(target_pos));
            }
            Cell::Pointer(origin) => {
                let origin = *origin;
                // Guaranteed to exist in `self.merged_sizes` since the pointer
                // at the target cell *must* point towards the top-left cell in the
                // block, which is added through the `Cell::Content` match arm
                let existing_size = self.merged_sizes.get(&origin).unwrap();

                let new_dimensions = match merge_dir {
                    MergeDirection::Left => {
                        if existing_size.0 == 1 {
                            Ok(*existing_size + dimension_base)
                        } else if curr_pos.1 > origin.1 + existing_size.1 as isize {
                            // We want to avoid the following grid layout:
                            // ```
                            // x <
                            // ^ < <
                            // ```
                            // where one left-merge "juts out". This should result in a merge error.
                            Err(TranslateError::TableMergeError)
                        } else {
                            println!("Here");
                            Ok(*existing_size)
                        }
                    }
                    MergeDirection::Top => {
                        if curr_pos.1 == origin.1 {
                            Ok(*existing_size + dimension_base)
                        } else if curr_pos.0 > origin.0 + existing_size.0 as isize {
                            // Similar to above, we want to avoid the following layout:
                            // ```
                            // x <
                            // ^ <
                            // . ^
                            // ```
                            // where one top-merge "juts out".
                            Err(TranslateError::TableMergeError)
                        } else {
                            Ok(*existing_size)
                        }
                    }
                }?;

                self.merged_sizes
                    .entry(origin)
                    .and_modify(|e| *e = new_dimensions);

                self.add_cell(Cell::Pointer(origin));
            }
        }

        Ok(())
    }

    // Generating output of chunk

    /// Given the way a table cell should be aligned, add HTML properties
    /// to `element` to display that properly in HTML.
    fn add_alignment(element: &mut RenderElement, alignment: Alignment) {
        let align_str = match alignment {
            Alignment::None => None,
            Alignment::Left => Some("left"),
            Alignment::Center => Some("center"),
            Alignment::Right => Some("right"),
        };

        if let Some(value) = align_str {
            element.add_attribute(AttributeName::Align, value.to_string());
        }
    }

    /// Converts the chunk into a `RenderNode`. Can either be a `Thead` (with `Th` children)
    /// or a `Tbody` (with `Td` children), depending on `self.is_head`.
    fn into_node(self, alignment: &[Alignment]) -> RenderNode {
        let mut outer = RenderElement::new(if self.is_head {
            ElementTag::Thead
        } else {
            ElementTag::Tbody
        });

        for (row_index, row) in self.cells.into_iter().enumerate() {
            let mut table_row = RenderElement::new(ElementTag::Tr);

            for (col_index, cell) in row.into_iter().enumerate() {
                let Cell::Content(content) = cell else {
                    continue;
                };

                let mut table_cell = RenderElement::new(if self.is_head {
                    ElementTag::Th
                } else {
                    ElementTag::Td
                });

                Self::add_alignment(&mut table_cell, alignment[col_index]);

                for node in content {
                    table_cell.add_child(node);
                }

                // Check whether our cell is larger than 1x1, in which case
                // add `rowspan` and `colspan` attributes
                let dimensions = self
                    .merged_sizes
                    .get(&CellPosition(row_index as isize, col_index as isize));

                if let Some(dimensions) = dimensions {
                    table_cell.add_attribute(AttributeName::Rowspan, dimensions.0.to_string());
                    table_cell.add_attribute(AttributeName::Colspan, dimensions.1.to_string());
                }

                table_row.add_child(table_cell.into());
            }

            outer.add_child(table_row.into());
        }

        outer.into()
    }
}

/// Simple wrapper around two `Chunk`s, one representing the head
/// and one the body.
pub struct Table {
    pub is_head: bool,
    alignment: Vec<Alignment>,
    head: Chunk,
    body: Chunk,
    cell: Vec<RenderNode>,
}

impl Table {
    pub fn new(alignment: Vec<Alignment>) -> Self {
        Self {
            is_head: false,
            alignment,
            head: Chunk::new(true),
            body: Chunk::new(false),
            cell: vec![],
        }
    }

    // Add content

    pub fn add_row(&mut self) {
        if self.is_head {
            self.head.add_row();
        } else {
            self.body.add_row();
        }
    }

    pub fn add_child(&mut self, child: RenderNode) {
        self.cell.push(child);
    }

    pub fn create_cell(&mut self) -> Result<(), TranslateError> {
        let contents = std::mem::take(&mut self.cell);

        if self.is_head {
            self.head.add_contents(contents)?;
        } else {
            self.body.add_contents(contents)?;
        }

        Ok(())
    }
}

impl From<Table> for RenderNode {
    fn from(value: Table) -> Self {
        let mut table_element = RenderElement::new(ElementTag::Table);

        let table_head = value.head.into_node(&value.alignment);
        table_element.add_child(table_head);

        let table_body = value.body.into_node(&value.alignment);
        table_element.add_child(table_body);

        table_element.into()
    }
}

impl From<Table> for Container<'_> {
    fn from(value: Table) -> Self {
        Self::Table(value)
    }
}
