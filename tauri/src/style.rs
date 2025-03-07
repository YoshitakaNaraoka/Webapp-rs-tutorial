use stylist::{style, Style};

pub fn background() -> Style {
  style!(
    r#"
    &.light_mode {
    color: #0f0f0f;
    background-color: #f6f6f6;
    }

    &.dark_mode {
    color: #f6f6f6;
    background-color: #2f2f2f;
    }
    "#
  ).unwrap()
}