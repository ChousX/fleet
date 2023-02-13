use std::rc::Rc;

#[macro_use]
use enum_display_derive;
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use courier::account_data::AccountData;

mod components;
mod pages;

use pages::Route;

use components::organs::NavBar;
use yewdux::prelude::use_store;

#[styled_component(App)]
pub fn app() -> Html {
    let (account_info, _account_info_dispatch) = use_store::<AccountData>();
    html! {
        <BrowserRouter>
            <ContextProvider<Rc<AccountData>> context={account_info}>
                <NavBar/>
                <Switch<Route> render={Route::switch}/>
            </ContextProvider<Rc<AccountData>>>
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
