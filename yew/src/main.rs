use yew::prelude::*;
use yew::function_component;
use yew::html;

fn main() {
    yew::start_app::<Saludo>();
}

#[function_component(Saludo)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Hi!"}</h1>
        </div>
    }
}
