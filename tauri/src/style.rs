use stylist::{css, Style};

pub struct BgState {
    pub background_state: bool,
}

pub fn get_styles(_background_state: bool) -> Style {
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
            text-align: center;
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
    ))
    .unwrap();

    stylesheet
}