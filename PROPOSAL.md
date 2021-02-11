# Resume Generator Proposal

### Authors
- Adam McDaniel
- Colby Smith

## **_I_**. Introduction

Our project is a resume or CV generator that aims to runs on: the web, the desktop (including Windows, MacOS, and Debian based Linux distributions), and potentially mobile. The resume generator will prompt the user with several radio buttons, checklists, and text fields for info including: information about their previous employment, their education, a description of themselves, their contact information, their skills, and finally layout and style information about the generated resume.

The generated resume will be in HTML format for maximum portability; HTML will allow the users to easily feature their resume on a website, show their resume on any device, and print to a PDF using any browser. Additionally, HTML will allow the resume generator to plug and play with a variety of stylesheets available to the user.

Our team is very familiar with Rust, the web, and all of the technologies we plan to use for the project. Of all the components for the application, we predict that the frontend of the project, the GUI for interacting with the resume generator, will be the most difficult portion of the project. The GUI libraries we plan to use, though, are well documented and have first class support for both native desktop and web implementations. Overall, there aren't any major obstacles that stand in the way of the finished product.

## **_II_**. Customer Value

The main demographic for a resume generator program is going to be unemployeed people seeking employment. Most people in this class will be in this demographic at some point in the next two years. This demographic values
- A clean looking product
- An intuitive GUI
- Low time commitment
- Free tools

Our project will hit all of these goals. Ideally it will end up on a website allowing its users to forgo downloading anything other than their finished resume. The GUI should be easy to understand and self-explanatory. The user should be able to find our website and have their own HTML file within 10 minutes. They will just have to enter their data which gets shoved into pre-made HTML files.

## **_III_**. Technology

This project will be implemented using the Rust programming language. The reason we chose Rust is because Rust allows us to support compiling native Desktop applications in addition to WASM targets.

Our program will consist of two parts: the binary and the library API. The library API will allow us to write platform independent implementations of the actual HTML generating portion of the resume that doesn't require a runtime. Then, on top of the API, the binary application will implement a GUI that _uses_ the library API. This way, the GUI and the resume generation are independent, and multiple frontend implementations are possible. If we choose to add a GUI implementation for a new platform, we can create a new binary that implements a new GUI on top of our API.

![Flow Chart](assets/flow-chart.png)

## **_IV_**. Team

Our team is not new to writing projects like these, but one of our members has not used Rust extensively.

All members of the team will contribute to each component, but Adam will focus more on the backend, while Colby will focus more on the frontend.

## **_V_**. Project Management
Project meetings and management will be handled on Discord on a weekly basis. Here is a tentative schedule that will surely see changes over the course of the semester. We will follow the general guidelines from these dates and project goals.
| Date              | Activity                                        | Goals                                                                    |
|-------------------|-------------------------------------------------|--------------------------------------------------------------------------|
| Thursday, Feb. 18 | Begin work on project.                          | Have a single basic template resume to feed user input into              |
| Friday, Feb. 26   | Second sprint                                   | Add a second template and allow the user to pick between the two         |
| Thursday, Mar. 4  | Submit iteration 1 status report                |                                                                          |
| Friday, Mar. 12   | Third sprint                                    | Upgrade from a commandline based project to having a basic GUI           |
| Thursday, Mar. 18 | Submit iteration 2 status report                |                                                                          |
| Friday, Mar. 26   | Fourth sprint                                   | Improve upon the GUI and make the whole project cleaner and more robust  |
| Thursday, Apr. 1  | Submit iteration 3 status report                |                                                                          |
| Friday, Apr. 9    | Final sprint                                    | Fix bugs, clean up project. add any more ideas we think of along the way |
| Thursday, Apr. 15 | Submit project report and present final product |                                                                          |

This schedule should see the project get finished by the end of the semester. We are not interested in making a profit from this project, just making it public would be ideal. As long as we have a working project by the end of the semester, this schedule will have worked.
## References

Formal resume definition and expectations outlined [here](https://en.wikipedia.org/wiki/R%C3%A9sum%C3%A9 "Resume Wikipedia")  
Rust [homepage](https://www.rust-lang.org/ "Rust"), [wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))  
[Iced GUI for rust](https://news.ycombinator.com/item?id=22766639 "ICED")  

