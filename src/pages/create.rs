use yew::prelude::*;

use crate::components::tissues::form::{Form, FormEntry};

pub enum Msg{

}
#[derive(Properties, PartialEq)]
pub struct Props {

}

pub struct CreateAccount;
impl Component for CreateAccount {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use FormEntry::*;
        html!{
            <Form entries={vec![Text("Username".to_owned()), Text("Email Adress".to_owned()), Password("Password".to_owned())]}/>
        }
    }
}


