use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    text: String,
}

#[function_component(App)]
fn app() -> Html {
    let message = use_state(|| String::new());
    let message_clone = message.clone();

    use_effect(move || {
        spawn_local(async move {
            let fetched_message = Request::get("http://127.0.0.1:8080/api/message")
                .send()
                .await
                .unwrap()
                .json::<Message>()
                .await
                .unwrap_or(Message { text: "Error fetching message".to_string() });
                message_clone.set(fetched_message.text);
            });
            || ()
    });

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
