#[macro_use]
use enum_display_derive;
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
mod components;
mod pages;

mod stores;
use pages::Route;

use components::organs::NavBar;

#[styled_component(App)]
pub fn app() -> Html {
    // let user = User {
    //     username: "Garrett".to_owned(),
    //     token: "".to_owned(),
    // };

    html! {
        // <ContextProvider<User> context={user}>
            <BrowserRouter>
                    <NavBar/>
                <Switch<Route> render={Route::switch} />
            </BrowserRouter>
           
        // </ContextProvider<User>>
    }
}
