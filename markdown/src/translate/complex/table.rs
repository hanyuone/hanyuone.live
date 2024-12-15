use pulldown_cmark::Alignment;

use crate::translate::{
    element::{AttributeName, ElementTag, RenderElement},
    node::RenderNode,
};

type Cell = Vec<RenderNode>;

pub struct Table {
    alignment: Vec<Alignment>,
    pub is_head: bool,
    head: Vec<Cell>,
    body: Vec<Vec<Cell>>,
}

impl Table {
    pub fn new(alignment: Vec<Alignment>) -> Self {
        Self {
            alignment,
            is_head: false,
            head: vec![],
            body: vec![],
        }
    }

    pub fn create_row(&mut self) {
        self.body.push(vec![]);
    }

    pub fn add_cell(&mut self, cell: Cell) {
        if self.is_head {
            self.head.push(cell);
        } else {
            self.body.last_mut().unwrap().push(cell);
        }
    }

    fn add_alignment(cell: &mut RenderElement, alignment: Alignment) {
        let align_str = match alignment {
            Alignment::None => None,
            Alignment::Left => Some("left"),
            Alignment::Center => Some("center"),
            Alignment::Right => Some("right"),
        };

        align_str
            .and_then(|value| Some(cell.add_attribute(AttributeName::Align, value.to_string())));
    }

    pub fn to_node(self) -> RenderNode {
        let mut table_element = RenderElement::new(ElementTag::Table);

        // Add head row items
        let mut table_head = RenderElement::new(ElementTag::Thead);
        let mut head_row = RenderElement::new(ElementTag::Tr);

        for (index, head_cell) in self.head.into_iter().enumerate() {
            let mut render_head_cell = RenderElement::new(ElementTag::Th);
            Self::add_alignment(&mut render_head_cell, self.alignment[index]);

            for node in head_cell {
                render_head_cell.add_child(node);
            }

            head_row.add_child(render_head_cell.into());
        }

        table_head.add_child(head_row.into());
        table_element.add_child(table_head.into());

        // Add body items
        let mut table_body = RenderElement::new(ElementTag::Tbody);

        for row in self.body {
            let mut table_row = RenderElement::new(ElementTag::Tr);

            for (index, cell) in row.into_iter().enumerate() {
                let mut render_cell = RenderElement::new(ElementTag::Td);
                Self::add_alignment(&mut render_cell, self.alignment[index]);

                for node in cell {
                    render_cell.add_child(node);
                }

                table_row.add_child(render_cell.into());
            }

            table_body.add_child(table_row.into());
        }

        table_element.add_child(table_body.into());

        table_element.into()
    }
}
