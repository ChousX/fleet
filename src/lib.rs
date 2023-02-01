use std::rc::Rc;

#[macro_use]
use enum_display_derive;
use stores::account_info::AccountInfo;
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
mod components;
mod pages;

mod stores;
use pages::Route;

use components::organs::NavBar;
use yewdux::prelude::use_store;

#[styled_component(App)]
pub fn app() -> Html {
    let (account_info, _account_info_dispatch) = use_store::<AccountInfo>();
    html!{
        <BrowserRouter>
            <ContextProvider<Rc<AccountInfo>> context={account_info}>
                <NavBar/>
                <Switch<Route> render={Route::switch}/>
            </ContextProvider<Rc<AccountInfo>>>
        </BrowserRouter>

    }
}

/*
    html! {
        <BrowserRouter>
            <NavBar/>
            <ContextProvider<AuthStore> context={AuthStore::default()}>
                <Switch<Route> render={Route::switch}/>
            </ContextProvider<AuthStore>>
        </BrowserRouter>
    }
*/