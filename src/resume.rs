use super::{HTML, Color, HorizontalAlignment, VerticalAlignment};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Work {
    pub start_year: u32,
    pub end_year: u32,

    pub position: String,
    pub company: String,

    pub description: String
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Degree {
    AssociatesOf(String),
    BachelorsOf(String),
    MastersOf(String),
    PhDOf(String),
    HighSchoolDiploma,

    Other(String),
}

impl ToString for Degree {
    fn to_string(&self) -> String {
        match self {
            Self::AssociatesOf(x) => format!("Associates degree of {}", x),
            Self::BachelorsOf(x) => format!("Bachelors degree of {}", x),
            Self::MastersOf(x) => format!("Masters degree of {}", x),
            Self::PhDOf(x) => format!("PhD of {}", x),
            Self::HighSchoolDiploma => String::from("high school diploma"),
            Self::Other(x) => format!("{}", x),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Education {
    pub start_year: u32,
    pub end_year: u32,

    pub school: String,
    pub degree: Option<Degree>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Proficiency {
    None,
    Barely,
    Some,
    Strong,
    Expert,
}

impl ToString for Proficiency {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("None"),
            Self::Barely => String::from("Barely"),
            Self::Some => String::from("Some"),
            Self::Strong => String::from("Strong"),
            Self::Expert => String::from("Expert"),
        }
    }
}

impl From<Proficiency> for u32 {
    fn from(level: Proficiency) -> Self {
        match level {
            Proficiency::None   => 0,
            Proficiency::Barely => 1,
            Proficiency::Some   => 2,
            Proficiency::Strong => 3,
            Proficiency::Expert => 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContactInfo {
    pub email:    Option<String>,
    pub phone:    Option<String>,
    pub website:  Option<String>,
    pub github:   Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resume {
    pub first_name: String,
    pub last_name: String,

    pub description: String,
    pub contact_info: ContactInfo,

    pub profession: String,
    pub skills: Vec<(String, Option<Proficiency>)>,
    pub education: Vec<Education>,
    pub work_experience: Vec<Work>
}

impl Resume {
    pub fn generate(&self) -> HTML {
        HTML::html(vec![
            HTML::container(vec![
                HTML::col(vec![
                    HTML::row(vec![
                        HTML::section(HTML::col(vec![
                            HTML::aligned(
                                HTML::fg(HTML::title(format!("{} {}", self.first_name, self.last_name)), Color::Violet),
                                HorizontalAlignment::Center,
                                VerticalAlignment::SameAsParent
                            ),
                            HTML::aligned(
                                HTML::fg(HTML::italics(HTML::section_title(&self.profession)), Color::Pink),
                                HorizontalAlignment::Center,
                                VerticalAlignment::SameAsParent
                            ),
                            // HTML::fg(HTML::text(&self.description), Color::Green),
                            HTML::text(&self.description),
                        ])),
                        HTML::section(HTML::col(vec![
                            HTML::aligned(
                                HTML::fg(HTML::italics(HTML::section_title("Skills")), Color::Pink),
                                HorizontalAlignment::Center,
                                VerticalAlignment::SameAsParent
                            ),
                            HTML::aligned(HTML::ul(self.skills.iter().map(|(skill, level)| {
                                if let Some(level) = level {
                                    HTML::row(vec![HTML::text(skill), HTML::PercentBar(u32::from(*level) * 100 / u32::from(Proficiency::Expert), level.to_string())])
                                } else {
                                    HTML::text(skill)
                                }
                            }).collect::<Vec<HTML>>()), HorizontalAlignment::Center,
                            VerticalAlignment::SameAsParent)
                        ])),
                        // HTML::section(HTML::aligned(HTML::text("hello world!"), HorizontalAlignment::Center, VerticalAlignment::Center)),
                    ]),
                    HTML::section(HTML::col(vec![
                        HTML::aligned(HTML::fg(HTML::section_title("Education"), Color::Blue), HorizontalAlignment::Center, VerticalAlignment::SameAsParent),
                        HTML::ol(self.education.iter().map(|e| {
                            if let Some(degree) = &e.degree {
                                HTML::text(format!("Attended <b>{}</b> from <i>{}</i> to <i>{}</i> and acheived {}", e.school, e.start_year, e.end_year, degree.to_string()))
                            } else {
                                HTML::text(format!("Attended <b>{}</b> from <i>{}</i> to <i>{}</i>", e.school, e.start_year, e.end_year))
                            }
                        }).collect::<Vec<HTML>>()),
                        HTML::aligned(HTML::fg(HTML::section_title("Professional Experience"), Color::Blue), HorizontalAlignment::Center, VerticalAlignment::SameAsParent),
                        HTML::ul(self.work_experience.iter().map(|j| {
                            HTML::text(format!("<b>{}</b> at {} from <i>{}</i> to <i>{}</i>", j.position, j.company, j.start_year, j.end_year))
                        }).collect::<Vec<HTML>>())
                    ])),
                ])
            ])
        ])
    }
}
