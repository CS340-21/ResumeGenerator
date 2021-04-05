use super::{Color, Direction, HorizontalAlignment, Theme, VerticalAlignment};

// The actual internal structure that is directly 1:1 with the output code.
// When the program finally inserts the user data into the resume, the
// resume is converted into an HTML instance. Then, the HTML instance
// is converted to text with a `get_html()` method.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HTML {
    // Encloses the entire document
    Document(Vec<Self>),

    // A nice div to hold other things in a fixed width
    Container(Vec<Self>),

    Aligned(Box<Self>, HorizontalAlignment, VerticalAlignment),

    Row(Vec<Self>),
    Column(Vec<Self>),

    // Aligned(Box<Self>, HorizontalAlignment, VerticalAlignment),
    Text(String),
    Title(String),
    SectionTitle(String),

    // 1, 2, 3, ...
    OrderedList(Vec<Self>),
    // Dotted list
    UnorderedList(Vec<Self>),

    PercentBar(u32, String),

    // A colored rectangle with border radius
    Rectangle(Box<Self>, u32, Color),

    // A place for text that will be specially formatted by the theme
    Section(Box<Self>),

    // Make everything inside italicized
    Italics(Box<Self>),
    // Bold everything inside
    Bold(Box<Self>),
    // Turn whatever is inside into a link
    Link(Box<Self>, String),
    // Colorize text
    ColoredForeground(Box<Self>, Color),
    ColoredBackground(Box<Self>, Color),

    FadeIn(Direction),
}

impl From<String> for HTML {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl HTML {
    pub fn html(contents: Vec<Self>) -> Self {
        Self::Document(contents)
    }

    pub fn aligned(contents: Self, hori: HorizontalAlignment, vert: VerticalAlignment) -> Self {
        Self::Aligned(Box::new(contents), hori, vert)
    }

    pub fn title(title: impl ToString) -> Self {
        Self::Title(title.to_string())
    }
    pub fn section_title(title: impl ToString) -> Self {
        Self::SectionTitle(title.to_string())
    }

    pub fn text(text: impl ToString) -> Self {
        Self::Text(text.to_string())
    }
    pub fn rect(content: Self, border_radius: u32, color: Color) -> Self {
        Self::Rectangle(Box::new(content), border_radius, color)
    }

    pub fn section(content: Self) -> Self {
        Self::Section(Box::new(content))
    }
    pub fn italics(content: Self) -> Self {
        Self::Italics(Box::new(content))
    }
    pub fn bold(content: Self) -> Self {
        Self::Bold(Box::new(content))
    }
    pub fn link(content: Self, link: impl ToString) -> Self {
        Self::Link(Box::new(content), link.to_string())
    }

    pub fn fg(content: Self, color: Color) -> Self {
        Self::ColoredForeground(Box::new(content), color)
    }
    pub fn bg(content: Self, color: Color) -> Self {
        Self::ColoredBackground(Box::new(content), color)
    }

    pub fn row<T>(items: Vec<T>) -> Self
    where
        T: Into<HTML>,
    {
        Self::Row(items.into_iter().map(Into::into).collect())
    }

    pub fn col<T>(items: Vec<T>) -> Self
    where
        T: Into<HTML>,
    {
        Self::Column(items.into_iter().map(Into::into).collect())
    }

    pub fn container<T>(items: Vec<T>) -> Self
    where
        T: Into<HTML>,
    {
        Self::Container(items.into_iter().map(Into::into).collect())
    }

    pub fn ol<T>(items: Vec<T>) -> Self
    where
        T: Into<HTML>,
    {
        Self::OrderedList(items.into_iter().map(Into::into).collect())
    }

    pub fn ul<T>(items: Vec<T>) -> Self
    where
        T: Into<HTML>,
    {
        Self::UnorderedList(items.into_iter().map(Into::into).collect())
    }

