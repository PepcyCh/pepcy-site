use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    utils::{self, RawHtml},
    MainRoute,
};

#[derive(Clone, Routable, PartialEq)]
pub enum BlogRoute {
    #[at("/algo")]
    DefaultPage,
    #[at("/algo/:s")]
    Page { s: String },
    #[at("/algo/tags")]
    Tags,
    #[at("/algo/tags/:tag")]
    TagPage { tag: String },
    #[not_found]
    #[at("/algo/404")]
    NotFound,
}

pub fn blog_switch(route: &BlogRoute) -> Html {
    let inner = match route {
        BlogRoute::DefaultPage => html! { <BlogList page={1} /> },
        BlogRoute::Page { s } => {
            if let Ok(page) = u32::from_str_radix(&s, 10) {
                html! { <BlogList {page} /> }
            } else {
                html! { <BlogPage s={s.clone()} /> }
            }
        }
        BlogRoute::Tags => html! { <BlogTags /> },
        BlogRoute::TagPage { tag } => html! { <BlogTagPage tag={tag.clone()} /> },
        BlogRoute::NotFound => html! { <Redirect<MainRoute> to={MainRoute::NotFound} /> },
    };

    let nav_burger_onclick = Callback::from(|_| {
        let doc = web_sys::window().unwrap().document().unwrap();
        if let Some(nav) = doc.get_element_by_id("blogsNavbar") {
            let burger = doc.get_element_by_id("blogsNavbarBurger").unwrap();
            let nav_list = nav.class_list();
            let burger_list = burger.class_list();
            if nav_list.contains("is-active") {
                nav_list.remove_1("is-active").unwrap();
                burger_list.remove_1("is-active").unwrap();
            } else {
                nav_list.add_1("is-active").unwrap();
                burger_list.add_1("is-active").unwrap();
            }
        }
    });

    html! {
        <>
        <nav class="navbar is-info" role="navigation">
            <div class="navbar-brand">
                <div class="navbar-item">
                    <Link<MainRoute> to={MainRoute::Home}>
                        <strong>{ "Pepcy" }</strong>
                    </Link<MainRoute>>
                    <span>{ " | 博客-OI/*CPC" }</span>
                </div>
                <a role="button" class="navbar-burger" data-target="blogsNavbar"
                    id="blogsNavbarBurger" onclick={nav_burger_onclick}>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>
            <div id="blogsNavbar" class="navbar-menu">
                <div class="navbar-start">
                    <a href="/algo" class="navbar-item">
                        <span class="material-icons">{ "home" }</span>
                        <span>{ "首页" }</span>
                    </a>
                    <Link<BlogRoute> to={BlogRoute::Tags} classes="navbar-item">
                        <span class="material-icons">{ "sell" }</span>
                        <span>{ "标签" }</span>
                    </Link<BlogRoute>>
                </div>
                <div class="navbar-end">
                </div>
            </div>
        </nav>
        <section class="section">
        <div class="container">
        { inner }
        </div>
        </section>
        </>
    }
}

#[derive(Debug, serde::Deserialize)]
struct Blog {
    title: String,
    tags: Vec<String>,
    create_time: i64,
    last_modified: i64,
    toc: String,
    html: String,
}

pub enum Msg {
    Response(String),
}

#[derive(PartialEq, Properties)]
pub struct BlogPageProp {
    pub s: String,
}

pub struct BlogPage {
    text: String,
    fetching: bool,
}

impl Component for BlogPage {
    type Message = Msg;
    type Properties = BlogPageProp;

