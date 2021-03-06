use super::{Color, HTML};

pub trait Theme {
    fn get_color_rgb(&self, color: Color) -> (u8, u8, u8);
    fn get_color_hex(&self, color: Color) -> String {
        let (r, g, b) = self.get_color_rgb(color);
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn compile_section_html(&self, content: HTML) -> String where Self: Sized {
        format!("<div class=\"card\" style=\"height:100%\"><div class=\"card-body\" style=\"height:100%\">{}</div></div>", content.compile(self))
    }
}