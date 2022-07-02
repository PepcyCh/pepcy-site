use yew::prelude::*;
use yew_router::prelude::*;

use crate::{acgn, algo, note, utils::RawHtml, MainRoute};

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let algo_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(algo::BlogRoute::DefaultPage));
        html! {
            <button {onclick} class="button mr-3">{ "OI/*CPC" }</button>
        }
    };

    let note_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(note::BlogRoute::DefaultPage));
        html! {
            <button {onclick} class="button mr-3">{ "笔记" }</button>
        }
    };

    let acgn_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(acgn::BlogRoute::DefaultPage));
        html! {
            <button {onclick} class="button mr-3">{ "ACGN" }</button>
        }
    };

    let about_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(MainRoute::About));
        html! {
            <button {onclick} class="button mr-3">{ "关于我" }</button>
        }
    };

    let resume_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(MainRoute::Resume));
        html! {
            <button {onclick} class="button mr-3">{ "简历" }</button>
        }
    };

    html! {
        <section class="section">
            <div class="container is-max-desktop">
                <div class="block card">
                    <div class="card-content">
                        <div class="media">
                            <div class="media-left">
                                <figure class="image is-128x128">
                                    <img src="/figures/avatar.jpg" />
                                </figure>
                            </div>
                            <div class="media-content">
                                <div class="columns">
                                    <div class="column is-one-third">
                                        <span class="title is-1">{ "Pepcy_Ch" }</span>
                                    </div>
                                    <div class="column" style="position: relative">
                                        <div style="position: absolute; bottom: 0;">
                                        <span class="mx-2">
                                            <a href="https://github.com/PepcyCh">{ "Github" }</a>
                                        </span>
                                        <span class="mx-2">
                                            <a href="https://www.zhihu.com/people/pepcy-chen">{ "知乎" }</a>
                                        </span>
                                        <span class="mx-2">
                                            <a href="https://bgm.tv/user/551240">{ "bangumi" }</a>
                                        </span>
                                        </div>
                                    </div>
                                </div>
                                <hr />
                                <h2 class="subtitle is-5">{ "CG / ACGN / 前 OIer & *CPCer" }</h2>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="block message is-primary">
                    <div class="message-header">
                        <p>{ "博客" }</p>
                    </div>
                    <div class="message-body">
                        { algo_button }
                        { note_button }
                        { acgn_button }
                    </div>
                </div>
                <div class="block message is-info">
                    <div class="message-header">
                        <p>{ "关于" }</p>
                    </div>
                    <div class="message-body">
                        { about_button }
                        { resume_button }
                    </div>
                </div>
                <div class="block message">
                    <div class="message-header">
                        <p>{ "其他" }</p>
                    </div>
                    <div class="message-body">
                        <button class="button mr-3">
                            <a href="http://pepcy.top/icpc-templates">{ "*CPC 算法模板" }</a>
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let hint = include_str!("../../generated/404.html");

    html! {
        <section class="section">
            <div class="container is-max-desktop">
                <h1 class="title is-1">{ "404" }</h1>
                <div class="message is-danger">
                    <div class="message-header">
                        <p>{ "未能识别的 URL" }</p>
                    </div>
                    <div class="message-body">
                        <div class="content block">
                            <RawHtml html_str={hint} />
                        </div>
                        <Link<MainRoute> to={MainRoute::Home}>
                        { "回首页" }
                        </Link<MainRoute>>
                    </div>
                </div>
            </div>
        </section>
    }
}