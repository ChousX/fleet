use std::fmt::Display;

use yew::prelude::*;
use yew_router::prelude::Navigator;

use crate::pages::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Nav)]
pub fn nav(Props { name, onclick }: &Props) -> Html {
    if let Some(clicked) = onclick {
        html! {
            <a onclick={clicked}>{name}</a>
        }
    } else {
        html! {
            <a>{name}</a>
        }
    }
}
