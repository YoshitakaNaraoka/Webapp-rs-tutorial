use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
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
/*
    let color_mode = use_state(|| bool);
    let light_mode = "light";
    let dark_mode = "dark";
    match  {
        color_mode = true; => light_mode,
        color_mode = false; => dark_mode,
    }
    let mut background_color = [light_mode, dark_mode];
    let mut toggle_light != fn (!color_mode) {
        color_mode.set(!color_mode);
        
    }
*/
    let color_mode = use_state(|| String::new());
    let light_mode = ".light_mode";
    let dark_mode = ".dark_mode";
    let toggle_light = {
        let light_mode = light_mode.clone();
        let dark_mode = dark_mode.clone();
        let color_mode = color_mode.clone();
        Callback::from(move |_| {
            if color_mode.is_empty() {
                color_mode.set(light_mode.to_string());
            } else if color_mode == light_mode {
                color_mode.set(dark_mode.to_string());
            } else {
                color_mode.set(light_mode.to_string());
            }
        })
    };
    
    html! {
        <main class={"container"}>
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
            <p>{ &*greet_msg }</p>
            <div class="row" /*onsubmit={background_color}*/>
                <button type="submit">{"background_mode"}</button>
            </div>
        </main>
    }
}
