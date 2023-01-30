use std::fmt::Display;

use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};

use crate::{components::cell::nav_element::Nav, pages::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub routes: Vec<Route>,
}

#[function_component(NavList)]
pub fn nav(Props { routes }: &Props) -> Html {
    if routes.len() < 1 {
        panic!("entrys are less then 1 in the nav list")
    }
    let first = routes.first().unwrap();
    let dropdown = if routes.len() > 1{"DropDown"} else {"Single"};

    let navs = if let Some(navigator) = use_navigator() {
        routes
            .into_iter()
            .map(|route| {
                let route = route.clone();
                let navigator = navigator.clone();
                html! {
                    <Nav name={route.to_string()} onclick={Callback::from(move |_| navigator.push(&route))}/>
                }
            })
            .collect::<Html>()
    } else {
        routes
            .into_iter()
            .map(|entry| {
                html! {
                    <Nav name={entry.to_string()}/>
                }
            })
            .collect::<Html>()
    };

    html! {
        <div class={dropdown}>
        <ul class={first.to_string()}>
            {navs}
        </ul>
        </div>
    }
}
