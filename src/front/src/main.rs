use yew::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::get;

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    text: String,
}

#[function_component(App)]
fn app() -> Html {
    let message = use_state(|| String::new());

    {
        let message = message.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_message = get("http://127.0.0.1:8080/api/message")
                    .await
                    .unwrap()
                    .json::<Message>()
                    .await
                    .unwrap();
                message.set(fetched_message.text);
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h1>{ "Message from Backend:" }</h1>
            <p>{ (*message).clone() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}