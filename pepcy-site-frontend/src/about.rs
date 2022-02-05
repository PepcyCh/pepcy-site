use yew::prelude::*;
use yew_router::prelude::*;

use crate::{utils::RawHtml, MainRoute};

pub enum Msg {
    Response(String),
}

pub struct About {
    text: String,
}

impl Component for About {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let text = match reqwasm::http::Request::get("/generated/about.html")
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(text) => self.text = text,
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <nav class="navbar is-info" role="navigation">
                <div class="navbar-brand">
                    <div class="block mx-4 my-auto">
                        <Link<MainRoute> to={MainRoute::Home}>
                            <strong>{ "Pepcy" }</strong>
                        </Link<MainRoute>>
                        <span>{ " | 关于" }</span>
                    </div>
                    <a role="button" class="navbar-burger" data-target="blogsNavbar">
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>
                <div id="blogsNavbar" class="navbar-menu">
                </div>
                <div class="navbar-end">
                </div>
            </nav>
            <section class="section">
                <div class="container is-max-desktop">
                    <div class="content box">
                        <RawHtml html_str={self.text.clone()} />
                    </div>
                </div>
            </section>
            </>
        }
    }
}
