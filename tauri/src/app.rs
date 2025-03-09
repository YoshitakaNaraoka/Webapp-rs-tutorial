use stylist::yew::styled_component;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::style::*;
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

    // モードの状態を保持する変数(初期値はライトモード)
    let dark_mode = use_state(|| false);

    let toggle_light: Callback<MouseEvent> = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // ライトモードに設定
            dark_mode.set(false);
        })
    };
    let toggle_dark: Callback<MouseEvent> = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dark_mode.set(true);
        })
    };

    let mut main_classes = Classes::new();
    main_classes.push(container_styles());
    if *dark_mode {
        main_classes.push(get_dark_mode_styles());
    } else {
        main_classes.push(get_light_mode_styles());
    };
    
    
    html! {
        <>
            <main class={main_classes}>
                <h1>{"Welcome to Tauri + Yew"}</h1>

                <div class={classes!(row_styles())}>
                    <a href="https://tauri.app" target="_blank">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://yew.rs" target="_blank">
                        <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                    </a>

                </div>
                <p class={classes!(center_styles())}>{"Click on the Tauri and Yew logos to learn more."}</p>

                <form class="row" onsubmit={greet}>
                    <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                    <button type="submit">{"Greet"}</button>
                </form>
                <p>{&*greet_msg}</p>

                <div class={classes!(center_styles())}>
                    <button type="submit" onclick={toggle_light}>{"Toggle Light Mode"}</button>
                    <button type="submit" onclick={toggle_dark}>{"Toggle Dark Mode"}</button>
                </div>
                <p>{if *dark_mode {"Dark Mode"} else {"Light Mode"}}</p>
            </main>
        </>
    }
}
// let mut brightness_mode:UseStateHandle<bool> = use_state(|| false);

#[function_component(App)]
fn toggle_brightness(brightness_mode: UseStateHandle<bool>) {
    // モードの状態を保持する変数(初期値はライトモード)
    
    let brightness_mode = dark_mode.clone();


    let toggle_light: Callback<MouseEvent> = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // ライトモードに設定
            brightness_mode.set(false);
        })
    };
    let toggle_dark: Callback<MouseEvent> = {
        let brightness_mode = brightness_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            brightness_mode.set(true);
        })
    };

    let mut main_classes = Classes::new();
    main_classes.push(container_styles());
    if *brightness_mode {
        main_classes.push(get_dark_mode_styles());
    } else {
        main_classes.push(get_light_mode_styles());
    };
}
