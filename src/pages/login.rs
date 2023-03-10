use stylist::{yew::styled_component, style, Style};
use yew::prelude::*;

use crate::components::{cell::text_input::TextInput, tissues::auth_form::AuthForm};

#[styled_component(Login)]
pub fn login() -> Html {
    html! {
        <body>
            <div class={get_style()}>
                <div class={"center"}> 
                    <img src="/img/fleet.png" alt="Fleet Logo"/> 
                    <h1>{"Login"}</h1>
                    <AuthForm/>
                </div>
            </div>
        </body>
    }
}


fn get_style() -> Style{
    style!{
        r#" 
            img {
                width: 148px;
            }

            .center {
                width: 148px;
                hieght: 300px;
                margin: auto;
            }
        "#
    }.expect("failed to build stlye sheet login_page")
}