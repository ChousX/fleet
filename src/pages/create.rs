use yew::prelude::*;

use crate::components::tissues::{auth_form::AuthForm, new_account::AccountCreateForm};

#[function_component(Create)]
pub fn create() -> Html {
    html! {
        <>
            <img src="/img/fleet.png" alt="Fleet Logo"/>
            <h1>{"Create New Account"}</h1>
            <AccountCreateForm/>
        </>
    }
}
