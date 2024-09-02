use enum_iterator;
use macros::generate_tag_id;

pub struct TagMetadata {
    pub colour: String,
}

generate_tag_id!("public/tags.yaml");
