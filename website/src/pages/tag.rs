use markdown::structs::tag::{TagId, TagMetadata};
use yew::{function_component, html, html_nested, use_context, Html, Properties};
use yew_router::prelude::Link;

use crate::{
    components::{
        blog::{card::BlogCard, item::BlogItem},
        head::Head,
    },
    context::BlogContext,
    pages::Route,
};

#[derive(PartialEq, Properties)]
pub struct TagPageProps {
    pub tag_id: TagId,
}

#[function_component(Page)]
pub fn page(props: &TagPageProps) -> Html {
    let blog_context = use_context::<BlogContext>().unwrap();
    let tag_id = props.tag_id.clone();
    let tag_metadata: TagMetadata = tag_id.clone().into();

    let sorted_blogs = blog_context.get_sorted();
    let mut tag_blogs = sorted_blogs
        .into_iter()
        .filter(|(_, metadata)| metadata.front_matter.tags.contains(&tag_id.to_string()));
    let first_blog = tag_blogs.next();

    html! {
        <>
            <Head>
                <title>{format!("{} | Hanyuan's Website", tag_id.to_string())}</title>
            </Head>
            <div class="flex flex-col">
                <Link<Route> to={Route::Tag { tag_id: tag_id.clone() }}>
                    <div class="flex">
                        <div class={format!("grow-0 rounded-sm px-2 transition bg-{0}/50 hover:bg-{0}", tag_metadata.colour)}>
                            <h2 class="font-bold text-2xl">{tag_id.to_string()}</h2>
                        </div>
                    </div>
                </Link<Route>>
                <div>{tag_metadata.description}</div>
            </div>
            {
                if let Some((first_id, first_metadata)) = first_blog {
                    html! {
                        <>
                            <Head>
                                <title>{"Blog | Hanyuan's Website"}</title>
                            </Head>
                            <BlogCard
                                id={*first_id}
                                metadata={first_metadata.clone()} />
                            <div>
                                {
                                    tag_blogs
                                        .map(|(id, metadata)| {
                                            html_nested! {
                                                <BlogItem
                                                    id={*id}
                                                    metadata={metadata.clone()} />
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
