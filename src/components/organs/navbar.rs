use std::fmt::Display;

use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::{navigator, prelude::*};

use crate::{components::tissues::nav_list::NavList, pages::Route};

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    // let context = use_context::<User>();
    let nav_bar_style = style! {
        r#"
        "#
    }
    .expect("failed to build css");
    let nav_lists = Route::NAV
        .iter()
        .map(|routes| {
            let routes = Vec::from(*routes);
            html! {
                <NavList routes={routes}/>
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