    fn create(ctx: &Context<Self>) -> Self {
        let s = ctx.props().s.clone();

        ctx.link().send_future(async move {
            let text =
                match reqwasm::http::Request::get(format!("/generated/algo/{}.json", s).as_str())
                    .send()
                    .await
                {
                    Ok(res) => res.text().await.unwrap(),
                    Err(err) => err.to_string(),
                };
            Msg::Response(text)
        });

        Self {
            text: "".to_owned(),
            fetching: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(text) => {
                self.text = text;
                self.fetching = false;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.fetching {
            return html! {};
        } else if self.text.is_empty() {
            return html! {
                <Redirect<MainRoute> to={MainRoute::NotFound} />
            };
        }

        let blog: Blog = serde_json::from_str(&self.text).unwrap();
        html! {
            <div class="block">
            <div class="columns">
                <div class="column is-three-quarters">
                    <div class="content box">
                        <h1>{ blog.title }</h1>
                        { utils::created_and_modified_time(blog.create_time, blog.last_modified) }
                        <div class="tags">
                        {
                            for blog.tags
                                .iter()
                                .map(|tag| {
                                    let tag_url = utils::percent_encode(tag);
                                    html! {
                                        <a href={format!("/algo/tags/{}", tag_url)} class="tag is-success is-light">
                                        { tag }
                                        </a>
                                    }
                                })
                        }
                        </div>
                        <RawHtml html_str={blog.html} />
                    </div>
                </div>
                <div class="column">
                {
                    if blog.toc.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <div class="box toc">
                                <div class="block"><h2 class="subtitle is-4">
                                { "目录" }
                                </h2></div>
                                <RawHtml html_str={blog.toc} />
                            </div>
                        }
                    }
                }
                </div>
            </div>
            </div>
        }
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
struct BlogHeader {
    title: String,
    url: String,
    tags: Vec<String>,
    part: String,
    create_time: i64,
    last_modified: i64,
}

#[derive(PartialEq, Properties)]
pub struct BlogListProp {
    pub page: u32,
}

pub struct BlogList {
    page: u32,
    text: String,
    fetching: bool,
}

impl Component for BlogList {
    type Message = Msg;
    type Properties = BlogListProp;

    fn create(ctx: &Context<Self>) -> Self {
        let page = ctx.props().page;

        ctx.link().send_future(async move {
            let text = match reqwasm::http::Request::get(
                format!("/generated/algo/_page{}.json", page).as_str(),
            )
            .send()
            .await
            {
                Ok(res) => res.text().await.unwrap(),
                Err(err) => err.to_string(),
            };
            Msg::Response(text)
        });

        Self {
            page,
            text: "".to_owned(),
            fetching: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(text) => {
                self.text = text;
                self.fetching = false;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.fetching {
            return html! {};
        } else if self.text.is_empty() {
            return html! {
                <Redirect<MainRoute> to={MainRoute::NotFound} />
            };
        }

        let articles: serde_json::Value = serde_json::from_str(&self.text).unwrap();
        let total_page_count = articles.get("page_count").unwrap().as_i64().unwrap();
        let articles = articles.get("articles").unwrap().as_array().unwrap();

        html! {
            <>
            <div class="block">
            {
                for articles
                    .iter()
                    .map(|article| {
                        let article: BlogHeader = serde_json::from_value(article.clone()).unwrap();
                        html!{
                            <div class="box"><div class="content">
                                <h2><a href={format!("/algo/{}", article.url)}>{
                                    article.title
                                }</a></h2>
                                {
                                    utils::created_and_modified_time(
                                        article.create_time,
                                        article.last_modified,
                                    )
                                }
                                <div class="tags">
                                {
                                    for article.tags
                                        .iter()
                                        .map(|tag| {
                                            let tag_url = utils::percent_encode(tag);
                                            html! {
                                                <a href={format!("/algo/tags/{}", tag_url)} class="tag is-success is-light">
                                                { tag }
                                                </a>
                                            }
                                        })
                                }
                                </div>
                                <span>
                                    <RawHtml html_str={ article.part } />
                                </span>
                            </div></div>
                        }
                    })
            }
            </div>
            <div class="block">
            { utils::pagination_nav(self.page, total_page_count as u32, "/algo") }
            </div>
            </>
        }
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
struct BlogTag {
    name: String,
    count: u32,
}

pub struct BlogTags {
    text: String,
    fetching: bool,
}

impl Component for BlogTags {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let text = match reqwasm::http::Request::get("/generated/algo/_tag.json")
            .send()
            .await
            {
                Ok(res) => res.text().await.unwrap(),
                Err(err) => err.to_string(),
            };
            Msg::Response(text)
        });

        Self {
            text: "".to_owned(),
            fetching: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(text) => {
                self.text = text;
                self.fetching = false;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.fetching {
            return html! {};
        } else if self.text.is_empty() {
            return html! {
                <Redirect<MainRoute> to={MainRoute::NotFound} />
            };
        }

        let tags: serde_json::Value = serde_json::from_str(&self.text).unwrap();
        let tags = tags.as_array().unwrap();

        html! {
            <div class="block">
            {
                for tags
                    .iter()
                    .map(|tag| {
                        let tag: BlogTag = serde_json::from_value(tag.clone()).unwrap();
                        let tag_url = utils::percent_encode(&tag.name);
                        html!{
                            <span class="card p-3 m-1" style="line-height: 4rem">
                                <a href={format!("/algo/tags/{}", tag_url)}>
                                { format!("{} ({})", tag.name, tag.count) }
                                </a>
                            </span>
                        }
                    })
            }
            </div>
        }
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
struct BlogTagArticle {
    title: String,
    url: String,
}

#[derive(PartialEq, Properties)]
pub struct BlogTagPageProp {
    pub tag: String,
}

pub struct BlogTagPage {
    tag: String,
    text: String,
    fetching: bool,
}

impl Component for BlogTagPage {
    type Message = Msg;
    type Properties = BlogTagPageProp;

    fn create(ctx: &Context<Self>) -> Self {
        let tag_url = ctx.props().tag.clone();
        let tag = utils::percent_decode(&tag_url);
        let tag_file = tag.clone();

        ctx.link().send_future(async move {
            let text = match reqwasm::http::Request::get(
                format!("/generated/algo/_tag_{}.json", tag_file).as_str()
            )
            .send()
            .await
            {
                Ok(res) => res.text().await.unwrap(),
                Err(err) => err.to_string(),
            };
            Msg::Response(text)
        });

        Self {
            tag,
            text: "".to_owned(),
            fetching: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(text) => {
                self.text = text;
                self.fetching = false;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.fetching {
            return html! {};
        } else if self.text.is_empty() {
            return html! {
                <Redirect<MainRoute> to={MainRoute::NotFound} />
            };
        }

        let articles: serde_json::Value = serde_json::from_str(&self.text).unwrap();
        let articles = articles.as_array().unwrap();

        html! {
            <div class="block">
            <h2 class="title is-3">{ format!("标签：{}", self.tag) }</h2>
            {
                for articles
                    .iter()
                    .map(|article| {
                        let article: BlogTagArticle =
                            serde_json::from_value(article.clone()).unwrap();
                        html!{
                            <div class="box">
                                <a href={format!("/algo/{}", article.url)}>
                                    <strong>{ article.title }</strong>
                                </a>
                            </div>
                        }
                    })
            }
            </div>
        }
    }
}