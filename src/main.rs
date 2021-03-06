extern crate resume;
use resume::{Resume, Degree, Work, Education, Proficiency, HTML, Theme, Color};


pub struct DefaultTheme;
impl Theme for DefaultTheme {
    fn get_color_rgb(&self, color: Color) -> (u8, u8, u8) {
        match color {
            Color::Black => (0, 0, 0),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Orange => (255, 165, 0),
            Color::Pink => (255, 192, 203),
            Color::Grey => (128, 128, 128),
            Color::Violet => (127, 0, 255),
            Color::White => (255, 255, 255),
            Color::Brown => (165, 42, 42),
        }
    }
}


fn main() {
    println!("{}",
        Resume {
            first_name: "Adam".to_string(),
            last_name: "McDaniel".to_string(),

            description: r#"I'm a musician, programmer, and college student. I've been developing software in Python, Rust, C++, and many more different languages for several years now."#.to_string(),

            profession: "Software Engineer".to_string(),
            skills: vec![
                ("💽 Systems Programming".to_string(), Some(Proficiency::Barely)),
                ("🧶 Knitting".to_string(), Some(Proficiency::None)),
                ("⚙️ Compiler Design".to_string(), Some(Proficiency::Some)),
                ("🧪 Continuous Integration".to_string(), Some(Proficiency::Some)),
                ("📖 Making Terrible Grades".to_string(), Some(Proficiency::Expert)),
                ("♔ Losing at Chess".to_string(), Some(Proficiency::Strong)),
            ],
            education: vec![
                Education {
                    degree: Some(Degree::HighSchoolDiploma),
                    start_year: 2016,
                    end_year: 2020,

                    school: "South Doyle High School".to_string()
                },
                Education {
                    degree: None,
                    start_year: 2019,
                    end_year: 2020,

                    school: "Pellissippi State Community College".to_string()
                },
                Education {
                    degree: None,
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
        }.generate().compile(&DefaultTheme)
    );
}
