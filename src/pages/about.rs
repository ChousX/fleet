use yew::prelude::*;

// pub struct About;

// impl Component for About {
//     type Message = ();
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//         }
//     }
// }

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <h1>{"Home"}</h1>
    }
}
