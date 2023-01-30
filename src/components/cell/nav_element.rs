use std::fmt::Display;

use yew::prelude::*;
use yew_router::prelude::Navigator;

use crate::pages::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onclick: Option<Callback<MouseEvent>>,
    pub dropdown: Option<bool>,
}

#[function_component(Nav)]
pub fn nav(Props { name, onclick,  dropdown}: &Props) -> Html {
    let class = if dropdown.unwrap_or(true) {
        "nav_dropdown"
    } else {
        "nav_first"
    };
    if let Some(clicked) = onclick {
        html! {
            <a onclick={clicked} class={class}>{name}</a>
        }
    } else {
        html! {
            <a class="nav_cell">{name}</a>
        }
    }
}

