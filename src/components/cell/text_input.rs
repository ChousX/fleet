use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub lable: String,
    pub place_holder: Option<String>,
    pub t_ype: Option<String>,
    pub f_or: Option<String>,
}

#[function_component(TextInput)]
pub fn text_input(
    Props {
        lable,
        place_holder,
        f_or,
        t_ype,
    }: &Props,
) -> Html {
    html! {
        <div>
            <div>
                <lable for={f_or.clone().unwrap_or("input".to_owned())}>{lable}</lable>
            </div>
            <div>
                <input type={t_ype.clone().unwrap_or("text".to_owned())} id="input" placeholder={place_holder.clone().unwrap_or_default()} />
            </div>
        </div>
    }
}
