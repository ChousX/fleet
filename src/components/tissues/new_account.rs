use std::rc::Rc;

use stylist::{style, Style};
use yew::{prelude::*, callback};
use yewdux::{prelude::Dispatch, store::Store};
use crate::{components::cell::text_input::TextInput, stores::authstore::*};

pub enum Msg {
    Store(Rc<Form>),
    UpdateUserName(String),
    UpdatePassword(String),
    UpdateEmail(String),
    Submit
}

#[derive(Debug, Clone, PartialEq, Eq, Store, Properties, Default)]
pub struct Form{
    user_name: String,
    password: String,
    email: String,
}

pub struct AccountCreateForm {
    pub stylesheet: Style,
    pub form_dispatch: Dispatch<Form>,
    pub test: bool,
}

impl Component for AccountCreateForm {
    type Message = Msg;
    type Properties = ();

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Store(_store) => {
                false
            },
            Msg::UpdateUserName(input) => {
                let form = self.form_dispatch.get();
                self.form_dispatch.set(Form{
                    user_name: input,
                    password: form.password.clone(),
                    email: form.email.clone(),
                });
                false
            },
            Msg::UpdatePassword(input) => {
                let form = self.form_dispatch.get();
                self.form_dispatch.set(Form{
                    user_name: form.user_name.clone(),
                    password: input,
                    email: form.email.clone(),
                });
                false
            },
            
            Msg::UpdateEmail(input) => {
                let form = self.form_dispatch.get();
                self.form_dispatch.set(Form{
                    user_name: form.user_name.clone(),
                    password: form.password.clone(),
                    email: input,
                });
                false
            },

            Msg::Submit => {
                self.test = !self.test;
                true
            }
        }
        
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
        let stylesheet = style! {
            r#"
            "#
        }
        .expect("failed to build auth_form");
        let form_dispatch = Dispatch::<Form>::subscribe(ctx.link().callback(Msg::Store));
        Self {
            stylesheet,
            form_dispatch,
            test: false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let form = self.form_dispatch.get();
        html! {
            <div class="auth_form">
                <TextInput lable={"Emailaddress"} place_holder={"joheshmo@gmail.com"} handle_onchange={ctx.link().callback(|s: String| Msg::UpdateEmail(s))}/>
                <TextInput lable={"Username"} place_holder={"username"} handle_onchange={ctx.link().callback(|s: String| Msg::UpdateUserName(s))}/>
                <TextInput lable={"Passsword"} place_holder={"********"} t_ype={"password"} handle_onchange={ctx.link().callback(|s: String| Msg::UpdatePassword(s))}/>
                <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Create"}</button>
            </div>
        }
    }
}
