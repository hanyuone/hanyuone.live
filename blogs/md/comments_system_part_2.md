---
title: Comments system devlog, Part 2
description: Encountering our first roadblock - moving to a new library.
image: /blog_media/comments_system/leptos.png
publish_date: 2026-02-15 23:13
tags:
- meta
- programming
---

First off, I wanted part 2 of this series to be me implementing the most basic MVP on the
client side - something that actually pushes progress for the project forward. Linking a button
with the GitHub logo to the authentication system, and having mock comments on blogs that I
could actually see, something like that. And I did make some progress on that front, on a
pull request, but... Yew got in the way.

### Rust web frameworks

The way `hanyuone.live` is structured is mostly based on [this blog post](https://blakerain.com/blog/ssg-and-hydration-with-yew/) -
I wanted to make a website entirely in Rust, with a statically-generated blog system, and someone
had already done the hard design work for me. Inside the blog you can find insights into Yew
internals, hydration and Markdown parsing/regeneration on the client, all done in an efficient way.
I highly recommend giving it a read.

The author decided to use [Yew](https://yew.rs) as their main web framework - it is essentially
a Rust version of React, with the same HTML-in-code magic done through a macro, `html!`. It's great
for single-page applications (SPAs), all the rage a few years ago, and has... some support for **static site
generation** (SSG), where HTML pages are generated in advance, very useful for SEO. This limited support
for anything outside of client-side rendering (CSR, what is used for SPAs) will come up later.

From there, the website structure is as follows:
- Locally, the website is built as a SPA, using Yew's default CSR. Nothing crazy here.
- However, when *building* the website to be deployed, Yew's server-side rendering (SSR) features are used,
  plus SSG and another feature called **hydration**, where client-side WASM "injects" reactivity into what
  are normally static HTML pages.

These two modes are very, very different, and there were headaches when trying to set up a build system
that could accommodate both (some work was done by investigating the blog and GitHub repos, other work
had to be done completely from scratch by yours truly). But the system worked, blog Markdown files were converted
into webpages (or rather, converted into a condensed form that was trivial to convert into HTML, to speed
up website load times), and the CSR and SSR versions of the site functioned identically.

That is, until I tried to implement a client for the comments system.

### Yew, `Suspense` and OAuth

From [Part 1](/blog/comments_system_part_1) of the devlog, the comments system uses the OAuth 2 flow to allow
users to log in through larger tech platforms like GitHub or Google (and therefore, offload most of our
security concerns to these large platforms). Accessing information using a GitHub account and OAuth 2 looks
something like this:

![A diagram of the OAuth 2 flow.](/blog_media/comments_system/oauth2.png)

The comments system currently has a few endpoints related to authentication[^1]:
- `/auth/login`, which redirects the user to `github.com` to verify their account, and returns a GH access token
- `/auth/session`, which generates a local session token from the GH access token, storing that user's username
  and PFP on a local database.
- `/profile`, which first goes through some authentication middleware that converts a session token into
  a corresponding user ID, and returns that user's information.

For an unregistered user, when we access a blog post, the website does the following:
1. It first attempts to fetch a user's profile. If the middleware fails (e.g. we don't have a token, our token
   expired), a button to login via GitHub is displayed.
2. The user clicks the button, and `/auth/login` is requested.
3. The user logs in via GitHub, and is redirected to the `/auth/session` endpoint.
4. A session ID gets generated, and the user is redirected again, back to that blog post.
5. The website fetches the user's profile again, and because we now have a valid session ID, the user's details
   are fetched, and they can now write comments.

Phew! That was a lot of low-level auth to go through, and now we have enough context to see why Yew was a problem.
In order to check whether a user is logged in or not (and thus, either display a login button or a list of comments),
we need to send a request to the comments system, specifically to `/profile`. We don't want the page to hang while
that request is being made, however - this is where the [`Suspense`](https://yew.rs/docs/concepts/suspense) component
comes in handy.

How it works is, `Suspense` renders some HTML element `fallback` until whatever is inside `<Suspense />` has finished
processing. We indicate whether we finish processing in the background using `SuspensionResult`, like in the example
code below:

```rust
#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // If a user is loaded, then we return it as Ok(user).
        Some(m) => Ok(m),
        None => {
            // When user is still loading, then we create a `Suspension`
            // and call `SuspensionHandle::resume` when data loading
            // completes, the component will be re-rendered
            // automatically.
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}

#[component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
```

As you can see, `Suspense` works wonders for very basic use-cases, like above, but it breaks apart for anything
remotely more complicated. The main problem was `Suspense` and the Rust/WASM difficulties mentioned in Part 1,
specifically around `reqwest` and async. Because Yew hadn't had an update in over 2 years at the time of
implementation[^2], some libraries around that area were discontinued and had become buggy, resulting in
unsolvable bugs. Couple that with little to no documentation around `Suspense` itself, and it quickly became
impossible to make progress.

After a week or two of no progress, it was time to switch libraries.

### In comes Leptos

[Leptos](https://www.leptos.dev/) is a newer library modelled after [SolidJS](https://www.solidjs.com/) and
[Svelte](https://svelte.dev/), which also has reactivity (through *signals*) but foregoes a virtual DOM. After
some initial research, it appeared that Leptos was a good fit for refactoring - it uses very similar JSX-like
syntax, and was very actively developed.

As I moved the website over to Leptos, I also took the chance to change the `website` crate up somewhat:
- SSR in Leptos is more supported, with its model of `server` and `component` functions being easier to work with.
  The original build system, of local development being done in CSR and deployments in SSR, has now been switched to
  both running SSR, with local builds running a minimal Axum server and deployments using SSG.
- Leptos has a feature called [*islands*](https://book.leptos.dev/islands.html), where sites are static by default
  with small areas that are reactive. I've now moved `website` to using islands, which greatly reduces the size of
  my built HTML files without sacrificing functionality.

For now I'm really happy with Leptos, and from the next devlog on I can, hopefully, start actually doing dev work.

[^1]: *Authentication* and *authorisation* are two similar-sounding, but different concepts, and our system does
a bit of both. OAuth 2 is used for *authorisation* - given a user has already logged in, OAuth2 provides that user
with a token that allows them to perform certain actions. The actions I actually care about in our system is
accessing a user's username and profile picture, since I'm really just using GitHub's OAuth as a lazy substitute
for account management. I use GitHub's OAuth for *authentication* - verifying a user's identity through linking it
to a GitHub account - but on the GitHub side, I create an app that authorises the comments system to fetch username
and PFP information.

[^2]: As of December 8, 2025, Yew [released v0.22.0](https://github.com/yewstack/yew/releases/tag/yew-v0.22.0), which
I believe fixes [a huge problem](https://github.com/yewstack/yew/pull/3776) that I had when implementing the frontend.
