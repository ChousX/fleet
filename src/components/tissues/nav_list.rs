use std::fmt::Display;

use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};

use crate::{components::cell::nav_element::Nav, pages::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub routes: Vec<Route>,
    pub name: Option<String>,
}

#[function_component(NavList)]
pub fn nav(Props { routes, name }: &Props) -> Html {
    if routes.len() < 1 {
        panic!("entrys are less then 1 in the nav list")
    }
    let multy = routes.len() > 1;
    let mut routes = routes.clone();
    let first = routes.remove(0);
    let navigator = use_navigator();
    //Clickable
    let afters = if let Some(navigator) = navigator.clone() {
        routes
            .into_iter()
            .map(|route| {
                let route = route.clone();
                let navigator = navigator.clone();
                    html! {
                        <li><Nav name={route.to_string()} onclick={Callback::from(move |_| navigator.push(&route))}/></li>
                    }
            })
            .collect::<Html>()
    } else {
        routes
            .into_iter()
            .map(|route| {
                html! {
                    <li><Nav name={route.to_string()}/></li>
                }
            })
            .collect::<Html>()
    };

    let name = if let Some(name) = name {
        name.to_owned()
    } else {
        first.to_string()
    };
    let out_first = if let Some(navigator) = navigator {
        let first = first.clone();
        html! {
            <Nav name={name} onclick={Callback::from(move |_| navigator.push(&first))}/>
        }
    } else {
        html! {
            <Nav name={name}/>
        }
    };

    if multy {
        html! {
            <li>
                {out_first}
                <ul>
                    {afters}
                </ul>
            </li>
        }
    } else {
        html! {
            <li>{out_first}</li>
        }
    }
}
