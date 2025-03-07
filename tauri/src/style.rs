use stylist::style;
use yew::prelude::*;


#[derive(Clone, PartialEq, Properties)]
pub struct BackgroundComponentProps {
  pub background_state: bool,
}
#[function_component(BackgroundComponent)]
pub fn button_bg_component(props: &BackgroundComponentProps) -> Html {
  let style = style!(
    r#"
    :host {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;
        background-color: #f6f6f6;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }
    &.light_mode {
        color: #0f0f0f;
        background-color: #f6f6f6;
    }

    &.dark_mode {
        color: #f6f6f6;
        background-color: #2f2f2f;
    }
    "#
    ).unwrap();

  let class = if props.background_state { "dark_mode" } else { "light_mode" };
  html! {
    <div class={classes!(style, class.to_string())}>
    </div>
  }
}
