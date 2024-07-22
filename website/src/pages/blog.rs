use yew::{function_component, html, html_nested, use_context, Html};

use crate::{
    components::{
        blog::{card::BlogCard, item::BlogItem},
        head::Head,
    },
    context::BlogContext,
};

#[function_component(Page)]
pub fn page() -> Html {
    let blog_context = use_context::<BlogContext>().unwrap();

    let mut blogs = blog_context.content.into_iter().collect::<Vec<_>>();
    blogs.sort_by(|(_, a), (_, b)| {
        b.front_matter
            .publish_date
            .cmp(&a.front_matter.publish_date)
    });

    let mut blogs = blogs.into_iter();
    let first_blog = blogs.next();

    if let Some((first_id, first_metadata)) = first_blog {
        return html! {
            <>
                <Head>
                    <title>{"Blog | Hanyuan's Website"}</title>
                </Head>
                <BlogCard
                    id={first_id}
                    metadata={first_metadata} />
                <div>
                    {
                        blogs
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
        };
    }

    html! {
        <>
            <Head>
                <title>{"Blog | Hanyuan's Website"}</title>
            </Head>
            <p>{"No blogs found!"}</p>
        </>
    }
}
