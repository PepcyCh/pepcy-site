use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RawHtmlProps {
    pub html_str: String,
}

#[function_component(RawHtml)]
pub fn raw_html(props: &RawHtmlProps) -> Html {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&props.html_str);

    let node = web_sys::Node::from(div);
    let vnode = yew::virtual_dom::VNode::VRef(node);
    vnode
}

pub fn percent_encode(str: &str) -> String {
    percent_encoding::utf8_percent_encode(str, percent_encoding::NON_ALPHANUMERIC).to_string()
}

pub fn percent_decode(str: &str) -> String {
    let bytes_old = str.as_bytes();
    let len = bytes_old.len();
    let mut bytes = Vec::with_capacity(len);

    fn hex_to_dec(ch: u8) -> u8 {
        if ch >= 48 && ch < 58 {
            ch - 48
        } else if ch >= 65 && ch < 71 {
            ch - 55
        } else if ch >= 97 && ch < 103 {
            ch - 87
        } else {
            unreachable!()
        }
    }

    let mut i = 0;
    while i < len {
        let byte = if bytes_old[i] == 37 { // 37 - %
            let hi = hex_to_dec(bytes_old[i + 1]);
            let lo = hex_to_dec(bytes_old[i + 2]);
            i += 3;
            hi * 16 + lo
        } else {
            i += 1;
            bytes_old[i - 1]
        };

        bytes.push(byte);
    }

    String::from_utf8(bytes).unwrap()
}

pub fn pagination_nav(curr_page: u32, total_page: u32, url_prefix: &str) -> Html {
    let prev_url = format!("{}/{}", url_prefix, curr_page - 1);
    let prev = if curr_page == 1 {
        html! { <a class="pagination-previous is-disabled">{ "<" }</a> }
    } else {
        html! {
            <a class="pagination-previous" href={prev_url.clone()}>
                { "<" }
            </a>
        }
    };

    let next_url = format!("{}/{}", url_prefix, curr_page + 1);
    let next = if curr_page == total_page {
        html! { <a class="pagination-next is-disabled">{ ">" }</a> }
    } else {
        html! {
            <a class="pagination-next" href={next_url.clone()}>
                { ">" }
            </a>
        }
    };

    let left = if curr_page == 1 {
        html! {}
    } else if curr_page == 2 {
        html! {
            <li>
                <a class="pagination-link" href={format!("{}/1", url_prefix)}>{ 1 }</a>
            </li>
        }
    } else if curr_page == 3 {
        html! {
            <>
            <li>
                <a class="pagination-link" href={format!("{}/1", url_prefix)}>{ 1 }</a>
            </li>
            <li>
                <a class="pagination-link" href={format!("{}/2", url_prefix)}>{ 2 }</a>
            </li>
            </>
        }
    } else {
        html! {
            <>
            <li>
                <a class="pagination-link" href={format!("{}/1", url_prefix)}>{ 1 }</a>
            </li>
            <li>
                <span class="pagination-ellipsis">{ "..." }</span>
            </li>
            <li>
                <a class="pagination-link" href={prev_url}>{ curr_page - 1 }</a>
            </li>
            </>
        }
    };

    let last_url = format!("{}/{}", url_prefix, total_page);
    let right = if total_page - curr_page == 0 {
        html! {}
    } else if total_page - curr_page == 1 {
        html! {
            <li>
                <a class="pagination-link" href={last_url.clone()}>{ total_page }</a>
            </li>
        }
    } else if total_page - curr_page == 2 {
        html! {
            <>
            <li>
                <a class="pagination-link" href={next_url.clone()}>{ total_page - 1 }</a>
            </li>
            <li>
                <a class="pagination-link" href={last_url.clone()}>{ total_page }</a>
            </li>
            </>
        }
    } else {
        html! {
            <>
            <li>
                <a class="pagination-link" href={next_url}>{ curr_page + 1 }</a>
            </li>
            <li>
                <span class="pagination-ellipsis">{ "..." }</span>
            </li>
            <li>
                <a class="pagination-link" href={last_url}>{ total_page }</a>
            </li>
            </>
        }
    };

    let curr = html! {
        <li>
            <a class="pagination-link is-current">{ curr_page }</a>
        </li>
    };

    html! {
        <nav class="pagination is-centered is-rounded">
            { prev }
            { next }
            <ul class="pagination-list">
                { left }
                { curr }
                { right }
            </ul>
        </nav>
    }
}

pub fn created_and_modified_time(mut create: i64, modify: i64) -> Html {
    if modify > 0 && create == 0 {
        create = modify;
    }

    if modify > 0 {
        let create_date = chrono::NaiveDateTime::from_timestamp(create, 0);
        let modify_date = chrono::NaiveDateTime::from_timestamp(modify, 0);

        html! {
            <div class="block">
                <span class="material-icons" style="font-size:16px">{ "insert_invitation" }</span>
                <span>
                { format!("发表于：{}", create_date.format("%Y-%m-%d %H:%M:%S").to_string()) }
                </span>
                <span class="m-2">{ " " }</span>
                <span class="material-icons" style="font-size:16px">{ "edit_calendar" }</span>
                <span>
                { format!("更新于：{}", modify_date.format("%Y-%m-%d %H:%M:%S").to_string()) }
                </span>
            </div>
        }
    } else {
        html! {}
    }
}
