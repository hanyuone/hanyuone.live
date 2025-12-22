---
title: Comments system devlog, Part 1
description: And the trials and tribulations I had to go through to get it up and running.
image: /public/blog_media/hello_world/title.png
publish_date: 2025-09-18 22:51
tags:
- meta
- programming
---

This website has been my passion project, on and off, for the past year or so. Most of the effort
has been spent on the blog, and most of the effort of *that* is on the custom Markdown parser (which
actually supports more features, at this point, than GitHub-flavoured Markdown). It's become a nice
and functional place to put my writing in, but there is still work to be done before it reaches version
1.0, a goal of mine for 2025.

Ultimately, I want this website to be my little corner of the internet. And in this corner, I want a place
for friends (and eventually strangers) to leave their thoughts on... my thoughts on my blog.

Thus, I need to make a comments system. And this series of devlogs will serve as a journal on my
implementation journey.

### Picking a service

The first thought in my head, which goes against everything a software developer stands for, was to not
reinvent the wheel. To start with, to make a comments system, I need a database to store all the comments in,
an accounts system, an authorisation system for those accounts, a spam filter, notifications... and wow,
that's a *lot*. Many, *many* people looked into this before me - why not piggyback off the effort of some
poor soul that already had to deal with these headaches?

Getting costs to an absolute minimum were a top priority, as I am but a broke university student. Thus,
anything proprietary was off the table, and any external hosting options were too. I managed to find some
great open-source tools that look easy to set up - all of the ones below simply work(TM) with Docker:

- [Discourse](https://github.com/discourse/discourse)
- [Commento](https://github.com/adtac/commento)
- [Isso](https://github.com/isso-comments/isso)

My website itself is hosted on Cloudflare, and so I thought it would be easy to just plop a Docker container
on CF and call it a day. Cloudflare itself has a product, known as Containers, which allows me to do exactly
that. Let's take a deeper look into how Containers works, and...

![Wait... what's that bit highlighted in blue?](/public/blog_media/comments_system/containers.png)

![Oh...](/public/blog_media/comments_system/workers_paid.png)

Change of plans - we have to roll our own comments system. Time to reinvent the wheel.

### Rolling our own service

The great thing is, it seems that Cloudflare was one step ahead of us, and has provided us with two
tutorials - one for [building a comments API](https://developers.cloudflare.com/d1/tutorials/build-a-comments-api/)
and one for [writing Workers in Rust](https://developers.cloudflare.com/workers/languages/rust/). Let's
go ahead and start work on making a comments service in Rust!

The MVP for such a service, and thus the first thing I'm working towards, would allow users to login in some way
and add comments. All the extra features can come later. To do that, I need to first create an accounts system,
but I want it to be secure. I don't know much about security, so there is a high risk of something messing up -
passwords not being encrypted enough, databases getting breached. The same advice for doing the entire comments
system applies here too - why worry about this myself when I can use what other people have already made?

Luckily for me, there is already an industry standard, [OAuth](https://oauth.net/2/) - by following the user flows
that OAuth says to implement, I can just get users to "sign in via GitHub" or Google, because they provide OAuth
integration, and have those platforms handle security for me. Those platforms have millions, or billions, of users,
and have entire teams dedicated to making sure their databases are secure, their passwords never get deciphered.
Furthermore, if I implement the OAuth flow for one platform (GitHub), then it can be pretty easily extended to
other big platforms too (Google, Meta...), since they all follow the standard.

The implementation started easily enough - I followed both tutorials, I set up some API testing using
[Insomnia](https://insomnia.rest/), I found a Rust crate that handles all the OAuth stuff for me,
[`oauth2-rs`](https://github.com/ramosbugs/oauth2-rs). The package even supports popular Rust HTTP clients, like
`reqwest` and `ureq`! So I started following the example project in `oauth2-rs`, used `reqwest` for my client,
and...

### Rust and WASM

[WebAssembly](https://webassembly.org/), or WASM, is a very new technology, designed to eventually allow
programming languages other than JavaScript to work on browsers. It's the reason why I can write my entire
website in Rust to begin with, and thank goodness for that, because I'm having a blast not spending my time
on JavaScript or TypeScript.

However, this means that the Rust/WASM ecosystem is very new, and thus subject to a lot of changes and
unimplemented/broken things. Take for example the `rustwasm` organisation, which worked officially within
the Rust organisation and maintained a lot of fundamental tools like `wasm-bindgen` and `gloo`. This group
was officially sunset [in July](https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/),
and the tools under its purview were given to new owners.

And it's not necessarily the fault of the people on the Rust side either, who are doing the lord's work.
The surrounding ecosystem is extremely new (read: unstable) as well - the first big issue I ran into was
trying to get `reqwest` working with the OAuth flow. CF workers are required to use WASM, but `oauth2-rs`
and `reqwest` were not initially built for WASM, and thus some functionality is not truly cross-platform.

With GitHub OAuth specifically, I have to get the user to redirect to GitHub's login page, which then
returns a token that allows us to have permission to access that user's profile information (usernames,
profile photos). But the OAuth call requires *manual* redirecting to prevent vulnerabilities such as
[server-side request forgery](https://portswigger.net/web-security/ssrf) - this is unsupported in `reqwest`
for WASM, because the `fetch` API (which browsers use to make HTTP requests) is inconsistent (see [this PR](https://github.com/seanmonstar/reqwest/pull/2119)
for more details). Unfortunately, this is something that is completely outside `reqwest`'s control, and
is obviously not their fault.

Thus, `reqwest` is unusable, and a similar tale of the "wild west" of Rust/WASM applied to `ureq` (the newest
version is simply unsupported by `oauth2-rs`). The problem eventually did end up being solved - I used
the request API provided by CloudFlare instead - but it was a solid two weeks of digging through GitHub
issues and low-level source code that I can't get back.

### Takeaways

In 2025, the Rust and WASM ecosystem is still being actively worked on, which means that creating whole
services and websites on WASM is not exactly the smoothest experience in the world, *yet*. The keyword is
"yet" - the open source community is always amazing, and I'm still able to write Rust code seamlessly for both
my machine and the browser.

If you care about the very niche issue of having OAuth work properly for WASM, you can track (or help fix!)
some of the following issues/PRs:
- [The redirects issue for WASM `reqwest`](https://github.com/seanmonstar/reqwest/issues/2071)
- [`oauth2-rs` support for `ureq` v0.3](https://github.com/ramosbugs/oauth2-rs/pull/324)

Join me in the next devlog, where I'll focus on integrating the comments system into `hanyuone.live` itself!
