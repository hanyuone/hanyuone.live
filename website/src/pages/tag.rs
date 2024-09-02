use markdown::structs::tag::{TagId, TagMetadata};
use yew::{function_component, html, html_nested, use_context, Html, Properties};

use crate::{components::{blog::{card::BlogCard, item::BlogItem, tag::Tag}, head::Head}, context::BlogContext};

#[derive(PartialEq, Properties)]
pub struct TagPageProps {
    pub tag_id: TagId,
}

#[function_component(Page)]
pub fn page(props: &TagPageProps) -> Html {
    let blogs = use_context::<BlogContext>().unwrap().content;
    let tag_id = props.tag_id.clone();
    let tag_metadata: TagMetadata = tag_id.clone().into();

    let mut tag_blogs = blogs
        .into_iter()
        .filter(|(_, metadata)| {
            metadata
                .front_matter
                .tags
                .contains(&tag_id.to_string())
        });
    let first_blog = tag_blogs.next();

    html! {
        <>
            <Head>
                <title>{format!("{} | Hanyuan's Website", tag_id.to_string())}</title>
            </Head>
            <div>
                <Tag
                    name={tag_id.to_string()}
                    colour={tag_metadata.colour} />
            </div>
            {
                if let Some((first_id, first_metadata)) = first_blog {
                    html! {
                        <>
                            <Head>
                                <title>{"Blog | Hanyuan's Website"}</title>
                            </Head>
                            <BlogCard
                                id={first_id}
                                metadata={first_metadata} />
                            <div>
                                {
                                    tag_blogs
                                        .map(|(id, metadata)| {
                                            html_nested! {
                                                <BlogItem
                                                    id={id}
                                                    metadata={metadata} />
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }
                            </div>
                        </>
                    }
                } else {
                    html! { <p>{"No blogs found!"}</p> }
                }
            }
        </>
    }
}
