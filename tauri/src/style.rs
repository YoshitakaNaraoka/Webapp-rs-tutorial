use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BackgroundComponentProps {
    pub background_state: bool,
}

#[styled_component(BackgroundComponent)]
pub fn button_bg_component(props: &BackgroundComponentProps) -> Html {
    let stylesheet = Style::new(css!(
        r#"
        :host {
            font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
            font-size: 16px;
            line-height: 24px;
            font-weight: 400;

            font-synthesis: none;
            text-rendering: optimizeLegibility;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
            -webkit-text-size-adjust: 100%;
        }
        .light_mode {
            color: #0f0f0f;
            background-color: #f6f6f6;
        }

        .dark_mode {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }
        "#
    )).unwrap();
    let class = if props.background_state {
        "dark_mode"
    } else {
        "light_mode"
    };

    let mut classes = Classes::new();
    classes.push(class);

    html! {
        <div class={classes} {stylesheet}>{ "This is Background" }</div>
    }
}
