use std::rc::Rc;

use yew::{prelude::*, Component, Properties};
use yewdux::{prelude::Dispatch, store::Store};

use crate::components::cell::text_input::TextInput;


pub struct Form {
    pub dispatch: Dispatch<FormData>,
    pub entries: Vec<FormEntry>,
    pub callback: Option<Callback<FormData>>
}

pub enum Msg {
    Store(Rc<FormData>),
    UpdateEntry(usize, FormEntry),
    Submit
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entries: Vec<FormEntry>,
    pub callback: Option<Callback<FormData>>
}

#[derive(PartialEq, Clone,)]
pub enum FormEntry {
    Text(String),
    Password(String),
}
impl FormEntry{
    pub fn test() -> Vec<Self>{
        use FormEntry::*;
        vec![Text("Name".to_owned()), Password("Password".to_owned()), Text("Note".to_owned())]
    } 
}

#[derive(Clone, PartialEq, Store, Default)]
pub struct FormData {pub entries: Vec<FormEntry>}

impl Component for Form {
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Store(_) => false,
            Msg::UpdateEntry(index, entry) => {
                let mut data = self.dispatch.get().as_ref().clone();
                data.entries[index] = entry;
                self.dispatch.set(data);
                false
            },
            Msg::Submit => {
                if let Some(callback) = self.callback.clone(){
                    let data = self.dispatch.get().as_ref().clone();
                    callback.emit(data);
                }
                true
            },
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &yew::Context<Self>) {}

    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let Props { entries, callback } = ctx.props();
        let dispatch = Dispatch::<FormData>::subscribe(ctx.link().callback(Msg::Store));
        let data = FormData{
            entries: entries.iter().map(|e| { 
                match *e{
                    FormEntry::Text(_) => FormEntry::Text("".to_owned()),
                    FormEntry::Password(_) => FormEntry::Password("".to_owned()),
                }
            }).collect(),
        };
        dispatch.set(data);
        Self {
            dispatch,
            entries: entries.clone(),
            callback: callback.clone()
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let entries = self.entries.iter().enumerate().map(|(i, entry)|{
            match entry.clone(){
                FormEntry::Text(text) => {
                    html!{
                        <TextInput lable={text} handle_onchange={ctx.link().callback(move |s| Msg::UpdateEntry(i, FormEntry::Text(s)))}/>
                    }
                },
               FormEntry::Password(password) => {
                
                    html!{
                        <TextInput lable={password} handle_onchange={ctx.link().callback(move |s| Msg::UpdateEntry(i, FormEntry::Password(s)))} t_ype={"password"} place_holder={"********"}/>
                    }
                },
            }
        }).collect::<Html>();

        html! {
            <div>
                {entries}
                <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Create"}</button>
            </div>
        }
    }
}
