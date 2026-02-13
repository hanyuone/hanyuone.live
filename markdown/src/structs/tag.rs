use enum_iterator;
use macros::generate_tag_id;

pub struct TagMetadata {
    pub colour: String,
    pub description: String,
}

generate_tag_id!("blogs/tags.yaml");

impl TagId {
    pub fn get_all() -> Vec<TagId> {
        enum_iterator::all::<TagId>().collect()
    }
}
