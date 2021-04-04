extern crate resume;
use resume::{Resume, Degree, ContactInfo, Work, Education, Proficiency, HTML, Theme, Color};
use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Column,
    Container, Element, HorizontalAlignment, Image, Length, Radio, Row,
    Sandbox, Scrollable, Settings, Slider, Space, Text, TextInput,
};
use std::cmp::{min, max};

pub struct DefaultTheme;
impl Theme for DefaultTheme {
    fn get_color_rgb(&self, color: Color) -> (u8, u8, u8) {
        match color {
            Color::DefaultForeground | Color::Black => (0, 0, 0),
            Color::DefaultBackground | Color::White => (255, 255, 255),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Orange => (255, 165, 0),
            Color::Pink => (255, 192, 203),
            Color::Grey => (128, 128, 128),
            Color::Violet => (127, 0, 255),
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
            Color::Blue => (139, 233, 253),
            Color::Violet => (189, 147, 249),
            
            Color::Grey => (68, 71, 90),
            Color::Pink => (255, 121, 198),
            Color::Brown => (165, 42, 42),
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
fn button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
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
            other => self.steps.update(other)
        }
    }

    fn view(&mut self) -> Element<Message> {
        let copy = self.clone();
        let Self { steps, scroll, next_button, back_button, .. } = self;

        let mut controls = Row::new();

        if steps.has_last() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary)
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

