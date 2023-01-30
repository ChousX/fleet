use std::fmt::Display;

use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::{navigator, prelude::*};

use crate::{pages::Route, components::tissues::nav_list::NavList};

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    // let context = use_context::<User>();
    let nav_bar_style = style! {
        r#"
        .navbar{
            border-top:1px solid #ddd;
            border-bottom:1px solid #ddd;
            margin:2em;
            
        }
        
        li a{
            display:flex;
            text-align:center;
            text-decoration:none;
            color:#777;
            padding:1rem;
        }

        li a:hover{
            background:#777;
            color:#fff;
          }

          @media (min-width: 800px) {
            .menu{
              flex-direction:row;
              justify-content:space-between;
            }
        }



        "#
    }
    .expect("failed to build css");
    let nav_lists = Route::NAV.iter().map(|routes|{
        let routes = Vec::from(*routes);
        html!{
            <NavList routes={routes}/>
        }
    }).collect::<Html>();
    
    html! {
        <div class= {nav_bar_style}>
            <nav class = {"navbar"}>
                <ul class="menu">{nav_lists}</ul>
            </nav>
        </div>
    }
}


