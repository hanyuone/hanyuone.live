// [x] Render footnotes separately, always include at very bottom of file
// [x] Return button at end of footnote
// [ ] Hover tooltip for footnote

use std::collections::HashMap;

use pulldown_cmark::CowStr;

use crate::translate::{
    element::{AttributeName, ElementTag, RenderElement},
    node::RenderNode,
};

/// Helper class for storing footnotes, so we can *always* render them at the very end
/// of a Markdown file, regardless of where they were defined.
///
/// We render footnotes in a similar way to Obsidian and GitHub.
pub struct Footnotes<'a> {
    count: usize,
    indices: HashMap<CowStr<'a>, usize>,
    mapping: HashMap<CowStr<'a>, RenderElement>,
}

impl<'a> Footnotes<'a> {
    pub fn new() -> Self {
        Self {
            count: 1,
            indices: HashMap::new(),
            mapping: HashMap::new(),
        }
    }

    pub fn add_index(&mut self, name: CowStr<'a>) {
        self.indices.entry(name).or_insert(self.count);
        self.count += 1;
    }

    pub fn insert(&mut self, name: CowStr<'a>, element: RenderElement) {
        self.mapping.insert(name, element);
    }

    pub fn get_index(&self, name: CowStr<'a>) -> Option<usize> {
        self.indices.get(&name).copied()
    }

    fn footnote_to_node(
        (index, name, mut element): (&usize, CowStr<'a>, RenderElement),
    ) -> RenderNode {
        let mut footnote = RenderElement::new(ElementTag::Div);
        footnote.add_attribute(AttributeName::Id, format!("footnote_{name}"));

        let children = &mut element.children;

        // Add <p>{index}: </p> at beginning of each footnote
        let index_text = format!("{index}: ");

        // We know that the first child has to be a render element
        let RenderNode::Element(first_element) = children.first_mut().unwrap() else {
            unreachable!()
        };

        if first_element.tag == ElementTag::P {
            first_element
                .children
                .insert(0, RenderNode::Text(index_text));
        } else {
            let mut index_element = RenderElement::new(ElementTag::P);
            index_element.add_child(index_text.into());
            children.insert(0, index_element.into());
        }

        // Add return button at end of each footnote
        let mut return_button = RenderElement::new(ElementTag::A);
        return_button.add_attribute(AttributeName::Href, format!("#anchor_{name}"));
        return_button.add_child("↩️".to_string().into());

        // We know that the last child has to be a render element
        let RenderNode::Element(last_element) = children.last_mut().unwrap() else {
            unreachable!()
        };

        if last_element.tag == ElementTag::P {
            // Add space
            last_element.add_child(" ".to_string().into());
            last_element.add_child(return_button.into());
        } else {
            let mut return_element = RenderElement::new(ElementTag::P);
            return_element.add_child(return_button.into());
            element.add_child(return_element.into());
        }

        footnote.add_child(element.into());
        footnote.into()
    }

    pub fn to_nodes(self) -> Vec<RenderNode> {
        let cloned_indices = self.indices;
        let mut sorted_footnotes = self
            .mapping
            .into_iter()
            .filter_map(|(name, element)| {
                cloned_indices
                    .get(&name)
                    .map(|index| (index, name, element))
            })
            .collect::<Vec<_>>();

        sorted_footnotes.sort_by_key(|(index, _, _)| *index);

        sorted_footnotes
            .into_iter()
            .map(Footnotes::footnote_to_node)
            .collect::<Vec<_>>()
    }
}
