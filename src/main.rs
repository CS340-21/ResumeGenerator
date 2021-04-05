extern crate resume;
use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Column, Container, Element,
    HorizontalAlignment, Image, Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space,
    Text, TextInput,
};
use resume::{Color, ContactInfo, Degree, Education, Proficiency, Resume, Theme, Work, HTML};
use std::{
    cmp::{max, min},
    fs::write,
};

pub struct DefaultTheme;
impl Theme for DefaultTheme {
    fn get_color_rgb(&self, color: Color) -> (u8, u8, u8) {
        match color {
            Color::DefaultForeground | Color::Black => (0, 0, 0),
            Color::DefaultBackground | Color::White => (255, 255, 255),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::DefaultSectionTitle | Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Orange => (255, 165, 0),
            Color::Pink => (255, 192, 203),
            Color::DefaultSubtitle => (255, 0, 255),
            Color::Grey => (128, 128, 128),
            Color::DefaultTitle | Color::Violet => (127, 0, 255),
            Color::Brown => (165, 42, 42),
        }
    }
}

pub struct DraculaTheme;
impl Theme for DraculaTheme {
    fn get_color_rgb(&self, color: Color) -> (u8, u8, u8) {
        match color {
            Color::Black | Color::DefaultBackground => (40, 42, 54),
            Color::White | Color::DefaultForeground => (248, 248, 242),
            Color::Red => (255, 85, 85),
            Color::Orange => (255, 184, 108),
            Color::Yellow => (241, 250, 140),
            Color::Green => (80, 250, 123),
            Color::DefaultSectionTitle | Color::Blue => (139, 233, 253),
            Color::DefaultTitle | Color::Violet => (189, 147, 249),

            Color::Grey => (68, 71, 90),
            Color::DefaultSubtitle | Color::Pink => (255, 121, 198),
            Color::Brown => (165, 42, 42),
        }
    }
}

