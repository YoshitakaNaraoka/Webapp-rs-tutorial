use stylist::{css, Style};

pub struct BgState {
    pub background_state: Style,
}

pub fn get_base_styles() -> Style {
    Style::new(css!(
        r#"
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
        "#
    ))
    .unwrap()
}

pub fn container_styles() -> Style {
    Style::new(css!(
        r#"
            margin: 0;
            padding-top: 10vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
            text-align: center;
        "#
    ))
    .unwrap()
}

pub fn center_styles() -> Style {
    Style::new(css!(
        r#"
            justify-content: center;
            text-align: center;
        "#
    ))
    .unwrap()
}

pub fn row_styles() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: center;
        "#
    ))
    .unwrap()
}

pub fn get_light_mode_styles() -> Style {
    Style::new(css!(
        r#"
            color: #0f0f0f;
            background-color: #f6f6f6;
        "#
    ))
    .unwrap()
}

pub fn get_dark_mode_styles() -> Style {
    Style::new(css!(
        r#"
            color: #f6f6f6;
            background-color: #2f2f2f;
        "#
    ))
    .unwrap()
}