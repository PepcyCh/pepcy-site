#![recursion_limit = "512"]
#[rustfmt::skip::macros(html)]

use yew::prelude::*;
use yew_router::prelude::*;

mod utils;

mod home;

mod algo;
mod note;
mod acgn;

mod about;
mod resume;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/algo")]
    AlgoDefault,
    #[at("/algo/:s")]
    Algo,
    #[at("/algo/tags/:s")]
    AlgoTags,
    #[at("/note")]
    NoteDefault,
    #[at("/note/:s")]
    Note,
    #[at("/note/tags/:s")]
    NoteTags,
    #[at("/acgn")]
    AcgnDefault,
    #[at("/acgn/:s")]
    Acgn,
    #[at("/acgn/tags/:s")]
    AcgnTags,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn main_switch(routes: &MainRoute) -> Html {
    match routes {
        MainRoute::Home => html! { <home::Home /> },
        MainRoute::NotFound => html! { <home::NotFound /> },
        MainRoute::AlgoDefault | MainRoute::Algo | MainRoute::AlgoTags => html! {
            <Switch<algo::BlogRoute> render={Switch::render(algo::blog_switch)} />
        },
        MainRoute::NoteDefault | MainRoute::Note | MainRoute::NoteTags => html! {
            <Switch<note::BlogRoute> render={Switch::render(note::blog_switch)} />
        },
        MainRoute::AcgnDefault | MainRoute::Acgn | MainRoute::AcgnTags => html! {
            <Switch<acgn::BlogRoute> render={Switch::render(acgn::blog_switch)} />
        },
        MainRoute::About => html! { <about::About /> },
        MainRoute::Resume => html! { <resume::Resume /> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<MainRoute> render={Switch::render(main_switch)} />
            </BrowserRouter>
            <footer class="footer">
                <div class="content has-text-centered">
                    <p>{
                        "Copyright © 2017~2022 - Pepcy"
                    }</p>
                </div>
            </footer>
        </>
    }
}

fn main() {
    yew::start_app::<Main>();
}