pub struct ForestTheme;
impl Theme for ForestTheme {
    fn get_color_rgb(&self, color: Color) -> (u8, u8, u8) {
        match color {
            Color::Black => (40, 42, 54),
            Color::White | Color::DefaultForeground => (237, 245, 225), // #edf5e1
            Color::Red => (255, 85, 85),
            Color::Orange => (255, 184, 108),
            Color::Yellow => (241, 250, 140),
            Color::Green => (80, 250, 123),
            Color::DefaultSectionTitle => (55, 150, 131), // #379683

            Color::Blue | Color::DefaultBackground => (5, 56, 107), // #05386b
            Color::DefaultTitle => (142, 228, 175),                 // #8ee4af
            Color::Violet => (189, 147, 249),
            Color::Grey => (68, 71, 90),
            Color::DefaultSubtitle => (219, 92, 162), // #5cdb95
            Color::Pink => (255, 121, 198),
            Color::Brown => (165, 42, 42),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThemeOption {
    Default,
    Dracula,
    Forest,
}

impl From<ThemeOption> for String {
    fn from(t: ThemeOption) -> Self {
        format!("{:?}", t).replace("ThemeOption::", "")
    }
}

impl ThemeOption {
    fn all() -> [Self; 3] {
        [Self::Default, Self::Dracula, Self::Forest]
    }

    fn get_theme(&self) -> &dyn Theme {
        match self {
            Self::Default => &DefaultTheme,
            Self::Dracula => &DraculaTheme,
            Self::Forest => &ForestTheme,
        }
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

#[derive(Clone, Debug)]
pub struct App {
    steps: Steps,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            steps: Steps::new(),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        format!("{} - Resume Generator", self.steps.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::NextPressed => self.steps.advance(),
            Message::BackPressed => self.steps.go_back(),
            other => self.steps.update(other),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let copy = self.clone();
        let Self {
            steps,
            scroll,
            next_button,
            back_button,
            ..
        } = self;

        let mut controls = Row::new();

        if steps.has_last() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button(next_button, "Next")
                    .on_press(Message::NextPressed)
                    .style(style::Button::Primary),
            );
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(steps.view(&copy))
            .push(controls)
            .into();

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,

    FirstNameChanged(String),
    LastNameChanged(String),
    StartYearChanged(String),
    EndYearChanged(String),
    PositionChanged(String),
    CompanyChanged(String),
    DescriptionChanged(String),
    DegreeSelected(Degree),
    ContactsChanged(ContactInfo),
    ProficiencySelected(Proficiency),
    SaveFileChanged(String),

    ThemeSelected(ThemeOption),

    AddEducation,
    AddWork,
    AddSkill,
    SaveFile,
    Clear,
}

#[derive(Debug, Clone)]
struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Self {
        Self {
            steps: vec![
                Step::Welcome,
                Step::Name {
                    first_name: String::new(),
                    last_name: String::new(),

                    first_name_state: text_input::State::new(),
                    last_name_state: text_input::State::new(),
                },
                Step::Profession {
                    text: String::new(),
                    state: text_input::State::new(),
                },
                Step::Description {
                    text: String::new(),
                    state: text_input::State::new(),
                },
                Step::Skills {
                    skills: Vec::new(),
                    text: String::new(),
                    selection: None,
                    text_state: text_input::State::new(),
                    button_state: button::State::new(),
                    clear_state: button::State::new(),
                },
                Step::Education {
                    education_history: Vec::new(),

                    degree: None,

                    field: String::new(),
                    field_state: text_input::State::new(),
                    start_year: String::new(),
                    start_year_state: text_input::State::new(),
                    end_year: String::new(),
                    end_year_state: text_input::State::new(),
                    school: String::new(),
                    school_state: text_input::State::new(),

                    button_state: button::State::new(),
                    clear_state: button::State::new(),
                },
                Step::Work {
                    work_history: Vec::new(),

                    company: String::new(),
                    company_state: text_input::State::new(),
                    position: String::new(),
                    position_state: text_input::State::new(),
                    description: String::new(),
                    description_state: text_input::State::new(),
                    start_year: String::new(),
                    start_year_state: text_input::State::new(),
                    end_year: String::new(),
                    end_year_state: text_input::State::new(),

                    button_state: button::State::new(),
                    clear_state: button::State::new(),
                },
                Step::End {
                    theme: ThemeOption::Default,
                    save_file: String::from("resume.html"),
                    save_file_state: text_input::State::new(),
                    button_state: button::State::new(),
                },
            ],
            current: 0,
        }
    }

    fn to_resume(&self) -> String {
        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut profession = String::new();
        let mut description = String::new();
        let mut contact_info = ContactInfo {
            email: None,
            phone: None,
            github: None,
            website: None,
            linkedin: None,
        };
        let mut skills = Vec::new();
        let mut work_experience = Vec::new();
        let mut education = Vec::new();
        let mut theme = ThemeOption::Default;

        for step in &self.steps {
            match step {
                Step::Name {
                    first_name: f,
                    last_name: l,
                    ..
                } => {
                    first_name = f.clone();
                    last_name = l.clone();
                }
                Step::Profession { text, .. } => {
                    profession = text.clone();
                }
                Step::Description { text, .. } => {
                    description = text.clone();
                }
                Step::Skills { skills: s, .. } => {
                    skills = s.clone();
                }
                Step::Work { work_history, .. } => {
                    work_experience = work_history.clone();
                }
                Step::Education {
                    education_history, ..
                } => {
                    education = education_history.clone();
                }
                Step::End { theme: t, .. } => theme = *t,
                _ => {}
            }
        }

        Resume {
            first_name,
            last_name,
            profession,
            description,
            contact_info,
            skills,
            work_experience,
            education,
        }
        .generate()
        .compile(theme.get_theme())
    }

    fn title(&self) -> String {
        self.steps[self.current].title().to_string()
    }

    fn advance(&mut self) {
        self.current += 1;
        self.current = min(self.current, self.steps.len() - 1);
    }

    fn go_back(&mut self) {
        self.current -= 1;
        self.current = max(self.current, 0);
    }

    fn has_last(&self) -> bool {
        self.current > 0
    }
    fn has_next(&self) -> bool {
        self.current < self.steps.len() - 1
    }

    fn can_continue(&self) -> bool {
        self.has_next() && self.steps[self.current].can_continue()
    }

    fn view(&mut self, app: &App) -> Element<Message> {
        self.steps[self.current].view(app)
    }

    fn update(&mut self, msg: Message) {
        let copy = self.clone();
        self.steps[self.current].update(&copy, msg)
    }
}

#[derive(Debug, Clone)]
enum Step {
    Welcome,

    Name {
        first_name: String,
        last_name: String,

        first_name_state: text_input::State,
        last_name_state: text_input::State,
    },

    Profession {
        text: String,
        state: text_input::State,
    },

    ContactInfo {
        info: ContactInfo,

        email_state: text_input::State,
        phone_state: text_input::State,
        website_state: text_input::State,
        github_state: text_input::State,
        linkedin_state: text_input::State,
    },

    Description {
        text: String,
        state: text_input::State,
    },

    Skills {
        skills: Vec<(String, Option<Proficiency>)>,
        selection: Option<Proficiency>,
        text: String,
        text_state: text_input::State,
        button_state: button::State,
        clear_state: button::State,
    },

    Education {
        education_history: Vec<Education>,

        degree: Option<Degree>,

        field: String,
        field_state: text_input::State,
        start_year: String,
        start_year_state: text_input::State,
        end_year: String,
        end_year_state: text_input::State,
        school: String,
        school_state: text_input::State,

        button_state: button::State,
        clear_state: button::State,
    },

    Work {
        work_history: Vec<Work>,

        start_year: String,
        start_year_state: text_input::State,
        end_year: String,
        end_year_state: text_input::State,

        position: String,
        position_state: text_input::State,
        company: String,
        company_state: text_input::State,
        description: String,
        description_state: text_input::State,

        button_state: button::State,
        clear_state: button::State,
    },

    End {
        theme: ThemeOption,
        save_file: String,
        save_file_state: text_input::State,
        button_state: button::State,
    },
}

impl<'a> Step {
    fn update(&mut self, steps: &Steps, msg: Message) {
        match msg {
            Message::FirstNameChanged(name) => {
                if let Self::Name { first_name, .. } = self {
                    *first_name = name;
                }
            }
            Message::LastNameChanged(name) => {
                if let Self::Name { last_name, .. } = self {
                    *last_name = name;
                }
            }
            Message::StartYearChanged(year) => match self {
                Self::Work { start_year, .. } | Self::Education { start_year, .. } => {
                    *start_year = year;
                }
                _ => unreachable!(),
            },
            Message::EndYearChanged(year) => match self {
                Self::Work { end_year, .. } | Self::Education { end_year, .. } => {
                    *end_year = year;
                }
                _ => unreachable!(),
            },
            Message::PositionChanged(title) => match self {
                Self::Work { position, .. } => *position = title,
                Self::Education { field, .. } => *field = title,
                _ => unreachable!(),
            },
            Message::CompanyChanged(name) => match self {
                Self::Work { company, .. } => *company = name,
                Self::Education { school, .. } => *school = name,
                _ => unreachable!(),
            },

            Message::DescriptionChanged(desc) => match self {
                Self::Description { text, .. }
                | Self::Skills { text, .. }
                | Self::Profession { text, .. } => *text = desc,
                Self::Work { description, .. } => *description = desc,
                _ => unreachable!(),
            },
            Message::DegreeSelected(d) => {
                if let Self::Education { degree, .. } = self {
                    *degree = Some(d);
                }
            }
            Message::ContactsChanged(contacts) => {
                if let Self::ContactInfo { info, .. } = self {
                    *info = contacts;
                }
            }
            Message::ProficiencySelected(prof) => {
                if let Self::Skills { selection, .. } = self {
                    *selection = Some(prof);
                }
            }
            Message::AddSkill => {
                if let Self::Skills {
                    skills,
                    selection,
                    text,
                    text_state,
                    ..
                } = self
                {
                    skills.push((text.clone(), *selection));
                    *selection = None;
                    *text = String::new();
                    *text_state = text_input::State::new();
                }
            }
            Message::ThemeSelected(t) => {
                if let Self::End { theme, .. } = self {
                    *theme = t;
                }
            }
            Message::SaveFileChanged(name) => {
                if let Self::End { save_file, .. } = self {
                    *save_file = name;
                }
            }
            Message::SaveFile => {
                if let Self::End { save_file, .. } = self {
                    write(&save_file, steps.to_resume()).unwrap();
                }
            }
            Message::AddEducation => {
                if let Self::Education {
                    education_history,
                    school,
                    school_state,
                    field,
                    field_state,
                    degree,
                    start_year,
                    start_year_state,
                    end_year,
                    end_year_state,
                    ..
                } = self
                {
                    education_history.push(Education {
                        school: school.clone(),
                        field: if field.is_empty() {
                            None
                        } else {
                            Some(field.clone())
                        },
                        degree: degree.clone(),
                        start_year: start_year.parse::<u32>().unwrap(),
                        end_year: end_year.parse::<u32>().unwrap(),
                    });
                    *school = String::new();
                    *field = String::new();
                    *degree = None;
                    *start_year = String::new();
                    *end_year = String::new();

                    *school_state = text_input::State::new();
                    *field_state = text_input::State::new();
                    *start_year_state = text_input::State::new();
                    *end_year_state = text_input::State::new();
                }
            }
            Message::AddWork => {
                if let Self::Work {
                    work_history,
                    company,
                    company_state,
                    position,
                    position_state,
                    description,
                    description_state,
                    start_year,
                    start_year_state,
                    end_year,
                    end_year_state,
                    ..
                } = self
                {
                    work_history.push(Work {
                        company: company.clone(),
                        position: position.clone(),
                        description: description.clone(),
                        start_year: start_year.parse::<u32>().unwrap(),
                        end_year: end_year.parse::<u32>().unwrap(),
                    });
                    *company = String::new();
                    *position = String::new();
                    *description = String::new();
                    *start_year = String::new();
                    *end_year = String::new();

                    *company_state = text_input::State::new();
                    *position_state = text_input::State::new();
                    *description_state = text_input::State::new();
                    *start_year_state = text_input::State::new();
                    *end_year_state = text_input::State::new();
                }
            }
            Message::Clear => match self {
                Self::Skills {
                    skills,
                    selection,
                    text,
                    text_state,
                    ..
                } => {
                    *skills = Vec::new();
                    *selection = None;
                    *text = String::new();
                    *text_state = text_input::State::new();
                }
                Self::Education {
                    education_history,
                    school,
                    school_state,
                    field,
                    field_state,
                    degree,
                    start_year,
                    start_year_state,
                    end_year,
                    end_year_state,
                    ..
                } => {
                    *education_history = Vec::new();
                    *school = String::new();
                    *field = String::new();
                    *degree = None;
                    *start_year = String::new();
                    *end_year = String::new();

                    *school_state = text_input::State::new();
                    *field_state = text_input::State::new();
                    *start_year_state = text_input::State::new();
                    *end_year_state = text_input::State::new();
                }
                Self::Work {
                    work_history,
                    company,
                    company_state,
                    position,
                    position_state,
                    description,
                    description_state,
                    start_year,
                    start_year_state,
                    end_year,
                    end_year_state,
                    ..
                } => {
                    *work_history = Vec::new();
                    *company = String::new();
                    *position = String::new();
                    *description = String::new();
                    *start_year = String::new();
                    *end_year = String::new();

                    *company_state = text_input::State::new();
                    *position_state = text_input::State::new();
                    *description_state = text_input::State::new();
                    *start_year_state = text_input::State::new();
                    *end_year_state = text_input::State::new();
                }
                _ => unreachable!(),
            },
            Message::NextPressed | Message::BackPressed => {}
        }
    }

    fn title(&self) -> &str {
        match self {
            Self::Welcome => "Welcome",
            Self::Name { .. } => "Name",
            Self::Profession { .. } => "Profession",
            Self::ContactInfo { .. } => "Contact Information",
            Self::Description { .. } => "Description",
            Self::Skills { .. } => "Skills",
            Self::Education { .. } => "Education",
            Self::Work { .. } => "Work",
            Self::End { .. } => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Self::Welcome | Self::ContactInfo { .. } | Self::Skills { .. } => true,
            Self::Name {
                first_name,
                last_name,
                ..
            } => !first_name.is_empty() && !last_name.is_empty(),
            Self::Profession { text, .. } | Self::Description { text, .. } => !text.is_empty(),
            Self::Education { .. } => true,
            Self::Work { .. } => true,
            Self::End { .. } => false,
        }
    }

    fn view(&mut self, app: &App) -> Element<Message> {
        match self {
            Self::Welcome => Self::welcome(),
            Self::End {
                theme,
                save_file,
                save_file_state,
                button_state,
            } => Self::end(Some(*theme), save_file, save_file_state, button_state, app),
            Self::Name {
                first_name,
                last_name,
                first_name_state,
                last_name_state,
            } => Self::name(first_name, first_name_state, last_name, last_name_state),
            Self::Profession { text, state } => Self::profession(text, state),
            Self::Description { text, state } => Self::description(text, state),
            Self::Skills {
                skills,
                text,
                selection,
                text_state,
                button_state,
                clear_state,
            } => Self::skills(
                &skills,
                text,
                *selection,
                text_state,
                button_state,
                clear_state,
            ),
            Self::Education {
                education_history,

                degree,

                field,
                field_state,
                start_year,
                start_year_state,
                end_year,
                end_year_state,
                school,
                school_state,

                button_state,
                clear_state,
            } => Self::education(
                education_history,
                *degree,
                field,
                field_state,
                start_year,
                start_year_state,
                end_year,
                end_year_state,
                school,
                school_state,
                button_state,
                clear_state,
            ),
            Self::Work {
                work_history,
                company,
                company_state,
                position,
                position_state,
                description,
                description_state,
                start_year,
                start_year_state,
                end_year,
                end_year_state,
                button_state,
                clear_state,
            } => Self::work(
                work_history,
                company,
                company_state,
                position,
                position_state,
                description,
                description_state,
                start_year,
                start_year_state,
                end_year,
                end_year_state,
                button_state,
                clear_state,
            ),
            _ => unreachable!(),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, Message> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn end(
        selection: Option<ThemeOption>,
        save_file: &str,
        save_file_state: &'a mut text_input::State,
        button_state: &'a mut button::State,
        app: &App,
    ) -> Column<'a, Message> {
        let theme_input = Column::new().push(ThemeOption::all().iter().cloned().fold(
            Column::new().padding(10).spacing(20),
            |choices, option| {
                choices.push(Radio::new(
                    option,
                    option,
                    selection,
                    Message::ThemeSelected,
                ))
            },
        ));

        let text_input = TextInput::new(
            save_file_state,
            "Type something",
            save_file,
            Message::SaveFileChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        Self::container("End")
            .push(Text::new("What theme would you like your resume to have?"))
            .push(theme_input)
            .push(Text::new("Where do you want to save your resume?"))
            .push(text_input)
            .push(
                button(button_state, "Save")
                    .on_press(Message::SaveFile)
                    .style(style::Button::Primary),
            )
    }

    fn work(
        work_history: &Vec<Work>,

        company: &str,
        company_state: &'a mut text_input::State,
        position: &str,
        position_state: &'a mut text_input::State,
        description: &str,
        description_state: &'a mut text_input::State,

        start_year: &str,
        start_year_state: &'a mut text_input::State,
        end_year: &str,
        end_year_state: &'a mut text_input::State,

        button_state: &'a mut button::State,
        clear_state: &'a mut button::State,
    ) -> Column<'a, Message> {
        let company_input = TextInput::new(
            company_state,
            "Type something to continue",
            company,
            Message::CompanyChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        let position_input = TextInput::new(
            position_state,
            "Type something to continue",
            position,
            Message::PositionChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        let description_input = TextInput::new(
            description_state,
            "Type something to continue",
            description,
            Message::DescriptionChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        let years_input = Row::new()
            .padding(20)
            .spacing(10)
            .push(
                TextInput::new(
                    start_year_state,
                    "Start Year",
                    start_year,
                    Message::StartYearChanged,
                )
                .padding(10)
                .width(Length::Fill)
                .size(20),
            )
            .push(Text::new("to"))
            .push(
                TextInput::new(
                    end_year_state,
                    "End Year",
                    end_year,
                    Message::EndYearChanged,
                )
                .padding(10)
                .width(Length::Fill)
                .size(20),
            );

        let result = Self::container("Work History")
            .push(
                work_history
                    .iter()
                    .cloned()
                    .fold(Column::new(), |items, e| {
                        items.push(Text::new(format!("{} at {}", e.position, e.company)))
                    }),
            )
            .push(Text::new("Where did you work?"))
            .push(company_input)
            .push(Text::new("Between which years were you employed?"))
            .push(years_input)
            .push(Text::new("What was your position?"))
            .push(position_input)
            .push(Text::new("Describe your work"))
            .push(description_input);

        if !position.is_empty()
            && !company.is_empty()
            && !description.is_empty()
            && start_year
                .parse::<u32>()
                .and(end_year.parse::<u32>())
                .is_ok()
        {
            result.push(
                Row::new()
                    .push(
                        button(clear_state, "Clear")
                            .on_press(Message::Clear)
                            .style(style::Button::Secondary),
                    )
                    .push(Space::with_width(Length::Fill))
                    .push(
                        button(button_state, "Add Work")
                            .on_press(Message::AddWork)
                            .style(style::Button::Primary),
                    ),
            )
        } else {
            result.push(
                Row::new().push(
                    button(clear_state, "Clear")
                        .on_press(Message::Clear)
                        .style(style::Button::Secondary),
                ),
            )
        }
    }

    fn education(
        education_history: &Vec<Education>,

        degree: Option<Degree>,

        field: &str,
        field_state: &'a mut text_input::State,
        start_year: &str,
        start_year_state: &'a mut text_input::State,
        end_year: &str,
        end_year_state: &'a mut text_input::State,
        school: &str,
        school_state: &'a mut text_input::State,

        button_state: &'a mut button::State,
        clear_state: &'a mut button::State,
    ) -> Column<'a, Message> {
        let school_input = TextInput::new(
            school_state,
            "Type something to continue",
            school,
            Message::CompanyChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        let field_input = TextInput::new(
            field_state,
            "Type something to continue",
            field,
            Message::PositionChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        let years_input = Row::new()
            .padding(20)
            .spacing(10)
            .push(
                TextInput::new(
                    start_year_state,
                    "Start Year",
                    start_year,
                    Message::StartYearChanged,
                )
                .padding(10)
                .width(Length::Fill)
                .size(20),
            )
            .push(Text::new("to"))
            .push(
                TextInput::new(
                    end_year_state,
                    "End Year",
                    end_year,
                    Message::EndYearChanged,
                )
                .padding(10)
                .width(Length::Fill)
                .size(20),
            );

        let degree_input =
            Column::new()
                .spacing(10)
                .push(Degree::all().iter().cloned().fold(
                    Column::new().padding(10).spacing(20),
                    |choices, prof| {
                        choices.push(Radio::new(prof, prof, degree, Message::DegreeSelected))
                    },
                ));

        let result = Self::container("Education History")
            .push(
                education_history
                    .iter()
                    .cloned()
                    .fold(Column::new(), |items, e| {
                        items.push(Text::new(format!("Attended {}", e.school)))
                    }),
            )
            .push(Text::new("Which school did you attend?"))
            .push(school_input)
            .push(Text::new("Between which years did you attend?"))
            .push(years_input)
            .push(Text::new("What was your field of study?"))
            .push(field_input)
            .push(Text::new("What degree did you receive?"))
            .push(degree_input);

        if !school.is_empty()
            && start_year
                .parse::<u32>()
                .and(end_year.parse::<u32>())
                .is_ok()
        {
            result.push(
                Row::new()
                    .push(
                        button(clear_state, "Clear")
                            .on_press(Message::Clear)
                            .style(style::Button::Secondary),
                    )
                    .push(Space::with_width(Length::Fill))
                    .push(
                        button(button_state, "Add Education")
                            .on_press(Message::AddEducation)
                            .style(style::Button::Primary),
                    ),
            )
        } else {
            result.push(
                Row::new().push(
                    button(clear_state, "Clear")
                        .on_press(Message::Clear)
                        .style(style::Button::Secondary),
                ),
            )
        }
    }

    fn skills(
        skills: &Vec<(String, Option<Proficiency>)>,
        text: &str,
        selection: Option<Proficiency>,
        text_state: &'a mut text_input::State,
        button_state: &'a mut button::State,
        clear_state: &'a mut button::State,
    ) -> Column<'a, Message> {
        let text_input = TextInput::new(
            text_state,
            "Type something to continue",
            text,
            Message::DescriptionChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        let proficiency_input =
            Column::new()
                .spacing(10)
                .push(Proficiency::all().iter().cloned().fold(
                    Column::new().padding(10).spacing(20),
                    |choices, prof| {
                        choices.push(Radio::new(
                            prof,
                            prof,
                            selection,
                            Message::ProficiencySelected,
                        ))
                    },
                ));

        let result = Self::container("Skills")
            .push(Text::new("Add some skills to your resume"))
            .push(
                skills
                    .iter()
                    .cloned()
                    .fold(Column::new(), |items, (text, proficiency)| {
                        items.push(Text::new(format!(
                            "{}: {}",
                            text,
                            match proficiency {
                                Some(p) => p.to_string().to_lowercase(),
                                None => "none".to_string(),
                            }
                        )))
                    }),
            )
            .push(text_input)
            .push(Text::new("What's your proficiency?"))
            .push(proficiency_input);

        if !text.is_empty() {
            result.push(
                Row::new()
                    .push(
                        button(clear_state, "Clear")
                            .on_press(Message::Clear)
                            .style(style::Button::Secondary),
                    )
                    .push(Space::with_width(Length::Fill))
                    .push(
                        button(button_state, "Add Skill")
                            .on_press(Message::AddSkill)
                            .style(style::Button::Primary),
                    ),
            )
        } else {
            result.push(
                Row::new().push(
                    button(clear_state, "Clear")
                        .on_press(Message::Clear)
                        .style(style::Button::Secondary),
                ),
            )
        }
    }

    fn profession(text: &str, state: &'a mut text_input::State) -> Column<'a, Message> {
        let text_input = TextInput::new(
            state,
            "Type something to continue",
            text,
            Message::DescriptionChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        Self::container("Profession")
            .push(Text::new("What is your profession?"))
            .push(text_input)
    }

    fn description(text: &str, state: &'a mut text_input::State) -> Column<'a, Message> {
        let text_input = TextInput::new(
            state,
            "Type something to continue",
            text,
            Message::DescriptionChanged,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        Self::container("Description")
            .push(Text::new("Talk about yourself"))
            .push(text_input)
    }

    fn name(
        first_name: &str,
        first_name_state: &'a mut text_input::State,
        last_name: &str,
        last_name_state: &'a mut text_input::State,
    ) -> Column<'a, Message> {
        let first_name_text_input = TextInput::new(
            first_name_state,
            "Type something to continue",
            first_name,
            Message::FirstNameChanged,
        )
        .padding(10)
        .size(30);
        let last_name_text_input = TextInput::new(
            last_name_state,
            "Type something to continue",
            last_name,
            Message::LastNameChanged,
        )
        .padding(10)
        .size(30);

        Self::container("Your Name")
            .push(Text::new("Enter your first name"))
            .push(first_name_text_input)
            .push(Text::new("Enter your last name"))
            .push(last_name_text_input)
    }

    fn welcome() -> Column<'a, Message> {
        Self::container("Welcome")
            .push(Text::new(
                "This is a tool to help you automatically create a nice resume webpage.",
            ))
            .push(example_image())
            .push(Text::new(
                "Simply go through each page and enter your information!",
            ))
    }
}

fn example_image<'a>() -> Container<'a, Message> {
    Container::new(
        // This should go away once we unify resource loading on native
        // platforms
        if cfg!(target_arch = "wasm32") {
            Image::new("assets/dracula-example.png")
        } else {
            Image::new(format!(
                "{}/assets/dracula-example.png",
                env!("CARGO_MANIFEST_DIR")
            ))
        }
        .width(Length::Units(350)),
    )
    .width(Length::Fill)
    .center_x()
}

fn main() {
    println!("{}",
        Resume {
            first_name: "Adam".to_string(),
            last_name: "McDaniel".to_string(),

            contact_info: ContactInfo {
                email: Some("adam.mcdaniel17@gmail.com".to_string()),
                phone: None,
                website: Some("adam-mcdaniel.net".to_string()),
                github: Some("adam-mcdaniel".to_string()),
                linkedin: None,
            },

            description: r#"I'm a musician, programmer, and college student. I've been developing software in Python, Rust, C++, and many more different languages for several years now."#.to_string(),

            profession: "Software Engineer".to_string(),
            skills: vec![
                ("💽 Systems".to_string(), Some(Proficiency::Barely)),
                ("🧶 Knitting".to_string(), Some(Proficiency::None)),
                ("⚙️ Compilers".to_string(), Some(Proficiency::Some)),
                ("🧪 Code".to_string(), Some(Proficiency::Some)),
                ("📖 Academics".to_string(), Some(Proficiency::Expert)),
                ("♔ Chess".to_string(), Some(Proficiency::Strong)),
            ],
            education: vec![
                Education {
                    degree: Some(Degree::HighSchoolDiploma),
                    field: None,
                    start_year: 2016,
                    end_year: 2020,

                    school: "South Doyle High School".to_string()
                },
                Education {
                    degree: None,
                    field: Some("Computer Science".to_string()),
                    start_year: 2019,
                    end_year: 2020,

                    school: "Pellissippi State Community College".to_string()
                },
                Education {
                    degree: None,
                    field: Some("Computer Science".to_string()),
                    start_year: 2020,
                    end_year: 2021,

                    school: "University of Tennessee".to_string()
                },
            ],
            work_experience: vec![
                Work {
                    start_year: 2016,
                    end_year: 2020,

                    company: "South Doyle High School".to_string(),
                    position: "Lead Programmer for FIRST Robotics Team".to_string(),

                    description: r#"Wrote robot code for four 120 pound robots using Python and WPIlib. The robos used various CAN-bus enabled devices, such as ultrasonic sensors, encoders, and pressure sensors. The robot code used data from these sensors to autonomously move and perform certain tasks."#.to_string()
                },
                Work {
                    start_year: 2018,
                    end_year: 2019,

                    company: "University of Tennessee, Knoxville".to_string(),
                    position: "Official University Software Vendor".to_string(),
                    description: r#"Helped develop Simulated Electronic Fetal Monitoring app in JavaScript, and rewrote the application in Dart to run natively on mobile and desktop."#.to_string()
                },
                Work {
                    start_year: 2019,
                    end_year: 2019,

                    company: "Oak Ridge National Laboratory".to_string(),
                    position: "Software Developer Intern".to_string(),
                    description: r#"Developed Rusty-CI, a general purpose GitHub and GitLab continuous integration tool, and multiple components of ASGarD (Adaptive Sparse Grid Discretization), a partial differential equation solver designed to run on exascale architectures."#.to_string()
                },
            ]
        }.generate().compile(&ForestTheme)
    );

    let mut settings = Settings::default();
    settings.window.size = (580, 840);
    App::run(settings).unwrap();
}
