# Final Report
### Team Name
- ResumeGenerator
### Authors
- Adam McDaniel
- Colby Smith

## **_I_**. Introduction

Our project is a resume or CV generator that runs on Windows, MacOS, and Debian based Linux distributions. The resume generator prompts the user for info including: information about their previous employment, their education, a description of themselves, their contact information, their skills, and style information about the generated resume.

The generated resume is in HTML format for maximum portability; HTML allows the users to easily feature their resume on a website, show their resume on any device, and print to a PDF using any browser. Additionally, HTML allows the resume generator to plug and play with a variety of stylesheets available to the user.

Our approach was to build the HTML generation before anything else, and then build the GUI around the HTML generating internal structure. This turned out to be a very good strategy, and resulted in the success of the project.

There we a few minor changes to the development plans: because the GUI library we used did not support file operations in the web, we couldn't download the generated resume to the user's computer.

Overall, our results were pretty much what we had in mind. The only goal we didn't complete was our stretch goal (web support).

## **_II_**. Customer Value

The main demographic for a resume generator program is going to be unemployeed people seeking employment. Most people in this class will be in this demographic at some point in the next two years. This demographic values
- A clean looking product
- An intuitive GUI
- Low time commitment
- Free tools

There were no changes from the original project proposal.

## **_III_**. Technology

This project will be implemented using the Rust programming language. The reason we chose Rust is because Rust allows us to support compiling native Desktop applications in addition to WASM targets.

Our program will consist of two parts: the binary and the library API. The library API will allow us to write platform independent implementations of the actual HTML generating portion of the resume that doesn't require a runtime. Then, on top of the API, the binary application will implement a GUI that _uses_ the library API. This way, the GUI and the resume generation are independent, and multiple frontend implementations are possible. If we choose to add a GUI implementation for a new platform, we can create a new binary that implements a new GUI on top of our API.

![Flow Chart](assets/flow-chart.png)

#### Risk Management

This structure has the added benefit of significantly reducing the risk of making changes to the project: changes in the binary can _only_ modify functionality of how the GUI interacts with the API, and changes in the library can _only_ modify how the API outputs HTML code. This is very good for managing risk when introducing changes to the code and the project in general.

#### Competing Technologies

1. Microsoft Word. Although this is often used for resume generation, it burdens the user with having to write the formatting themselves. It's a large time investment to write a resume in word. Our project will be focused on resumes, and will not cost money for a subscription.
2. resume.io. This is an easier tool to use, but it requires creating an account, verifying email, and logging in to use the tool fully. It's doesn't grab peoples attention with its functionality as much as ours potentially could. Additionally, the website freezes when entering `a@a.a` as the email. Not exactly robust. Ours will aim to be quick to use, sturdy, and not require any signups.
3. resumebuilder.indeed.com. This tool is better than the previous tool, but doesn't offer any in-depth resume options; as of now, the website only has sections for work experience and skills for the majority of its templates. Ours will offer more sections that will be available for all templates, and will have more optional features such as adding ratings for proficiency in given skills, more detailed contact information, etc.

Most other competing technologies cost money, have incomplete functionality, or are not easy to use. These all deter people from using their product.

## **_IV_**. Team

Our team is not new to writing projects like these, but one of our members has not used Rust extensively.

Our members' roles are as follows:

- Adam McDaniel: responsible for backend of the project
- Colby Smith: responsible for frontend of the project

## **_V_**. Project Management
Project meetings and management will be handled on Discord on a weekly basis. Here is a tentative schedule that will surely see changes over the course of the semester. We will follow the general guidelines from these dates and project goals.
| Date              | Activity                                        | Goals                                                                    |
|-------------------|-------------------------------------------------|--------------------------------------------------------------------------|
| Thursday, Feb. 18 | Begin work on project.                          | Have a single basic template resume to feed user input into              |
| Friday, Feb. 26   | Second sprint                                   | Add a second template and allow the user to pick between the two         |
| Thursday, Mar. 4  | Submit iteration 1 status report                |                                                                          |
| Friday, Mar. 14   | Third sprint                                    | Upgrade from a commandline based project to having a basic GUI           |
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

