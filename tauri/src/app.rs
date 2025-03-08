use stylist::yew::styled_component;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::style::{get_base_styles, get_light_mode_styles, get_dark_mode_styles}; // styles.rs から各スタイルを取得

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[styled_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(
            name2,
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    let background_state = use_state(|| false);

    let toggle_light: Callback<MouseEvent> = {
        let background_state = background_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            background_state.set(
                !*background_state.cast::<web_sys::HtmlStyleElement>()
            .unwrap().value,
            );
        }
        )
    };


    let base_styles = get_base_styles();
    let light_mode_styles = get_light_mode_styles();
    let dark_mode_styles = get_dark_mode_styles();

    let mut classes = Classes::new();
    classes.push(base_styles.clone());
    if *background_state {
        classes.push(dark_mode_styles);
        classes.push("dark_mode");
    } else {
        classes.push(light_mode_styles);
        classes.push("light_mode");
    };

    html! {
        <main class={classes!("container")}>
            <h1>{"Welcome to Tauri + Yew"}</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>

            </div>
            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>
            <p>{&*greet_msg}</p>

            <div>
                <button type="submit" onclick={toggle_light}>{"Toggle Background Mode"}</button>
            </div>
        </main>
    }
}