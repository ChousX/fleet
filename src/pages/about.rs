use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <h1>{"About"}</h1>
            <h3>{"Tech stack for this website:"}</h3>
        </>
    }
}