    pub fn compile(&self, theme: &dyn Theme) -> String {
        match self {
            Self::Document(contents) => {
                format!("<!DOCTYPE html5><html><head><meta content=\"text/html;charset=utf-8\" http-equiv=\"Content-Type\"><meta content=\"utf-8\" http-equiv=\"encoding\"><link rel=\"stylesheet\" href=\"https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css\" integrity=\"sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T\" crossorigin=\"anonymous\"><script src=\"https://code.jquery.com/jquery-3.3.1.slim.min.js\" integrity=\"sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo\" crossorigin=\"anonymous\"></script><script src=\"https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.7/umd/popper.min.js\" integrity=\"sha384-UO2eT0CpHqdSJQ6hJty5KVphtPhzWj9WO1clHTMGa3JDZwrnQq4sF86dIHNDz0W1\" crossorigin=\"anonymous\"></script><script src=\"https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/js/bootstrap.min.js\" integrity=\"sha384-JjSmVgyd0p3pXB1rRibZUAYoIIy6OrQ6VrjIEaFf/nJGzIxFDsf4x0xIM+B07jRM\" crossorigin=\"anonymous\"></script><style>{}\nbody, div {{ color: {}; background-color: {}; }}</style></head><body><br><br><br><br><br><br>{}<br><br><br><br><br><br></body></html>",
                theme.get_document_css(),
                theme.get_color_hex(Color::DefaultForeground),
                theme.get_color_hex(Color::DefaultBackground),
                contents.iter()
                    .map(|i| i.compile(theme))
                    .collect::<Vec<String>>()
                    .join("\n"))
            }

            Self::PercentBar(part, label) => {
                format!("<div class=\"progress\"><div class=\"progress-bar\" role=\"progressbar\" style=\"width:{part}%\"  aria-valuenow=\"{part}\" aria-valuemin=\"{part}\" aria-valuemax=\"100\">{}</div></div>", label, part=part)
            }

            Self::Aligned(contents, hori, vert) => {
                format!("<div style=\"height:100%; width:100%; display: flex; flex-grow: 1; {}{}\">{}</div>", match hori {
                    HorizontalAlignment::Left => "justify-content: left;",
                    HorizontalAlignment::Right => "justify-content: right;",
                    HorizontalAlignment::Center => "justify-content: center;",
                    HorizontalAlignment::SameAsParent => "",
                }, match vert {
                    VerticalAlignment::Top => "align-items: flex-start;",
                    VerticalAlignment::Center => "align-items: center;",
                    VerticalAlignment::Bottom => "align-items: flex-end;",
                    VerticalAlignment::SameAsParent => "",
                }, contents.compile(theme))
            }

            Self::Row(items) => {
                format!(
                    "<div class=\"row no-gutters\">{}</div>",
                    items
                        .iter()
                        .map(|i| format!(
                            "<div class=\"col no-gutters\">{}</div>",
                            i.compile(theme)
                        ))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }

            Self::Column(items) => {
                format!(
                    "<div class=\"col no-gutters\">{}</div>",
                    items
                        .iter()
                        .map(|i| format!(
                            "<div class=\"row no-gutters\">{}</div>",
                            i.compile(theme)
                        ))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }

            Self::Container(items) => {
                format!(
                    "<div class=\"container\">{}</div>",
                    items
                        .iter()
                        .map(|i| i.compile(theme))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }

            Self::OrderedList(items) => {
                format!(
                    "<ol style=\"width:100%\">{}</ol>",
                    items
                        .iter()
                        .map(|i| format!("<li>{}</li>", i.compile(theme)))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }

            Self::UnorderedList(items) => {
                format!(
                    "<ul style=\"width:100%\">{}</ul>",
                    items
                        .iter()
                        .map(|i| format!("<li>{}</li>", i.compile(theme)))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }

            Self::Rectangle(contents, border_radius, color) => {
                format!(
                    "<div style=\"height:100%; border-radius: {}%; background-color:{}\">{}</div>",
                    border_radius,
                    theme.get_color_hex(*color),
                    contents.compile(theme)
                )
            }

            Self::Title(title) => format!("<h1>{}</h1>", title),
            Self::SectionTitle(title) => format!("<h4>{}</h4>", title),
            Self::Section(contents) => theme.compile_section_html(contents.compile(theme)),
            Self::Text(text) => format!("<p>{}</p>", text),

            Self::Italics(x) => format!("<i>{}</i>", x.compile(theme)),
            Self::Bold(x) => format!("<b>{}</b>", x.compile(theme)),
            Self::Link(content, link) => {
                format!("<a href=\"{}\">{}</a>", link, content.compile(theme))
            }

            Self::ColoredForeground(contents, color) => {
                format!(
                    "<div style=\"color: {};\">{}</div>",
                    theme.get_color_hex(*color),
                    contents.compile(theme)
                )
            }

            Self::ColoredBackground(contents, color) => {
                format!(
                    "<div style=\"background-color: {};\">{}</div>",
                    theme.get_color_hex(*color),
                    contents.compile(theme)
                )
            }

            Self::FadeIn(_) => unimplemented!(),
        }
    }
}
