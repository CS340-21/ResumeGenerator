use super::{Color, HTML};

pub trait Theme {
    fn get_color_rgb(&self, color: Color) -> (u8, u8, u8);
    fn get_color_hex(&self, color: Color) -> String {
        let (r, g, b) = self.get_color_rgb(color);
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn get_document_css(&self) -> String {
        r#"
@import url('https://fonts.googleapis.com/css2?family=Merriweather:wght@300&display=swap');
// @import url('https://fonts.googleapis.com/css2?family=Roboto&display=swap');
@import url('https://fonts.googleapis.com/css2?family=Roboto:ital@1&display=swap');
* {
    // font-family: 'Merriweather', serif;
    font-family: 'Roboto', sans-serif;
}
"#
        .to_string()
    }

    fn compile_section_html(&self, content: String) -> String {
        format!("<div class=\"card\" style=\"height:100%; width:100%;\"><div class=\"card-body\" style=\"height:100%; width:100%;\">{}</div></div>", content)
    }
}
