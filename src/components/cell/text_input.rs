use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    text: String,
}

#[function_component(TextInput)]
pub fn text_input(Props { text, .. }: &Props) -> Html {
    html! {}
}
