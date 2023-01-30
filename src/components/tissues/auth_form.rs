use std::rc::Rc;

use stylist::{Style, style};
use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::{stores::authstore::*, components::cell::text_input::TextInput};

pub enum Msg{
    Store(Rc<AuthStore>),
    Username(String),
    Password(String),
    Login
}

pub struct AuthForm{
    dispatch: Dispatch<AuthStore>,
    pub stylesheet: Style,
}

impl Component for AuthForm{
    type Message = Msg;
    type Properties = ();

    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {}


    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));
        let stylesheet = style!{
            r#"
            "#
        }.expect("failed to build auth_form");
        Self { dispatch, stylesheet }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class="auth_form">
                <TextInput lable={"Username"} place_holder={"username"}/>
                <TextInput lable={"Passsword"} place_holder={"********"} t_ype={"password"}/>
                <button onclick={Callback::from(|_| ())}>{"Login"}</button>
            </div>
        }
    }
}