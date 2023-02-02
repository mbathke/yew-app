use yew::Properties;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub id: usize,
}

#[function_component(Detail)]
pub fn detail(props: &DetailProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    // load detailed video

    html! {
        <div>
            <h2>{ "Detail" }</h2>
            <h3>{ "Value of id:" }{props.id.clone()}</h3>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
