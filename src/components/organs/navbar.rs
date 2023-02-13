use std::{fmt::Display, rc::Rc};

use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::{navigator, prelude::*};

use courier::account_data::AccountData;

use crate::{pages::Route, components::tissues::nav_list::NavList};

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    let nav_bar_style = style! {
        r#"
        "#
    }
    .expect("failed to build css");

    let data = if let Some(account_info) = use_context::<Rc<AccountData>>() {
        account_info.clone()
    } else {
        panic!("")
    };
    
    let nav_lists = Route::NAV
        .iter()
        .map(|routes| {
            let routes = Vec::from(*routes);
            if routes[0] == Route::Account && data.name.is_some() {
                let name = data.name
                    .as_ref()
                    .expect("some very old things happend to brake here");
                html! {
                    <NavList routes={routes} name={name.clone()}/>
                }
            } else {
                html! {
                    <NavList routes={routes}/>
                }
            }
        })
        .collect::<Html>();

    html! {
        <div class= {nav_bar_style}>
            <nav class = {"navbar"}>
                <ul class="menu">{nav_lists}</ul>
            </nav>
        </div>
    }
}
