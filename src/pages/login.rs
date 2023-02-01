use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

use crate::components::{cell::text_input::TextInput, tissues::form::{Form, FormEntry}};

#[styled_component(Login)]
pub fn login() -> Html {
    use FormEntry::*;
    html! {
        <div class={get_style()}>
            <div class={"center"}>
                <img src="/img/fleet.png" alt="Fleet Logo"/>
                <h1>{"Login"}</h1>
                <Form entries={vec![Text("Username".to_owned()), Password("Password".to_owned())]}/>
            </div>
        </div>
    }
}

fn get_style() -> Style {
    style! {
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
    }
    .expect("failed to build stlye sheet login_page")
}
