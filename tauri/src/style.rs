use stylist::{css, Style};

pub struct BgState {
    pub background_state: Style,
}

pub fn get_base_styles() -> Style {
    Style::new(css!(
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
            text-align: center;
        }
        "#
    ))
    .unwrap()
}

pub fn get_light_mode_styles() -> Style {
    Style::new(css!(
        r#"
        .light_mode {
            color: #0f0f0f;
            background-color: #f6f6f6;
        }
        "#
    ))
    .unwrap()
}

pub fn get_dark_mode_styles() -> Style {
    Style::new(css!(
        r#"
        .dark_mode {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }
        "#
    ))
    .unwrap()
}