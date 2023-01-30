use std::fmt::Display;

use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::{navigator, prelude::*};

use crate::pages::Route;

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    // let context = use_context::<User>();
    let nav_bar_style = style! {
        r#"
        ul {
            list-style-type: none;
            margin: 0;
            padding: 0;
            overflow: hidden;
            background-color: #333;
            float: top
            
        }
        
        li {
            float: left;
            
        }

        li.Account{
            float: right
        }
        
        li a {
            display: block;
            color: white;
            text-align: center;
            padding: 14px 16px;
            text-decoration: none;
          }
        
        li a:hover {
            background-color: #111;
        }
        "#
    }
    .expect("failed to build css");

    let navigator = use_navigator().unwrap();
    html! {
        <nav class = {nav_bar_style}>
            <ul>
                {Route::NAV.into_iter().map(|n| html!{
                    // if *n == Route::Account{
                    //     if let Some(User{username, ..}) = &context{
                    //         {listify(username, n, &navigator)}
                    //     } else {
                    //         {listify(n, n, &navigator)}
                    //     }
                    // } else {
                        {listify(n, n, &navigator)}
                    // }
                }).collect::<Html>()}
            </ul>
        </nav>
    }
}

fn listify(name: &impl Display, route: &Route, navigator: &Navigator) -> Html {
    let route = route.clone();
    let navigator = navigator.clone();
    html! {
        <li class = {route.to_string()}>
            <a onclick={Callback::from(move |_| navigator.push(&route))}>{name}</a>
        </li>
    }
}