        let scrollable = Scrollable::new(scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

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
    DegreeChanged(Degree),
    ContactsChanged(ContactInfo),
    WorkChanged(Work),
    AddEducation,
    AddWork,
    ProficiencySelected(Proficiency),
    AddSkill,
}

#[derive(Debug, Clone)]
struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Self {
        Self { steps: vec![
            Step::Welcome,
            Step::Name {
                first_name: String::new(),
                last_name: String::new(),

                first_name_state: text_input::State::new(),
                last_name_state: text_input::State::new(),
            },
            Step::Profession { text: String::new(), state: text_input::State::new() },
            Step::Description { text: String::new(), state: text_input::State::new() },
            Step::Skills {
                skills: Vec::new(),
                text: String::new(),
                selection: None,
                text_state: text_input::State::new(),
                button_state: button::State::new()
            },
            Step::End
        ], current: 0 }
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

    fn has_last(&self) -> bool { self.current > 0 }
    fn has_next(&self) -> bool { self.current < self.steps.len() - 1 }

    fn can_continue(&self) -> bool {
        self.has_next() && self.steps[self.current].can_continue()
    }

    fn view(&mut self, app: &App)  -> Element<Message> {
        self.steps[self.current].view(app)
    }

    fn update(&mut self, msg: Message) {
        self.steps[self.current].update(msg)
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
        skills: Vec<(String, Proficiency)>,
        selection: Option<Proficiency>,
        text: String,
        text_state: text_input::State,
        button_state: button::State
    },

    Education {
        education: Education,
        field_state: text_input::State,
        start_year_state: text_input::State,
        end_year_state: text_input::State,
        school_state: text_input::State,
    },

    AddEducation(button::State),

    Work {
        work: Work,
        start_year_state: text_input::State,
        end_year_state: text_input::State,
        position_state: text_input::State,
        company_state: text_input::State,
        description_state: text_input::State,
    },

    AddWork(button::State),

    End
}


impl<'a> Step {
    fn update(&mut self, msg: Message) {
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
            Message::StartYearChanged(year) => {
                let n = year.parse::<u32>().unwrap();
                match self {
                    Self::Work { work, .. } => {
                        work.start_year = n;
                    }
                    Self::Education { education, .. } => {
                        education.start_year = n;
                    }
                    _ => unreachable!()
                }
            }
            Message::EndYearChanged(year) => {
                let n = year.parse::<u32>().unwrap();
                match self {
                    Self::Work { work, .. } => {
                        work.end_year = n;
                    }
                    Self::Education { education, .. } => {
                        education.end_year = n;
                    }
                    _ => unreachable!()
                }
            }
            Message::PositionChanged(title) => {
                if let Self::Work { work, .. } = self {
                    work.position = title;
                }
            }
            Message::CompanyChanged(company) => {
                if let Self::Work { work, .. } = self {
                    work.company = company;
                }
            }

            Message::DescriptionChanged(description) => {
                match self {
                    Self::Description { text, .. } | Self::Skills { text, .. } | Self::Profession { text, .. } => *text = description,
                    Self::Work { work, .. } => work.description = description,
                    _ => unreachable!()
                }
            }
            Message::DegreeChanged(degree) => {
                if let Self::Education { education, .. } = self {
                    education.degree = degree;
                }
            }
            Message::ContactsChanged(contacts) => {
                if let Self::ContactInfo { info, .. } = self {
                    *info = contacts;
                }
            }
            Message::WorkChanged(w) => {
                if let Self::Work { work, .. } = self {
                    *work = w;
                }
            }
            Message::ProficiencySelected(prof) => {
                if let Self::Skills { selection, .. } = self {
                    *selection = Some(prof);
                }
            }
            Message::AddSkill => {
                if let Self::Skills { skills, selection, text, text_state, .. } = self {
                    skills.push((text.clone(), selection.unwrap()));
                    *selection = None;
                    *text = String::new();
                    *text_state = text_input::State::new();
                }
            }
            Message::NextPressed | Message::BackPressed | Message::AddEducation | Message::AddWork => {}
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
            Self::Education { .. } | Self::AddEducation { .. } => "Education",
            Self::Work { .. } | Self::AddWork { .. } => "Education",
            Self::End => "End"
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Self::Welcome | Self::ContactInfo { .. } | Self::Skills { .. } | Self::AddEducation { .. } | Self::AddWork { .. } => true,
            Self::Name { first_name, last_name, .. } => !first_name.is_empty() && !last_name.is_empty(),
            Self::Profession { text, .. } | Self::Description { text, .. } => !text.is_empty(),
            Self::Education { education, .. } => true,
            Self::Work { work, .. } => true,
            Self::End => false,
        }
    }

    fn view(&mut self, app: &App) -> Element<Message> {
        match self {
            Self::Welcome => Self::welcome(),
            Self::End => Self::end(app),
            Self::Name {
                first_name,
                last_name,
                first_name_state,
                last_name_state
            } => Self::name(first_name, first_name_state, last_name, last_name_state),
            Self::Profession { text, state } => Self::profession(text, state),
            Self::Description { text, state } => Self::description(text, state),
            Self::Skills { skills, text, selection, text_state, button_state } => Self::skills(
                &skills,
                text,
                *selection,
                text_state,
                button_state
            ),
            _ => unreachable!()
        }.into()
    }

    fn container(title: &str) -> Column<'a, Message> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn end(app: &App) -> Column<'a, Message> {
        Self::container("End")
            .push(Text::new(
                "Done!",
            ))
    }

    fn skills(
        skills: &Vec<(String, Proficiency)>,
        text: &str,
        selection: Option<Proficiency>,
        text_state: &'a mut text_input::State,
        button_state: &'a mut button::State
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


        let proficiency_input = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("What's your proficiency?").size(24))
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


        Self::container("Skills")
            .push(Text::new(
                "Add some skills to your resume",
            ))
            .push(skills.iter().cloned().fold(
                Column::new(),
                |items, (text, proficiency)| {
                    items.push(Text::new(format!("\"{}\" => {}", text, proficiency)))
                },
            ))
            .push(text_input)
            .push(proficiency_input)
            .push(
                button(button_state, "Add Skill")
                    .on_press(Message::AddSkill)
                    .style(style::Button::Primary))
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
            .push(Text::new(
                "What is your profession?",
            ))
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
            .push(Text::new(
                "Talk about yourself",
            ))
            .push(text_input)
    }

    fn name(first_name: &str, first_name_state: &'a mut text_input::State, last_name: &str, last_name_state: &'a mut text_input::State) -> Column<'a, Message> {
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
            .push(Text::new(
                "Enter your first name",
            ))
            .push(first_name_text_input)
            .push(Text::new(
                "Enter your last name",
            ))
            .push(last_name_text_input)
            .push(
                Text::new(if first_name.is_empty() || last_name.is_empty() {
                    String::from("You have not typed anything yet...")
                } else {
                    (first_name.to_string() + " ") + &last_name
                })
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn welcome() -> Column<'a, Message> {
        Self::container("Welcome")
            .push(Text::new(
                "This is a simple tour meant to showcase a bunch of widgets \
                 that can be easily implemented on top of Iced.",
            ))
            .push(Text::new(
                "Iced is a cross-platform GUI library for Rust focused on \
                 simplicity and type-safety. It is heavily inspired by Elm.",
            ))
            .push(Text::new(
                "It was originally born as part of Coffee, an opinionated \
                 2D game engine for Rust.",
            ))
            .push(Text::new(
                "On native platforms, Iced provides by default a renderer \
                 built on top of wgpu, a graphics library supporting Vulkan, \
                 Metal, DX11, and DX12.",
            ))
            .push(Text::new(
                "Additionally, this tour can also run on WebAssembly thanks \
                 to dodrio, an experimental VDOM library for Rust.",
            ))
            .push(Text::new(
                "You will need to interact with the UI in order to reach the \
                 end!",
            ))
    }
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
                    degree: Degree::HighSchoolDiploma,
                    field: None,
                    start_year: 2016,
                    end_year: 2020,

                    school: "South Doyle High School".to_string()
                },
                Education {
                    degree: Degree::None,
                    field: Some("Computer Science".to_string()),
                    start_year: 2019,
                    end_year: 2020,

                    school: "Pellissippi State Community College".to_string()
                },
                Education {
                    degree: Degree::None,
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

                    position: "Lead Programmer for FIRST Robotics Team".to_string(),
                    company: "South Doyle High School".to_string(),

                    description: r#"Wrote robot code for four 120 pound robots using Python and WPIlib. The robos used various CAN-bus enabled devices, such as ultrasonic sensors, encoders, and pressure sensors. The robot code used data from these sensors to autonomously move and perform certain tasks."#.to_string()
                },
                Work {
                    start_year: 2018,
                    end_year: 2019,

                    position: "Official University Software Vendor".to_string(),
                    company: "University of Tennessee, Knoxville".to_string(),

                    description: r#"Helped develop Simulated Electronic Fetal Monitoring app in JavaScript, and rewrote the application in Dart to run natively on mobile and desktop."#.to_string()
                },
                Work {
                    start_year: 2019,
                    end_year: 2019,

                    position: "Software Developer Intern".to_string(),
                    company: "Oak Ridge National Laboratory".to_string(),

                    description: r#"Developed Rusty-CI, a general purpose GitHub and GitLab continuous integration tool, and multiple components of ASGarD (Adaptive Sparse Grid Discretization), a partial differential equation solver designed to run on exascale architectures."#.to_string()
                },
            ]
        }.generate().compile(&DraculaTheme)
    );


    App::run(Settings::default());
}
